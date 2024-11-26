use crate::db::get_database;
use crate::db_models::{matches::Match, user::User, votes::Vote};
use crate::services::email::send_email;
use chrono::{Duration, Utc};
use mongodb::bson::doc;
use rocket::futures::TryStreamExt;

pub async fn live_battle_cron() {
    let judge_weight = 0.67;
    let pop_weight = 0.33;
    let database = get_database().await;
    // Get the database  collection
    let match_collection = database.collection::<Match>("match");
    let vote_collection = database.collection::<Vote>("vote");
    let user_collection = database.collection::<User>("user");
    // Get the match data that ended voting period.
    let _ = match match_collection
        .find(
            doc! {
                "closed" : {"$ne" : true},
                "live_battle" : true,
                "live_admin_registration_id" : {"$ne" : "".to_string()}
            },
            None,
        )
        .await
    {
        Ok(res) => {
            // If data exist, try collect to make it Vec
            let filter_match: Vec<Match> = match res.try_collect::<Vec<Match>>().await {
                Ok(result) => {
                    if result.len() == 0 {
                        return;
                    }
                    // Filter voting period ended
                    result
                        .into_iter()
                        .filter(move |matches| {
                            matches.last_updated_timestamp
                                <= (Utc::now() - Duration::minutes(5 as i64))
                                    .timestamp()
                        })
                        .collect()
                }
                Err(e) => {
                    log::error!("In cron for voting to try_collect, Err : {}", e.to_string());
                    return;
                }
            };
            if filter_match.len() == 0 {
                return;
            }
            // for statement for calculation and set closed
            for data in filter_match {
                // Get the voting information using match_id
                let vote_info = match vote_collection
                    .find(
                        doc! { "match_id" : data.match_id.to_string(), "vote_type" : {"$ne" : 0 } },
                        None,
                    )
                    .await
                {
                    Ok(res) => {
                        // Try collect for voting information
                        match res.try_collect::<Vec<Vote>>().await {
                            Ok(result) => {
                                if result.len() == 0 {
                                    continue;
                                }
                                // return result
                                result
                            }
                            Err(e) => {
                                log::error!(
                                    "When try collection for voting, Error : {}",
                                    e.to_string()
                                );
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "In voting cron to get the vote information, Error : {}",
                            e.to_string()
                        );
                        return;
                    }
                };

                if vote_info.len() == 0 {
                    continue;
                }
                let mut a_judges_score: i32 = 0;
                let mut a_official_score: i32 = 0;
                let mut b_judges_score: i32 = 0;
                let mut b_official_score: i32 = 0;
                // for statement for sum all voting value.
                for vote in vote_info {
                    if vote.vote_type == 1 {
                        a_official_score += vote.a_camp_votes;
                        b_official_score += vote.b_camp_votes;
                    } else if vote.vote_type == 2 {
                        a_judges_score += vote.a_camp_votes;
                        b_judges_score += vote.b_camp_votes;
                    }
                }
                // Get the a_vote avg percent
                let a_avg_vote = if a_official_score == 0 {
                    0 as f64
                } else {
                    ((a_official_score as f64)
                        / ((a_official_score as f64) + (b_official_score as f64)))
                        * (100 as f64)
                };
                // Get the b_vote avg percent
                let b_avg_vote = if b_official_score == 0 {
                    0 as f64
                } else {
                    ((b_official_score as f64)
                        / ((a_official_score as f64) + (b_official_score as f64)))
                        * (100 as f64)
                };
                // Get the a_judge avg percent
                let a_avg_judge = if a_judges_score == 0 {
                    0 as f64
                } else {
                    ((a_judges_score as f64) / ((a_judges_score as f64) + (b_judges_score as f64)))
                        * (100 as f64)
                };
                // Get the b_judge avg percent
                let b_avg_judge = if b_judges_score == 0 {
                    0 as f64
                } else {
                    ((b_judges_score as f64) / ((a_judges_score as f64) + (b_judges_score as f64)))
                        * (100 as f64)
                };
                // Calculate final score for a_camp
                let a_final_score = if b_avg_judge + a_avg_judge == 0.0 {
                    a_avg_vote
                } else if a_avg_vote + b_avg_vote == 0.0 {
                    a_avg_judge
                } else {
                    (a_avg_vote * pop_weight + a_avg_judge * judge_weight).round()
                };
                // Calculate final score for b_camp
                let b_final_score = if b_avg_judge + a_avg_judge == 0.0 {
                    b_avg_vote
                } else if a_avg_vote + b_avg_vote == 0.0 {
                    b_avg_judge
                } else {
                    (b_avg_vote * pop_weight + b_avg_judge * judge_weight).round()
                };
                // Get winner username
                let winner_username = if a_final_score >= b_final_score {
                    data.a_camp_username.to_string()
                } else {
                    data.b_camp_username.to_string()
                };
                // Get the loser username
                let loser_username = if a_final_score >= b_final_score {
                    data.b_camp_username.to_string()
                } else {
                    data.a_camp_username.to_string()
                };
                // Update match collection.
                match match_collection
                    .update_one(
                        doc! { "match_id" : data.match_id.to_string() },
                        doc! {
                            "$set" : {
                                "a_camp_vote_count" : a_avg_vote as i32,
                                "a_camp_judge_vote_count" : a_avg_judge as i32,
                                "a_camp_final_vote_count" : a_final_score  as i32,
                                "b_camp_vote_count" : b_avg_vote  as i32,
                                "b_camp_judge_vote_count" : b_avg_judge  as i32,
                                "b_camp_final_vote_count" : b_final_score as i32,
                                "closed" : true,
                            }
                        },
                        None,
                    )
                    .await
                {
                    Ok(_) => {
                        // Update Winner User collection
                        let winner_information = match
                            user_collection.find_one(
                                doc! { "username" : winner_username.to_string() },
                                None
                            ).await
                        {
                            Ok(None) => {
                                continue;
                            }
                            Ok(Some(res)) => { res }
                            Err(_) => {
                                continue;
                            }
                        };
                        // Update Loser User collection
                        let loser_information = match
                            user_collection.find_one(
                                doc! { "username" : loser_username.to_string() },
                                None
                            ).await
                        {
                            Ok(None) => {
                                continue;
                            }
                            Ok(Some(res)) => { res }
                            Err(_) => {
                                continue;
                            }
                        };

                        // Send email
                        // send to winner
                        let score_string = format!("{} to {}", a_final_score, b_final_score);
                        let _ = send_email(
                            winner_information.email.to_string(),
                            format!(
                                "You WON against {} {} in live battle",
                                loser_information.username.to_string(),
                                score_string
                            ),
                            "<p>Congratulations! Great Job in live battle.</p>".to_string(),
                        );
                        // send to loser
                        let _ = send_email(
                            loser_information.username.to_string(),
                            format!(
                                "You LOST against {} {} in live battle",
                                loser_information.username.to_string(),
                                score_string
                            ),
                            "<p>Everyone loses some battles. Keep practicing though. Your participation is greatly appreciated.</p>".to_string()
                        );
                    }
                    Err(e) => {
                        log::error!(
                            "In voting cron, trying to update match collection, Error : {}",
                            e.to_string()
                        );
                        continue;
                    }
                }
            }
        }
        Err(e) => {
            log::error!("In cron for voting, Err : {}", e.to_string());
            return;
        } 
    };
}

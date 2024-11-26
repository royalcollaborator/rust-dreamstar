use crate::db_models::matches::Match;
use crate::db_models::user::User;
use crate::db_models::votes::Vote;
use crate::middleware::recaptcha_verify::Recaptcha;
use crate::models::error_response::ErrorRes;
use crate::models::user_card::UserSelect;
use crate::utils::util::{decode_jwt, DecodeJwtHelper};

use chrono::Utc;
use log;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, options::FindOptions, Database};
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetShowListResModel {
    data: Vec<SelectedMatch>,
    max_page: i32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BattleStatus {
    BattleClosed,
    Voting,
    WaitingResponse,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WinnerStatus {
    NotDetermine,
    WinnerA,
    WinnerB,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VotingType {
    Not,
    Official,
    Unofficial,
    Judge,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetShowListReqModel {
    pub search: String,
    pub count: i32,
    pub pagination: i32,
    pub show_take_backs: bool,
    pub show_incomplete: bool,
    pub show_close: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectedMatch {
    pub match_id: String,
    pub a_camp_username: String,
    pub b_camp_username: String,
    pub winner: WinnerStatus,
    pub status: BattleStatus,
    pub a_camp_img_src: String,
    pub b_camp_img_src: String,
    pub a_camp_vid_src: String,
    pub b_camp_vid_src: String,
    pub judges: Vec<String>,
    pub rules: String,
    pub a_camp_timestamp: i64,
    pub b_camp_timestamp: i64,
    pub b_reply : String
}

#[post("/show-battle-list", format = "json", data = "<req_data>")]
pub async fn show_battle_list(
    database: &State<Database>,
    req_data: Option<Json<GetShowListReqModel>>,
) -> Result<(Status, Json<GetShowListResModel>), (Status, Json<ErrorRes>)> {
    // Check request data
    // if it is invalid, return bad request status
    let req = match req_data {
        Some(val) => val,
        None => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Bad Request"),
                }),
            ));
        }
    };
    let req_parse = req.into_inner();
    let match_collection = database.collection::<Match>("match");
    // Basic filter
    let mut conditions = doc! { "a_camp_verified": true, "live_battle" : false };
    // Add filter condition base on request
    if !req_parse.search.is_empty() {
        conditions.extend(
            doc! {
            "$or" : [
                {"a_camp_username" : {"$regex": format!(".*{}.*", req_parse.search.to_string()), "$options": "i" }},
                {"b_camp_username" : {"$regex": format!(".*{}.*", req_parse.search.to_string()), "$options": "i" }}
            ]
        }
        );
    }
    // Add filter condition base on request
    if !req_parse.show_take_backs {
        conditions.extend(doc! {
            "a_camp_withdrawn": {"$ne": true},
            "b_camp_withdrawn": {"$ne": true},
        });
    }
    // Add filter condition base on request
    if !req_parse.show_incomplete {
        conditions.extend(doc! {
            "response_timestamp": {"$ne": 0},
        });
    }
    // Add filter condition base on request
    if !req_parse.show_close {
        conditions.extend(doc! {
            "closed": false,
        });
    }
    // log::debug!(format!("{}",conditions.to_string()));
    log::debug!("{}", conditions.to_string());
    let count = match match_collection
        .count_documents(conditions.clone(), None)
        .await
    {
        Ok(i) => i,
        Err(e) => {
            log::error!(
                "When get the count in match collection, Error : {}",
                e.to_string()
            );
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error".to_string(),
                }),
            ));
        }
    };
    // Filter options that used limit and skip
    let option = FindOptions::builder()
        .skip(Some((req_parse.count * (req_parse.pagination - 1)) as u64))
        .limit(Some(req_parse.count as i64))
        .build();
    // Find all user that match with my filter and option
    let all_list = match match_collection.find(conditions.clone(), option).await {
        Ok(res) => {
            // Get the all data as Vec<Match>
            match res.try_collect::<Vec<Match>>().await {
                // if Success,
                Ok(result) => result,
                // Return Error to front-end when get the error
                Err(e) => {
                    log::error!(
                        "When try collection for match list, Error : {}",
                        e.to_string()
                    );
                    return Err((
                        Status::InternalServerError,
                        Json(ErrorRes {
                            cause: "Server Error".to_string(),
                        }),
                    ));
                }
            }
        }
        // Return Error to front-end when get the error
        Err(e) => {
            log::error!(
                "When get match list with filter and option, Error : {}",
                e.to_string()
            );
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error ".to_string(),
                }),
            ));
        }
    };
    // Make Vec to send front-end
    let result: Vec<SelectedMatch> = all_list
        .into_iter()
        .map(move |res| {
            // Todo
            // Here, I have to check who is winner. I will complete it our next milestone.
            let winner = if res.closed && res.a_camp_verified && res.b_camp_verified {
                if res.a_camp_final_vote_count >= res.b_camp_final_vote_count {
                    WinnerStatus::WinnerA
                } else {
                    WinnerStatus::WinnerB
                }
            } else {
                WinnerStatus::NotDetermine
            };
            // Status  for battle is finish or b user didn't response and so on.
            let status = if res.a_camp_verified && !res.b_camp_verified {
                BattleStatus::WaitingResponse
            } else if res.a_camp_verified && res.b_camp_verified && !res.closed {
                BattleStatus::Voting
            } else if res.a_camp_verified && res.b_camp_verified && res.closed {
                BattleStatus::BattleClosed
            } else {
                BattleStatus::WaitingResponse
            };

            SelectedMatch {
                match_id: res.match_id,
                a_camp_username: res.a_camp_username.to_string(),
                b_camp_username: res.b_camp_username.to_string(),
                winner: winner,
                status: status,
                a_camp_img_src: res.a_camp_img_src,
                b_camp_img_src: res.b_camp_img_src,
                a_camp_vid_src: res.a_camp_vid_src,
                b_camp_vid_src: res.b_camp_vid_src,
                judges: res.judges,
                rules: res.rules,
                a_camp_timestamp: res.call_out_timestamp,
                b_camp_timestamp: res.response_timestamp,
                b_reply : res.responder_reply
            }
        })
        .collect();

    // Calculate max page
    let pages = (count as i32) / req_parse.count;
    let rest = (count as i32) % req_parse.count;
    let all_pages = if rest == 0 { pages } else { pages + 1 };
    // Send response to front-end
    Ok((
        Status::Ok,
        Json(GetShowListResModel {
            data: result,
            max_page: all_pages,
        }),
    ))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShowSelectedBattleReqModel {
    pub match_id: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShowSelectedBattleResModel {
    pub a_camp_information: UserSelect,
    pub b_camp_information: UserSelect,
    pub username_check: bool,
    pub battle_information: SelectedMatch,
    pub voting_type: VotingType,
}

#[post("/show-select-battle", format = "json", data = "<req_data>")]
pub async fn show_select_battle(
    database: &State<Database>,
    recaptcha: Recaptcha,
    req_data: Option<Json<ShowSelectedBattleReqModel>>,
) -> Result<(Status, Json<ShowSelectedBattleResModel>), (Status, Json<ErrorRes>)> {
    // Check recaptcha
    if !recaptcha.recaptcha_result {
        return Err((
            Status::Forbidden,
            Json(ErrorRes {
                cause: format!("ReCAPTCHA Error"),
            }),
        ));
    }
    // Check request data
    // if it is invalid, return bad request status
    let req_parse = match req_data {
        Some(val) => val.into_inner(),
        None => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Bad Request"),
                }),
            ));
        }
    };

    let match_collection = database.collection::<Match>("match");
    let user_collection = database.collection::<User>("user");
    let vote_collection = database.collection::<Vote>("vote");
    // Check user token validation, Get the user id from token
    let auth_user_id = if !req_parse.token.is_empty() {
        let vec_header = req_parse.token.split_whitespace().collect::<Vec<_>>();
        // Check here
        match decode_jwt(vec_header[1].to_string()) {
            DecodeJwtHelper::Ok(token_data) => {
                // Check expire time
                let expire_time = token_data.claims.exp;
                let now_time = Utc::now().timestamp() as usize;
                if expire_time > now_time {
                    token_data.claims.user_id.to_string()
                } else {
                    "".to_string()
                }
            }
            DecodeJwtHelper::Err => "".to_string(),
        }
    } else {
        "".to_string()
    };
    // Get the this username
    let auth_user_name = if auth_user_id.is_empty() {
        "".to_string()
    } else {
        let user_obj_id = ObjectId::parse_str(auth_user_id.to_string()).unwrap();
        match user_collection
            .find_one(doc! { "_id" : user_obj_id }, None)
            .await
        {
            Ok(None) => "".to_string(),
            Ok(Some(sel_user)) => sel_user.username,
            Err(_) => "".to_string(),
        }
    };
    // Get match information using request 's match id
    let battle_information = match match_collection
        .find_one(
            doc! { "match_id" : req_parse.match_id.to_string(), "a_camp_verified" : true, "live_battle" : false },
            None,
        )
        .await
    {
        Ok(None) => {
            // return bad request if there doesn't exist match
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "Bad request".to_string(),
                }),
            ));
        }
        Ok(Some(res)) => {
            let winner = if res.closed && res.a_camp_verified && res.b_camp_verified {
                if res.a_camp_final_vote_count >= res.b_camp_final_vote_count {
                    WinnerStatus::WinnerA
                } else {
                    WinnerStatus::WinnerB
                }
            } else {
                WinnerStatus::NotDetermine
            };
            // Status  for battle is finish or b user didn't response and so on.
            let status = if res.a_camp_verified && !res.b_camp_verified {
                BattleStatus::WaitingResponse
            } else if res.a_camp_verified && res.b_camp_verified && !res.closed {
                BattleStatus::Voting
            } else if res.a_camp_verified && res.b_camp_verified && res.closed {
                BattleStatus::BattleClosed
            } else {
                BattleStatus::WaitingResponse
            };
            // pick some essential filed
            SelectedMatch {
                match_id: res.match_id,
                a_camp_username: res.a_camp_username.to_string(),
                b_camp_username: res.b_camp_username.to_string(),
                winner: winner,
                status: status,
                a_camp_img_src: res.a_camp_img_src,
                b_camp_img_src: res.b_camp_img_src,
                a_camp_vid_src: res.a_camp_vid_src,
                b_camp_vid_src: res.b_camp_vid_src,
                judges: res.judges,
                rules: res.rules,
                a_camp_timestamp : res.call_out_timestamp,
                b_camp_timestamp : res.response_timestamp,
                b_reply : res.responder_reply
            }
        }
        Err(e) => {
            // Server Error
            log::error!(
                "When get the match information using match_id, Error : {}",
                e.to_string()
            );
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error".to_string(),
                }),
            ));
        }
    };

    // Get a_camp user information.
    let a_information = match user_collection
        .find_one(
            doc! { "username" : battle_information.a_camp_username.to_string(), "battler" : 1 },
            None,
        )
        .await
    {
        Ok(None) => {
            // Error there doesn't exist match username.
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "Bad request".to_string(),
                }),
            ));
        }
        Ok(Some(user)) => {
            // If success, get the necessary filed
            UserSelect {
                _id: user._id.to_string(),
                username: user.username,
                battler_score: user.battler_score,
                instagram_name: user.instagram_name,
                instagram_id: user.instagram_id,
                matches_won: user.matches_won.len() as i32,
                matches_lost: user.matches_lost.len() as i32,
                matches_withdrawn: user.matches_withdrawn.len() as i32,
                callout: user.work_a.len() as i32,
                response: user.work_b.len() as i32,
                one_hundred_badge: user.one_hundred_badge,
                first_tourney_badge: user.first_tourney_badge,
            }
        }
        Err(e) => {
            log::error!(
                "when the a_camp_user information, Error : {}",
                e.to_string()
            );
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error ".to_string(),
                }),
            ));
        }
    };

    // Get a_camp user information.
    let b_information = match user_collection
        .find_one(
            doc! { "username" : battle_information.b_camp_username.to_string(), "battler" : 1 },
            None,
        )
        .await
    {
        Ok(None) => {
            // Error there doesn't exist match username.
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "Bad request".to_string(),
                }),
            ));
        }
        Ok(Some(user)) => {
            // If success, get the necessary filed
            UserSelect {
                _id: user._id.to_string(),
                username: user.username,
                battler_score: user.battler_score,
                instagram_name: user.instagram_name,
                instagram_id: user.instagram_id,
                matches_won: user.matches_won.len() as i32,
                matches_lost: user.matches_lost.len() as i32,
                matches_withdrawn: user.matches_withdrawn.len() as i32,
                callout: user.work_a.len() as i32,
                response: user.work_b.len() as i32,
                one_hundred_badge: user.one_hundred_badge,
                first_tourney_badge: user.first_tourney_badge,
            }
        }
        Err(e) => {
            log::error!(
                "when the a_camp_user information, Error : {}",
                e.to_string()
            );
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error ".to_string(),
                }),
            ));
        }
    };
    // Determine voting available for this user.
    let voting_available = if auth_user_name.is_empty() {
        // User is not logged in, can't do voting
        VotingType::Not
    } else {
        if battle_information.a_camp_username.to_string() == auth_user_name.to_string()
            || battle_information.b_camp_username.to_string() == auth_user_name.to_string()
        {
            // If user is a_camp or b_camp, can't do voting
            VotingType::Not
        } else {
            // Check this user already did voting.
            match
                vote_collection.find_one(
                    doc! { "match_id" : req_parse.match_id.to_string(), "voter_name" : auth_user_name.to_string() },
                    None
                ).await
            {
                Ok(None) => {
                    // User didn't do voting, determined voting type
                    match battle_information.status {
                        BattleStatus::BattleClosed => {
                            // Voting period ended, only can do Official voting.
                            VotingType::Unofficial
                        }
                        BattleStatus::Voting => {
                            if battle_information.judges.contains(&auth_user_name.to_string()) {
                                // Voting period is available and user is one of judge, can do join voting as judge.
                                VotingType::Judge
                            } else {
                                // Voting period is available and user is not judge, can do official voting.
                                VotingType::Official
                            }
                        }
                        BattleStatus::WaitingResponse => {
                            // If b_camp is not response yet, can't do voting.
                            VotingType::Not
                        }
                    }
                }
                Ok(Some(_)) => {
                    // If user already did join voting, can't do
                    VotingType::Not
                }
                Err(e) => {
                    log::error!("When get the vote information, Error : {}", e.to_string());
                    return Err((
                        Status::InternalServerError,
                        Json(ErrorRes { cause: "Server Error ".to_string() }),
                    ));
                }
            }
        }
    };

    // Check if this user is b_camp
    let check_username = if b_information._id == auth_user_id
        && battle_information.b_camp_vid_src.is_empty()
        && battle_information.b_camp_img_src.is_empty()
    {
        true
    } else {
        false
    };

    Ok((
        Status::Ok,
        Json(ShowSelectedBattleResModel {
            a_camp_information: a_information,
            b_camp_information: b_information,
            username_check: check_username,
            battle_information: battle_information,
            voting_type: voting_available,
        }),
    ))
}

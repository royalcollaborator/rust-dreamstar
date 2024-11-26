use super::battle_main::{BattleStatus, SelectedMatch, VotingType, WinnerStatus};
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
use mongodb::{bson::doc, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShowLiveBattleReqModel {
    pub code: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShowLiveBattleResModel {
    pub a_camp_information: UserSelect,
    pub b_camp_information: UserSelect,
    pub username_check: bool,
    pub battle_information: SelectedMatch,
    pub voting_type: VotingType,
}

#[post("/show-live-battle", format = "json", data = "<req_data>")]
pub async fn live_battle_show(
    database: &State<Database>,
    recaptcha: Recaptcha,
    req_data: Option<Json<ShowLiveBattleReqModel>>,
) -> Result<(Status, Json<ShowLiveBattleResModel>), (Status, Json<ErrorRes>)> {
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
            doc! { "live_admin_registration_id" : req_parse.code.to_string() , "live_battle" : true},
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
            doc! { "username" : battle_information.a_camp_username.to_string(), "battler" : 1,  },
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
                    doc! { "match_id" : battle_information.match_id.to_string(), "voter_name" : auth_user_name.to_string() },
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
        Json(ShowLiveBattleResModel {
            a_camp_information: a_information,
            b_camp_information: b_information,
            username_check: check_username,
            battle_information: battle_information,
            voting_type: voting_available,
        }),
    ))
}

use crate::db_models::matches::Match;
use crate::db_models::user::User;
use crate::middleware::recaptcha_verify::Recaptcha;
use crate::middleware::verify_token::AuthorizedUser;
use crate::models::error_response::ErrorRes;
use crate::utils::util::live_battle_code;

use chrono::Utc;
use log;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiveBattleSetupReq {
    pub a_1: String,
    pub a_2: String,
    pub a_3: String,
    pub a_4: String,
    pub a_5: String,
    pub statement: String,
    pub a_camp: String,
    pub b_camp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiveBattleSetupRes {
    pub result: Vec<String>,
    pub id : String
}

/**
 * Router for creating live battle
 */
#[post("/live-battle-setup", format = "json", data = "<req_data>")]
pub async fn live_battle_setup(
    database: &State<Database>,
    recaptcha: Recaptcha,
    auth_user: AuthorizedUser,
    req_data: Option<Json<LiveBattleSetupReq>>,
) -> Result<(Status, Json<LiveBattleSetupRes>), (Status, Json<ErrorRes>)> {
    let now_time = Utc::now().timestamp();
    let match_id = Uuid::new_v4().to_string();
    let short_id = Uuid::new_v4().to_string();
    let live_battle_id = live_battle_code().to_string();
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
    // get the user collection
    let user_collection = database.collection::<User>("user");
    // Get the match collection
    let match_collection = database.collection::<Match>("match");
    // check auth user
    let auth_user_id = auth_user.user_id;
    // get b camp username
    let b_username = match user_collection
        .find_one(
            doc! { "username" : req_parse.b_camp.to_string(), "battler" : 1 },
            None,
        )
        .await
    {
        Ok(None) => "".to_string(),
        Ok(Some(sel_user)) => sel_user.username,
        Err(_) => "".to_string(),
    };

    // get b camp username
    let a_username = match user_collection
        .find_one(
            doc! { "username" : req_parse.a_camp.to_string(), "battler" : 1 },
            None,
        )
        .await
    {
        Ok(None) => "".to_string(),
        Ok(Some(sel_user)) => sel_user.username,
        Err(_) => "".to_string(),
    };
    // If b username doesn't exist, return Error
    if b_username.is_empty() {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "b_camp is not battler".to_string(),
            }),
        ));
    }
    // If a username doesn't exist, return Error
    if a_username.is_empty() {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "You are not battler".to_string(),
            }),
        ));
    }
    // Get the user information
    let user_information = match ObjectId::parse_str(auth_user_id) {
        Ok(id) => {
            match user_collection
                .find_one(
                    doc! {"_id" : id, "$or" : [
                        {"battler" : 1},
                        {"judge" : 1}
                    ]},
                    None,
                )
                .await
            {
                Ok(None) => {
                    return Err((
                        Status::BadRequest,
                        Json(ErrorRes {
                            cause: format!("You are not permitted"),
                        }),
                    ));
                }
                Ok(Some(val)) => val,
                Err(e) => {
                    log::error!(
                        "When get the user information for live battle setup, Error : {}",
                        e.to_string()
                    );
                    return Err((
                        Status::InternalServerError,
                        Json(ErrorRes {
                            cause: format!("Server Error"),
                        }),
                    ));
                }
            }
        }
        Err(_) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Invalid Token"),
                }),
            ));
        }
    };
    // Make the judge array
    let mut arr: Vec<String> = Vec::new();
    arr.push(req_parse.a_1.to_string());
    arr.push(req_parse.a_2.to_string());
    arr.push(req_parse.a_3.to_string());
    arr.push(req_parse.a_4.to_string());
    arr.push(req_parse.a_5.to_string());
    let mut results: Vec<String> = Vec::new();
    // Check judge doesn't include a_camp or b_camp
    for judge_check in arr.clone() {
        if judge_check.to_string() == user_information.username.to_string()
            || judge_check.to_string() == b_username.to_string()
        {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "Judge list include a_camp or b_camp username.".to_string(),
                }),
            ));
        }
    }
    // Check which username doesn't not exist
    for val in arr.clone() {
        if val.is_empty() {
            continue;
        } else {
            match user_collection
                .find_one(doc! { "username" : val.to_string(), "judge" : 1 }, None)
                .await
            {
                Ok(None) => {
                    results.push(val.to_string());
                }
                Ok(Some(_)) => {}
                Err(e) => {
                    log::error!(
                        "When  find data in user collection, Error : {}",
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
        }
    }
    if results.clone().len() == 0 {
        // To check, already triggered same callout
        // this is condition
        let filter = doc! {
            "$or": [
                {
                    "$and": [
                        { "a_camp_username": a_username.to_string() },
                        { "b_camp_username": b_username.to_string() },
                        { "closed": false },
                    ]
                },
                {
                    "$and": [
                        { "a_camp_username": b_username.to_string() },
                        { "b_camp_username": a_username.to_string() },
                        { "closed": false },
                    ]
                },
            ]
        };

        match match_collection.find_one(filter, None).await {
            Ok(Some(_)) => {
                return Err((
                    Status::BadRequest,
                    Json(ErrorRes {
                        cause: "You 've already created live battle for these users".to_string(),
                    }),
                ));
            }
            Ok(None) => {}
            Err(e) => {
                log::error!(
                    "When check live battle is unique, Error : {}",
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
        // new live battle collection
        let create_match = Match {
            match_id: match_id.to_string(),
            short_id: short_id.to_string(),
            live_battle: true,
            last_updated_timestamp: now_time,
            call_out_timestamp: now_time,
            voting_duration: 24,
            a_camp_username: a_username.to_string(),
            a_camp_video_id: String::from(""),
            a_camp_video_width: Default::default(),
            a_camp_video_height: Default::default(),
            a_camp_video_type: String::from(""),
            a_camp_vote_count: 0,
            a_camp_judge_vote_count: 0,
            a_camp_final_vote_count: 0,
            a_camp_withdrawn: false,
            a_camp_verified: true,
            a_camp_img_src: String::from(""), // need fix
            a_camp_vid_src: String::from(""), //need fix.
            a_camp_instagram: Default::default(),
            rules: req_parse.statement.to_string(),
            b_camp_username: b_username.to_string(),
            b_camp_vote_count: 0,
            b_camp_judge_vote_count: 0,
            b_camp_final_vote_count: 0,
            b_camp_withdrawn: false,
            b_camp_verified: true,
            b_camp_img_src: Default::default(),
            b_camp_vid_src: Default::default(),
            b_camp_instagram: Default::default(),
            b_camp_video_id: Default::default(),
            judges: arr,
            reporting_sequence: Default::default(),
            closed: false,
            live_admin_name: user_information.username.to_string(),
            live_admin_registration_id: live_battle_id.to_string(),
            response_timestamp: now_time,
            responder_reply: Default::default(),
            b_camp_video_height: Default::default(),
            b_camp_video_type: Default::default(),
            b_camp_video_width: Default::default(),
        };
        // To insert live battle
        match match_collection
            .insert_one(create_match.clone(), None)
            .await
        {
            // if success
            Ok(_) => Ok((
                Status::Ok,
                Json(LiveBattleSetupRes {
                    result: results.clone(),
                    id : live_battle_id.to_string()
                }),
            )),
            // Error
            Err(e) => {
                log::error!("When create new live battle, Error : {}", e.to_string());
                return Err((
                    Status::InternalServerError,
                    Json(ErrorRes {
                        cause: "Server Error".to_string(),
                    }),
                ));
            }
        }
    } else {
        // if one of judges is invalid
        let mut error_str = String::from("");
        for val in results.clone() {
            error_str = error_str + &val;
        }
        Ok((
            Status::Ok,
            Json(LiveBattleSetupRes {
                result: results.clone(),
                id : String::from("")
            }),
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiveBattleCheckReq {
    code: String,
}

/**
 * Router for checking live battle code
 */
#[post("/live-battle-code-check", format = "json", data = "<req_data>")]
pub async fn live_battle_code_check(
    database: &State<Database>,
    recaptcha: Recaptcha,
    req_data: Option<Json<LiveBattleCheckReq>>,
) -> Result<Status, (Status, Json<ErrorRes>)> {
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
    // get the match collection
    let match_collection = database.collection::<Match>("match");
    // Check code
    let code_check = match match_collection
        .find_one(doc! {"live_admin_registration_id" : req_parse.code}, None)
        .await
    {
        Ok(None) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Code is not correct"),
                }),
            ));
        }
        Ok(Some(val)) => val,
        Err(e) => {
            log::error!("When check live battle code, Error : {}", e.to_string());
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error".to_string(),
                }),
            ));
        }
    };
    Ok(Status::Ok)
}

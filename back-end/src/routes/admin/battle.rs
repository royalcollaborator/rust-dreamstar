use crate::db_models::matches::Match;
use crate::db_models::user::User;
use crate::middleware::verify_token::AuthorizedAdmin;
use crate::models::error_response::ErrorRes;

use log;
use mongodb::{bson::doc, Database};
use rocket::serde::json::Json;
use rocket::State;
use rocket::{futures::TryStreamExt, http::Status};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSetUserReq {
    pub match_id: String,
}

/**
 * Admin Router for callout setup
 */
#[post("/callout-setup", format = "json", data = "<req_data>")]
pub async fn callout_setup(
    database: &State<Database>,
    req_data: Option<Json<GetSetUserReq>>,
    admin: AuthorizedAdmin,
) -> Result<Status, (Status, Json<ErrorRes>)> {
    // Request parse
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
    // Get match collection from database
    let match_collection = database.collection::<Match>("match");
    // Update match collection (a_camp verify)
    match match_collection
        .find_one_and_update(
            doc! {"match_id" : req_parse.match_id.to_string()},
            doc! {"$set" : {
                "a_camp_verified" : true
            }},
            None,
        )
        .await
    {
        // if battle doesn't exist, return error
        Ok(None) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Battle doesn't exist"),
                }),
            ));
        }
        // if exist, return success
        Ok(Some(_)) => Ok(Status::Ok),
        Err(e) => {
            log::error!(
                "When admin setup a_camp callout verify, Error :  {}",
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

/**
 * Admin Router for reply setup
 */
#[post("/reply-setup", format = "json", data = "<req_data>")]
pub async fn reply_setup(
    database: &State<Database>,
    req_data: Option<Json<GetSetUserReq>>,
    admin: AuthorizedAdmin,
) -> Result<Status, (Status, Json<ErrorRes>)> {
    // Request parse
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
    // get match collection
    let match_collection = database.collection::<Match>("match");
    // Get user collection
    let user_collection = database.collection::<User>("user");
    // Get match information with update
    let match_info = match match_collection
        .find_one_and_update(
            doc! {"match_id" : req_parse.match_id.to_string()},
            doc! {"$set" : {
                "b_camp_verified" : true
            }},
            None,
        )
        .await
    {
        Ok(None) => {
            // if battle doesn't exist, return error
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Battle doesn't exist"),
                }),
            ));
        }
        // success in update and get, battle return
        Ok(Some(battle)) => battle,
        Err(e) => {
            log::error!(
                "When admin setup b_camp callout verify, Error :  {}",
                e.to_string()
            );
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: format!("Server Error"),
                }),
            ));
        }
    };
    // a_camp  work update
    let a_update = doc! {
        "$addToSet": { "work_a": req_parse.match_id.to_string() }
    };
    // b_camp  work update
    let b_update = doc! {
        "$addToSet": { "work_b": req_parse.match_id.to_string() }
    };
    // update number of a_camp callout in user
    let _ = match user_collection
        .update_one(
            doc! {
                "username" : match_info.a_camp_username
            },
            a_update,
            None,
        )
        .await
    {
        Ok(_) => true,
        Err(e) => {
            log::error!(
                "When admin setup a_callout work in user collection, Error :  {}",
                e.to_string()
            );
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: format!("Server Error"),
                }),
            ));
        }
    };
    // update number of b_camp reply in user collection
    let _ = match user_collection
        .update_one(
            doc! {
                "username" : match_info.b_camp_username
            },
            b_update,
            None,
        )
        .await
    {
        Ok(_) => true,
        Err(e) => {
            log::error!(
                "When admin setup a_callout work in user collection, Error :  {}",
                e.to_string()
            );
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: format!("Server Error"),
                }),
            ));
        }
    };
    Ok(Status::Ok)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BattleVerifyType {
    ACamp,
    BCamp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminBattleListRes {
    pub data: Vec<AdminSelectMatch>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminSelectMatch {
    pub match_id: String,
    pub a_camp_username: String,
    pub b_camp_username: String,
    pub rules: String,
    pub responder_reply: String,
    pub a_video: String,
    pub a_img: String,
    pub b_video: String,
    pub b_img: String,
    pub a_verify: bool,
    pub b_verify: bool,
    pub voting_period: i32,
}

/**
 * Admin Router to get all battles that needed verify
 */
#[get("/get-battle-list")]
pub async fn get_battle_list(
    database: &State<Database>,
    admin: AuthorizedAdmin,
) -> Result<(Status, Json<AdminBattleListRes>), (Status, Json<ErrorRes>)> {
    // get match collection
    let match_collection = database.collection::<Match>("match");
    // Make filter condition base on request
    let condition = doc! {
        "$or" : [
            {"a_camp_verified" : {"$ne" : true}, "call_out_timestamp" : {"$ne" : 0}},
            {"b_camp_verified" : {"$ne" : true}, "response_timestamp" : {"$ne" : 0}},
        ],
        "a_camp_withdrawn" : {"$ne" : true},
        "b_camp_withdrawn" : {"$ne" : true},
        "live_battle" : {"$ne" : true},
        "closed" : false
    };
    // Get all battle list base on conditions
    let battle_list: Vec<AdminSelectMatch> = match match_collection.find(condition, None).await {
        Ok(val) => match val.try_collect::<Vec<Match>>().await {
            Ok(res) => res
                .into_iter()
                .map(move |val| AdminSelectMatch {
                    match_id: val.match_id,
                    a_camp_username: val.a_camp_username,
                    b_camp_username: val.b_camp_username,
                    rules: val.rules,
                    responder_reply: val.responder_reply,
                    a_video: val.a_camp_vid_src,
                    a_img: val.a_camp_img_src,
                    b_video: val.b_camp_vid_src,
                    b_img: val.b_camp_img_src,
                    a_verify: val.a_camp_verified,
                    b_verify: val.b_camp_verified,
                    voting_period: val.voting_duration,
                })
                .collect(),
            Err(e) => {
                log::error!(
                    "When admin try collection all battle list that needed verify, Error :  {}",
                    e.to_string()
                );
                return Err((
                    Status::InternalServerError,
                    Json(ErrorRes {
                        cause: "Server Error".to_string(),
                    }),
                ));
            }
        },
        Err(e) => {
            log::error!(
                "When admin get all battle list that needed verify, Error :  {}",
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
    Ok((Status::Ok, Json(AdminBattleListRes { data: battle_list })))
}

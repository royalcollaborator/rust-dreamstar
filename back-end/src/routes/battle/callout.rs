use crate::config::{google_img_bucket, google_key_json, google_vid_bucket, smtp_email};
use crate::db_models::matches::Match;
use crate::db_models::user::User;
use crate::middleware::{recaptcha_verify::Recaptcha, verify_token::AuthorizedUser};
use crate::models::error_response::ErrorRes;
use crate::models::user_card::UserSelect;
use crate::services::email::send_email;
use crate::utils::util::{decode_jwt, DecodeJwtHelper};

use chrono::Utc;
use google_cloud_storage::client::google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::sign::{SignedURLMethod, SignedURLOptions};
use log;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, options::FindOptions, Database};
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserListReqModel {
    pub search: String,
    pub count: i32,
    pub pagination: i32,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserListResModel {
    pub data: Vec<UserSelect>,
    pub max_pages: i32,
    pub battler_check: bool,
}

/**
 * Router for get user list for search
 */
#[post("/get-user-list", format = "json", data = "<req_data>")]
pub async fn get_user_list(
    database: &State<Database>,
    req_data: Option<Json<GetUserListReqModel>>,
) -> Result<(Status, Json<GetUserListResModel>), (Status, Json<ErrorRes>)> {
    let req_data = match req_data {
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
    let search = req_data.search.to_string();
    let list_num = req_data.count;
    let pagination = req_data.pagination;
    let user_collection = database.collection::<User>("user");
    log::info!("Accepted get user list request");

    // Check user token validation, Get the user id from token
    let auth_user_id = if !req_data.token.is_empty() {
        let vec_header = req_data.token.split_whitespace().collect::<Vec<_>>();
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

    // Check auth and battle flag.
    let battler_check = if auth_user_id.is_empty() {
        false
    } else {
        match ObjectId::parse_str(auth_user_id.to_string()) {
            Ok(id) => {
                match user_collection
                    .find_one(doc! { "_id" : id, "battler" : 1 }, None)
                    .await
                {
                    Ok(None) => false,
                    Ok(Some(_)) => true,
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    };

    // User filter condition
    // Determine the filter based on search term
    let user_filter = if search.is_empty() {
        doc! { "account_status" : "registered", "battler" : 1 }
    } else {
        doc! { "username" : {"$regex": format!(".*{}.*", search), "$options": "i" }, "account_status" : "registered", "battler" : 1 }
    };

    // Filter options that used limit and skip
    let user_option = FindOptions::builder()
        .skip(Some((list_num * (pagination - 1)) as u64))
        .limit(Some(list_num as i64))
        .build();

    // Get data from database
    match user_collection.find(user_filter.clone(), user_option).await {
        Ok(sel_data) => {
            // Get the all page count
            let all_pages = match user_collection.find(user_filter.clone(), None).await {
                Ok(res) => {
                    // If success, calculate pages
                    match res.try_collect::<Vec<User>>().await {
                        Ok(results) => {
                            let pages = (results.len() as i32) / list_num;
                            let rest = (results.len() as i32) % list_num;
                            if rest == 0 {
                                pages
                            } else {
                                pages + 1
                            }
                        }
                        Err(_) => 1,
                    }
                }
                Err(_) => 1,
            };
            // Make the Vec that send to front-end
            match sel_data.try_collect::<Vec<User>>().await {
                // If try_collect success,
                Ok(results) => {
                    if results.len() == 0 {
                        return Ok((
                            Status::Ok,
                            Json(GetUserListResModel {
                                data: Default::default(),
                                max_pages: 1,
                                battler_check: battler_check,
                            }),
                        ));
                    }
                    // Make the UserSelect Vec that sent to front-end
                    let user_arr: Vec<UserSelect> = results
                        .into_iter()
                        .map(|user| {
                            // return UserSelect struct
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
                        })
                        .collect();
                    return Ok((
                        Status::Ok,
                        Json(GetUserListResModel {
                            data: user_arr,
                            max_pages: all_pages,
                            battler_check: battler_check,
                        }),
                    ));
                }
                Err(e) => {
                    log::error!("Error to data try_collect : {}", e.to_string());
                    Err((
                        Status::InternalServerError,
                        Json(ErrorRes {
                            cause: "Server Error".to_string(),
                        }),
                    ))
                }
            }
        }
        Err(e) => {
            log::error!(
                "Can't interact with database to find all users, Error : {}",
                e.to_string()
            );
            Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error".to_string(),
                }),
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSignURLReqModel {
    pub a_1: String,
    pub a_2: String,
    pub a_3: String,
    pub a_4: String,
    pub a_5: String,
    pub opponent: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSignURLResModel {
    data: Vec<String>,
    video_id: String,
    image_id: String,
    video_url: String,
    image_url: String,
}

/**
 * Check judges validation and send the video and image uuid and get the cloud sign urls
 */
#[post("/verify-get-sign-url", format = "json", data = "<req_data>")]
pub async fn get_sign_url(
    auth_user: AuthorizedUser,
    recaptcha: Recaptcha,
    database: &State<Database>,
    req_data: Option<Json<GetSignURLReqModel>>,
) -> Result<(Status, Json<GetSignURLResModel>), (Status, Json<ErrorRes>)> {
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
    // Make the judge array
    let mut arr: Vec<String> = Vec::new();
    arr.push(req_parse.a_1.to_string());
    arr.push(req_parse.a_2.to_string());
    arr.push(req_parse.a_3.to_string());
    arr.push(req_parse.a_4.to_string());
    arr.push(req_parse.a_5.to_string());
    let mut results: Vec<String> = Vec::new();
    let match_collection = database.collection::<Match>("match");
    let user_collection = database.collection::<User>("user");

    log::info!("verify judges request accept with recaptcha");

    if req_parse.opponent == auth_user.user_id {
        return Err((
            Status::Conflict,
            Json(ErrorRes {
                cause: "You are same user".to_string(),
            }),
        ));
    }

    let a_user_id = ObjectId::parse_str(auth_user.user_id).unwrap();
    let b_user_id = ObjectId::parse_str(req_parse.opponent).unwrap();

    // get a camp username
    let a_username = match user_collection
        .find_one(doc! { "_id" : a_user_id, "battler" : 1 }, None)
        .await
    {
        Ok(None) => "".to_string(),
        Ok(Some(sel_user)) => sel_user.username,
        Err(_) => "".to_string(),
    };

    // get b camp username
    let b_username = match user_collection
        .find_one(doc! { "_id" : b_user_id, "battler" : 1 }, None)
        .await
    {
        Ok(None) => "".to_string(),
        Ok(Some(sel_user)) => sel_user.username,
        Err(_) => "".to_string(),
    };
    // If a username doesn't exist, return Error
    if a_username.is_empty() {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "You are not battler".to_string(),
            }),
        ));
    }
    // If b username doesn't exist, return Error
    if b_username.is_empty() {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "Can't find your target or your target is not battler".to_string(),
            }),
        ));
    }
    // Check judge doesn't include a_camp or b_camp
    for judge_check in arr.clone() {
        if judge_check.to_string() == a_username.to_string()
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
    // Make the filter that consider same user trigger callout
    let filter = doc! {
        "$or": [
            {
                "$and": [
                    { "a_camp_username": a_username.to_string() },
                    { "b_camp_username": b_username.to_string() },
                    { "b_camp_video_id": "" },
                ]
            },
            {
                "$and": [
                    { "a_camp_username": b_username.to_string() },
                    { "b_camp_username": a_username.to_string() },
                    { "b_camp_video_id": "" },
                ]
            },
        ]
    };

    let _ = match match_collection.find_one(filter, None).await {
        Ok(None) => {}
        Ok(_) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "You 've already submitted callout request for this user.".to_string(),
                }),
            ));
        }
        Err(e) => {
            log::error!(
                "When find data in user collection, Error : {}",
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

    // Check which username doesn't not exist
    for val in arr {
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
    if results.len() == 0 {
        let vid_file_id = Uuid::new_v4().to_string();
        let image_file_id = Uuid::new_v4().to_string();
        let vid_bucket_name = google_vid_bucket();
        let img_bucket_name = google_img_bucket();
        // Get google credential from config
        match CredentialsFile::new_from_str(google_key_json().as_str()).await {
            // If exist
            Ok(cre) => {
                // Make google config  for new google client
                match ClientConfig::default().with_credentials(cre).await {
                    Ok(config_client) => {
                        let client = Client::new(config_client);
                        let vid_extension = "mp4";
                        let expires = Duration::from_secs(90 * 60); // 15 minutes
                        let vid_content_type = if vid_extension == "mp4" {
                            "video/mp4"
                        } else {
                            "video/quicktime"
                        };
                        // Create Signed URL for video
                        let vid_url = client
                            .signed_url(
                                &vid_bucket_name,
                                &format!(
                                    "videos/{}.{}",
                                    vid_file_id.to_string(),
                                    vid_extension.to_string()
                                ),
                                None,
                                None,
                                SignedURLOptions {
                                    method: SignedURLMethod::PUT,
                                    start_time: None,
                                    expires: expires,
                                    content_type: Some(vid_content_type.to_string()),
                                    ..Default::default()
                                },
                            )
                            .await
                            .expect("Failed to create signed URL for video");

                        let img_url = client
                            .signed_url(
                                &img_bucket_name,
                                &format!("images/{}.{}", image_file_id.to_string(), "jpeg"),
                                None,
                                None,
                                SignedURLOptions {
                                    method: SignedURLMethod::PUT,
                                    start_time: None,
                                    expires: expires,
                                    content_type: Some("image/jpeg".to_string()),
                                    ..Default::default()
                                },
                            )
                            .await
                            .expect("Failed to create signed URL for video");

                        Ok((
                            Status::Ok,
                            Json(GetSignURLResModel {
                                data: results,
                                video_id: vid_file_id.to_string(),
                                image_id: image_file_id.to_string(),
                                video_url: String::from(vid_url),
                                image_url: String::from(img_url),
                            }),
                        ))
                    }
                    // Error Google credential Error
                    Err(e) => {
                        log::error!("Google credential Error : {}", e.to_string());
                        Err((
                            Status::InternalServerError,
                            Json(ErrorRes {
                                cause: "Server Error".to_string(),
                            }),
                        ))
                    }
                }
            }
            Err(e) => {
                log::error!("Google credential Error : {} ", e.to_string());
                Err((
                    Status::InternalServerError,
                    Json(ErrorRes {
                        cause: "Server Error".to_string(),
                    }),
                ))
            }
        }
    } else {
        Ok((
            Status::Ok,
            Json(GetSignURLResModel {
                data: results,
                video_id: String::from(""),
                image_id: String::from(""),
                video_url: String::from(""),
                image_url: String::from(""),
            }),
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetCalloutReqModel {
    pub a_1: String,
    pub a_2: String,
    pub a_3: String,
    pub a_4: String,
    pub a_5: String,
    pub opponent_id: String,
    pub video_id: String,
    pub image_id: String,
    pub video_type: String,
    pub rules: String,
    pub voting_duration: i32,
}

/**
 *  Router for setting collout.
 */
#[post("/set-callout", format = "json", data = "<req_data>")]
pub async fn set_callout(
    auth_user: AuthorizedUser,
    recaptcha: Recaptcha,
    database: &State<Database>,
    req_data: Option<Json<SetCalloutReqModel>>,
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
    let a_user_id = ObjectId::parse_str(auth_user.user_id).unwrap();
    let b_user_id = ObjectId::parse_str(req_parse.opponent_id).unwrap();
    let user_collection = database.collection::<User>("user");
    let match_collection = database.collection::<Match>("match");
    let match_id = Uuid::new_v4().to_string();
    let short_id = Uuid::new_v4().to_string();
    let now_time = Utc::now().timestamp();

    // get a camp username
    let a_username = match user_collection
        .find_one(doc! { "_id" : a_user_id, "battler" : 1 }, None)
        .await
    {
        Ok(None) => "".to_string(),
        Ok(Some(sel_user)) => sel_user.username,
        Err(_) => "".to_string(),
    };

    // get b camp username
    let b_username = match user_collection
        .find_one(doc! { "_id" : b_user_id, "battler" : 1 }, None)
        .await
    {
        Ok(None) => "".to_string(),
        Ok(Some(sel_user)) => sel_user.username,
        Err(_) => "".to_string(),
    };

    if a_username.is_empty() {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "You are not battler".to_string(),
            }),
        ));
    }

    if b_username.is_empty() {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "Can't find your target or your target is not battler".to_string(),
            }),
        ));
    }

    // make Vec string for judges
    let mut judges_arr: Vec<String> = Vec::new();
    if !req_parse.a_1.is_empty() {
        judges_arr.push(req_parse.a_1.to_string());
    }
    if !req_parse.a_2.is_empty() {
        judges_arr.push(req_parse.a_2.to_string());
    }
    if !req_parse.a_3.is_empty() {
        judges_arr.push(req_parse.a_3.to_string());
    }
    if !req_parse.a_4.is_empty() {
        judges_arr.push(req_parse.a_4.to_string());
    }
    if !req_parse.a_5.is_empty() {
        judges_arr.push(req_parse.a_5.to_string());
    }

    let image_src = format!(
        "https://storage.googleapis.com/{}/images/{}.jpeg",
        google_img_bucket(),
        req_parse.image_id.to_string()
    );
    let video_src = format!(
        "https://storage.googleapis.com/{}/videos/{}.{}",
        google_vid_bucket(),
        req_parse.video_id.to_string(),
        req_parse.video_type.to_string()
    );

    let create_match = Match {
        match_id: match_id.to_string(),
        short_id: short_id.to_string(),
        live_battle: false,
        last_updated_timestamp: now_time,
        call_out_timestamp: now_time,
        voting_duration: req_parse.voting_duration,
        a_camp_username: a_username.to_string(),
        a_camp_video_id: req_parse.video_id.to_string(),
        a_camp_video_width: Default::default(),
        a_camp_video_height: Default::default(),
        a_camp_video_type: req_parse.video_type,
        a_camp_vote_count: 0,
        a_camp_judge_vote_count: 0,
        a_camp_final_vote_count: 0,
        a_camp_withdrawn: false,
        a_camp_verified: false,
        a_camp_img_src: image_src.to_string(), // need fix
        a_camp_vid_src: video_src.to_string(), //need fix.
        a_camp_instagram: Default::default(),
        rules: req_parse.rules.to_string(),
        b_camp_username: b_username.to_string(),
        b_camp_vote_count: 0,
        b_camp_judge_vote_count: 0,
        b_camp_final_vote_count: 0,
        b_camp_withdrawn: false,
        b_camp_verified: false,
        b_camp_img_src: Default::default(),
        b_camp_vid_src: Default::default(),
        b_camp_instagram: Default::default(),
        b_camp_video_id: Default::default(),
        judges: judges_arr,
        reporting_sequence: Default::default(),
        closed: false,
        live_admin_name: Default::default(),
        live_admin_registration_id: Default::default(),
        response_timestamp: Default::default(),
        responder_reply: Default::default(),
        b_camp_video_height: Default::default(),
        b_camp_video_type: Default::default(),
        b_camp_video_width: Default::default(),
    };

    match match_collection
        .insert_one(create_match.clone(), None)
        .await
    {
        Ok(_) => {
            let body = format!(
                r#"
                <p>A-Camp: {}</p>
                <p>Match ID: {}</p>
                <p>Short ID: {}</p>
                <p>Img Src: {}</p>
                <p>Vid Src: {}</p>
                <p>rules: {}</p>
                <p>a_camp_video_id: {}</p>
                "#,
                a_username.to_string(),
                match_id.to_string(),
                short_id.to_string(),
                image_src.to_string(),
                video_src.to_string(),
                req_parse.rules.to_string(),
                req_parse.video_id.to_string()
            );

            let _ = send_email(
                smtp_email().to_string(),
                "DanceBattleZ".to_string(),
                body.to_string(),
            );
            return Ok(Status::Ok);
        }
        Err(e) => {
            log::error!(
                "When insert data into match collection, Error : {}",
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

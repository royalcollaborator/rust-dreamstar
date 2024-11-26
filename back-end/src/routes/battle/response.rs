use crate::config::{google_img_bucket, google_key_json, google_vid_bucket, smtp_email};
use crate::db_models::matches::Match;
use crate::db_models::user::User;
use crate::middleware::{recaptcha_verify::Recaptcha, verify_token::AuthorizedUser};
use crate::models::error_response::ErrorRes;
use crate::models::user_card::UserSelect;
use crate::services::email::send_email;

use chrono::Utc;
use google_cloud_storage::client::google_cloud_auth::credentials::CredentialsFile;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::sign::{SignedURLMethod, SignedURLOptions};
use log;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Database};
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponseUserListResModel {
    pub data: Vec<UserSelect>,
}

/**
 * Router for get user list for search
 */
#[post("/get-response-list", format = "json")]
pub async fn get_response_user_list(
    database: &State<Database>,
    auth_user: AuthorizedUser,
) -> Result<(Status, Json<GetResponseUserListResModel>), (Status, Json<ErrorRes>)> {
    // Get user_id (type is ObjectID)
    let user_id = match ObjectId::parse_str(auth_user.user_id) {
        Ok(id) => id,
        Err(_) => {
            return Err((
                Status::Unauthorized,
                Json(ErrorRes {
                    cause: "Unauthorized".to_string(),
                }),
            ));
        }
    };

    // Get Username collection
    let user_collection = database.collection::<User>("user");
    // Get Database collection
    let match_collection = database.collection::<Match>("match");

    log::info!("Accepted get user response request");

    // Filter for get reply list.
    // get a camp username
    let username = match user_collection
        .find_one(doc! { "_id" : user_id, "battler" : 1 }, None)
        .await
    {
        Ok(None) => "".to_string(),
        Ok(Some(sel_user)) => sel_user.username,
        Err(_) => "".to_string(),
    };

    // Check Username... if username doesn't exist, return
    if username.is_empty() {}

    // Filter to get all response for this user.
    let responsder_list_filter = doc! {
        "a_camp_verified" : true,
        "b_camp_username" : username.to_string(),
        "b_camp_video_id" : "".to_string(),
        "b_camp_withdrawn" : false,
        "closed" : false
    };
    // Find all match with filter
    let match_list = match match_collection.find(responsder_list_filter, None).await {
        Ok(res) => {
            // Get Vec Match
            match res.try_collect::<Vec<Match>>().await {
                Ok(result) => {
                    // Get all username in Match
                    result
                }
                Err(e) => {
                    log::error!(
                        "After get the all match list, When try_collect, Error : {} ",
                        e.to_string()
                    );
                    return Err((
                        Status::InternalServerError,
                        Json(ErrorRes {
                            cause: "Server Error ".to_string(),
                        }),
                    ));
                }
            }
        }
        Err(e) => {
            log::error!(
                "When get the all match list with filters in get , Err : {}",
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
    // if Match list is empty, return
    if match_list.len() == 0 {}

    let mut callout_users_list: Vec<UserSelect> = Vec::new();

    // Get all a_camp user informations
    for match_user in match_list {
        match user_collection
            .find_one(doc! { "username" : match_user.a_camp_username }, None)
            .await
        {
            // If username doesn't exist
            Ok(None) => {}
            Ok(Some(user)) => {
                // Push user
                callout_users_list.push(UserSelect {
                    _id: match_user.match_id.to_string(),
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
                });
            }
            Err(e) => {
                log::error!(
                    "When find users with username of match collection, Error : {}",
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
    }

    Ok((
        Status::Ok,
        Json(GetResponseUserListResModel {
            data: callout_users_list,
        }),
    ))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSignURLReqModel {
    pub a_camp_id: String,
    pub match_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSignURLResModel {
    video_id: String,
    image_id: String,
    video_url: String,
    image_url: String,
}

/**
 *  Router for, Get the video and image uuid and get the cloud sign urls
 */
#[post("/get-sign-url", format = "json", data = "<req_data>")]
pub async fn get_sign_url_for_reply(
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

    log::info!("verify judges request accept with recaptcha");

    if req_parse.a_camp_id == auth_user.user_id {
        return Err((
            Status::Conflict,
            Json(ErrorRes {
                cause: "User not allowed".to_string(),
            }),
        ));
    }
    let vid_file_id = Uuid::new_v4().to_string();
    let image_file_id = Uuid::new_v4().to_string();
    let vid_bucket_name = google_vid_bucket();
    let img_bucket_name = google_img_bucket();
    let b_user_id = ObjectId::parse_str(auth_user.user_id).unwrap();
    let a_user_id = ObjectId::parse_str(req_parse.a_camp_id).unwrap();
    let user_collection = database.collection::<User>("user");
    let match_collection = database.collection::<Match>("match");

    // Todo
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

    let _ = match match_collection
        .find_one(doc! { "match_id" : req_parse.match_id.to_string() }, None)
        .await
    {
        Ok(None) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "Can't find match id".to_string(),
                }),
            ));
        }
        Ok(Some(sel_data)) => {
            if sel_data.a_camp_verified
                && sel_data.b_camp_video_id == "".to_string()
                && sel_data.a_camp_username == a_username.to_string()
                && sel_data.b_camp_username == b_username.to_string()
            {
                true
            } else {
                return Err((
                    Status::BadRequest,
                    Json(ErrorRes {
                        cause: "Condition doesn't match ".to_string(),
                    }),
                ));
            }
        }
        Err(e) => {
            log::error!(
                "When verify the match collection information, Error  : {} ",
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
                    // Create Signed URL for video upload
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

                    // Create Signed URL for image upload
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetReplyReqModel {
    pub match_id: String,
    pub a_camp_id: String,
    pub video_id: String,
    pub image_id: String,
    pub video_type: String,
    pub responder_reply: String,
}

/**
 * Router for setting reply
 */
#[post("/set-reply", format = "json", data = "<req_data>")]
pub async fn set_reply(
    recaptcha: Recaptcha,
    auth_user: AuthorizedUser,
    database: &State<Database>,
    req_data: Option<Json<SetReplyReqModel>>,
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
    let a_user_id = ObjectId::parse_str(req_parse.a_camp_id).unwrap();
    let b_user_id = ObjectId::parse_str(auth_user.user_id).unwrap();
    let user_collection = database.collection::<User>("user");
    let match_collection = database.collection::<Match>("match");
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

    // Check usernames does match with match_id
    let target_match = match match_collection
        .find_one(doc! { "match_id" : req_parse.match_id.to_string() }, None)
        .await
    {
        Ok(None) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "Match Id doesn't exist".to_string(),
                }),
            ));
        }
        Ok(Some(res)) => res,
        Err(e) => {
            log::error!(
                "When get info from match collection using match id, Error : {}",
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
    // Check callout username and reply username is correct.
    if target_match.a_camp_username != a_username.to_string()
        || target_match.b_camp_username != b_username.to_string()
    {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "usernames and target id doesn't match".to_string(),
            }),
        ));
    }

    let image_src = format!(
        "https://storage.googleapis.com/{}/images/{}.jpeg",
        google_img_bucket().to_string(),
        req_parse.image_id.to_string()
    );
    let video_src = format!(
        "https://storage.googleapis.com/{}/videos/{}.{}",
        google_vid_bucket().to_string(),
        req_parse.video_id.to_string(),
        req_parse.video_type.to_string()
    );

    let match_update = doc! {
        "$set" : {
            "b_camp_video_id" : req_parse.video_id.to_string(),
            "b_camp_video_width" : 0,
            "b_camp_video_height" : 0,
            "b_camp_video_type" : req_parse.video_type,
            "b_camp_img_src" : image_src.to_string(),
            "b_camp_vid_src" : video_src.to_string(),
            "responder_reply" : req_parse.responder_reply.to_string(),
            "last_updated_timestamp"  : now_time,
            "response_timestamp" : now_time
        }
    };

    // Set reply
    match match_collection
        .update_one(
            doc! { "match_id" : req_parse.match_id.to_string() },
            match_update,
            None,
        )
        .await
    {
        Ok(_) => {
            // Send email
            let body = format!(
                r#"
                <p>B-Camp : {}</p>
                <p>Match ID: {}</p>
                <p>Img Src: {}</p>
                <p>Vid Src: {}</p>
                <p>rules: {}</p>
                <p>b_camp_video_id: {}</p>
                "#,
                b_username.to_string(),
                req_parse.match_id.to_string(),
                image_src.to_string(),
                video_src.to_string(),
                req_parse.responder_reply.to_string(),
                req_parse.video_id.to_string()
            );

            let _ = send_email(
                smtp_email().to_string(),
                "Response Request is pending".to_string(),
                body.to_string(),
            );
            Ok(Status::Ok)
        }
        Err(e) => {
            log::error!(
                "When update match collection for reply, Error : {}",
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

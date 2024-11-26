use crate::db_models::{invitation::InvitationCode, user::User};
use crate::middleware::recaptcha_verify::Recaptcha;
use crate::models::error_response::ErrorRes;
use crate::services::email::send_email;
use crate::utils::util::{generate_otp, hash_text};
use chrono::Utc;
use log;
use mongodb::{bson::doc, bson::oid::ObjectId, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignupReqModel {
    email: String,
    username: String,
    password: String,
}

#[post("/signup", format = "json", data = "<signup_data>")]
pub async fn signup(
    recaptcha: Recaptcha,
    database: &State<Database>,
    signup_data: Option<Json<SignupReqModel>>,
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
    let users = database.collection::<User>("user");
    let invitation_collection = database.collection::<InvitationCode>("invitation_code");
    // Check request type
    // if is is invalid, return bad request status
    let req_data = match signup_data {
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
    let email = req_data.email;
    let username = req_data.username;
    let password = req_data.password;

    log::info!("signup request accept with recaptcha verify from");

    let user_filter = doc! {
        "$or"  : [
            {"username" : username.clone()},
            {"email" : email.clone()},
            {"google_email" : email.clone()}
        ]
    };
    // Check username or email exist
    match users.find_one(user_filter, None).await {
        Ok(Some(user_result)) => {
            if user_result.account_status == "invited" && user_result.email == email {
                Err((
                    Status::AlreadyReported,
                    Json(ErrorRes {
                        cause: format!(
                            "You 've already registered but your account need email verify"
                        ),
                    }),
                ))
            } else {
                Err((
                    Status::Conflict,
                    Json(ErrorRes {
                        cause: format!("Email or Username already registered"),
                    }),
                ))
            }
        }
        Ok(None) => {
            let hashed_password = hash_text(password.clone(), 4).unwrap();
            let invitation_code = generate_otp();
            let requestor = User {
                _id: ObjectId::new(),
                username: username.clone(),
                email: email.clone(),
                temp_email: Default::default(),
                password: hashed_password.clone(),
                registration_timestamp: Utc::now().timestamp(),
                invitation_code: invitation_code.clone(),
                account_status: "invited".to_string(),
                battler_score: 0.00001,
                youtube_channels: Default::default(),
                youtube_channel_id: Default::default(),
                youtube_channel_name: Default::default(),
                youtube_thumbnail: Default::default(),
                youtube_state_code: Default::default(),
                twitter: Default::default(),
                twitter_name: Default::default(),
                twitter_token: Default::default(),
                twitter_token_secret: Default::default(),
                instagram_id: Default::default(),
                instagram_name: Default::default(),
                instagram_state_code: Default::default(),
                instagram_thumbnail: Default::default(),
                google_id: Default::default(),
                google_email: Default::default(),
                apple_id: Default::default(),
                apple_email: Default::default(),
                apple_state_code: Default::default(),
                apple_last_refresh: 0,
                paypal: Default::default(),
                highest_rank: 0,
                highest_rank_out_of: 0,
                votes_for: 0,
                votes_against: 0,
                current_votes_for: 0,
                judge_votes: 0,
                final_votes: 0,
                vote_periods: Default::default(),
                pay_cycle_events: Default::default(),
                password_reset_code: Default::default(),
                email_reset_code: Default::default(),
                session_id: Uuid::new_v4().to_string(),
                browser_id: Uuid::new_v4().to_string(),
                device_id: Uuid::new_v4().to_string(),
                matches_won: Default::default(),
                matches_lost: Default::default(),
                matches_withdrawn: Default::default(),
                work_a: Default::default(),
                work_b: Default::default(),
                top_ten_callouts: Default::default(),
                battler: 0,
                voter: 1,
                admin: 0,
                one_hundred_badge: Default::default(),
                first_tourney_badge: Default::default(),
                judge: Default::default(),
                live_admin: Default::default(),
            };
            match users.insert_one(requestor.clone(), None).await {
                Ok(_) => {
                    let user_invitation = InvitationCode {
                        _id: ObjectId::new(),
                        email: email.to_string(),
                        code: invitation_code.to_string(),
                        time_stamp: Utc::now().timestamp(),
                    };
                    match invitation_collection
                        .insert_one(user_invitation.clone(), None)
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!(
                                "Error in interact with database in signup invitation code insert in invitation collection, Error : {}",
                                e.to_string()
                            );
                        }
                    }
                    let body = format!(
                        "<p> Please paste this code to get started: {}</p>\n<p>This code will expire in 24 hours.</p>\n<p>Remember to read our policies on the bottom of the login before experiencing this app. Thank you.</p>\n<p>\"What's your style?\"</p>",
                        requestor.invitation_code
                    );
                    match send_email(
                        email.to_string(),
                        "DanceBattleZ".to_string(),
                        body.to_string(),
                    ) {
                        Ok(_) => Ok(Status::Ok),
                        Err(_) => Ok(Status::Ok),
                    }
                }
                Err(e) => {
                    log::error!(
                        "Error in interact with database in signup user insert in user collection, Error : {}",
                        e.to_string()
                    );
                    Err((
                        Status::InternalServerError,
                        Json(ErrorRes {
                            cause: format!("Server Error"),
                        }),
                    ))
                }
            }
        }
        Err(e) => {
            log::error!(
                "Error in interact with database in signup find user in user collection, Error : {}",
                e.to_string()
            );
            Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: format!("Server Error"),
                }),
            ))
        }
    }
}

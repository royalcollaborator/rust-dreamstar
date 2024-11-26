use crate::config::email_expire;
use crate::db_models::{invitation::InvitationCode, user::User};
use crate::middleware::recaptcha_verify::Recaptcha;
use crate::models::error_response::ErrorRes;
use crate::services::email::send_email;
use crate::utils::util::convert_str_to_i32;
use crate::utils::util::{generate_otp, hash_text};
use chrono::{Duration, Utc};
use log;
use mongodb::{bson::doc, bson::oid::ObjectId, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgetPassReqModel {
    email: String,
    password: String,
    code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForgetPassEmailReqModel {
    email: String,
}

#[post("/reset-pass-send-email", format = "json", data = "<data>")]
pub async fn reset_pass_send_email(
    recaptcha: Recaptcha,
    database: &State<Database>,
    data: Option<Json<ForgetPassEmailReqModel>>,
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
    let invitation_collection = database.collection::<InvitationCode>("invitation_code");
    let user_collection = database.collection::<User>("user");
    let req_data = match data {
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
    let data_info = req_data.into_inner();
    let email = data_info.email.to_string();

    log::info!("reset password email send  request accept with recaptcha verify from ");

    match user_collection
        .find_one(
            doc! { "email" : email.to_string(), "account_status" : "registered" },
            None,
        )
        .await
    {
        Ok(None) => Err((
            Status::Conflict,
            Json(ErrorRes {
                cause: "User not registered".to_string(),
            }),
        )),
        Ok(_) => {
            let generated_code = generate_otp();
            match user_collection
                .update_one(
                    doc! { "email" : email.to_string() },
                    doc! {"$set" :
                        {"password_reset_code" : generated_code.to_string()}
                    },
                    None,
                )
                .await
            {
                Ok(_) => {
                    let body = format!(
                        "<p> Please paste this code to get started: {}</p>\n<p>This code will expire in 24 hours.</p>\n<p>Remember to read our policies on the bottom of the login before experiencing this app. Thank you.</p>\n<p>\"What's your style?\"</p>",
                        generated_code.to_string()
                    );
                    match send_email(
                        email.to_string(),
                        "DanceBattleZ".to_string(),
                        body.to_string(),
                    ) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                    match invitation_collection
                        .find_one(doc! { "email" : email.to_string() }, None)
                        .await
                    {
                        Ok(None) => {
                            let create_invitation = InvitationCode {
                                _id: ObjectId::new(),
                                email: email.to_string(),
                                code: generated_code.to_string(),
                                time_stamp: Utc::now().timestamp(),
                            };
                            match invitation_collection
                                .insert_one(create_invitation.clone(), None)
                                .await
                            {
                                Ok(_) => Ok(Status::Ok),
                                Err(e) => {
                                    // Error because, Can't insert invitation collection in reset password
                                    log::error!(
                                        "Can't insert invitation collection in reset password : {}",
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
                        Ok(_) => {
                            let update_doc = doc! {
                             "$set" : {
                                 "code" : generated_code.to_string(),
                                 "time_stamp" : Utc::now().timestamp(),
                             }
                            };
                            match invitation_collection
                                .update_one(doc! { "email" : email.to_string() }, update_doc, None)
                                .await
                            {
                                Ok(_) => Ok(Status::Ok),
                                Err(e) => {
                                    // Error because, Can't update invitation collection in reset password.
                                    log::error!(
                                        "Can't update invitation collection in reset password : {}",
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
                        Err(e) => {
                            // Error because, when find data in invitation collection, Error occur
                            log::error!(
                                "when find data in invitation collection, Error occur : {}",
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
                Err(e) => {
                    // Can't update user collection.
                    log::error!("Can't update user collection : {}", e.to_string());
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
            // When find data in user collection, error occur
            log::error!(
                "When find data in user collection, Error : {}",
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

#[post("/reset-pass", format = "json", data = "<forget_data>")]
pub async fn reset_pass(
    database: &State<Database>,
    recaptcha: Recaptcha,
    forget_data: Option<Json<ForgetPassReqModel>>,
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
    let invitation_collection = database.collection::<InvitationCode>("invitation_code");
    let user_collection = database.collection::<User>("user");
    // Check request data
    // if request is invalid, return bad request status
    let req_data = match forget_data {
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
    let password = req_data.password;
    let code = req_data.code;

    log::info!("forget password request accept with recaptcha verify");

    let invitation_filter = doc! { "email" : email.to_string(), "code" : code.to_string() };
    let _ = match invitation_collection
        .find_one(invitation_filter, None)
        .await
    {
        Ok(None) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Email or code doesn't match"),
                }),
            ));
        }
        Ok(Some(sel_user)) => {
            let email_expire_time = convert_str_to_i32(email_expire().as_str());
            if sel_user.time_stamp
                < (Utc::now() - Duration::hours(email_expire_time.into())).timestamp()
            {
                return Err((
                    Status::GatewayTimeout,
                    Json(ErrorRes {
                        cause: format!("Code expired"),
                    }),
                ));
            }
            true
        }
        Err(e) => {
            log::error!("When find code, Error : {}", e.to_string());
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: format!("Server Error"),
                }),
            ));
        }
    };

    match user_collection
        .find_one(
            doc! { "email" : email.to_string(), "password_reset_code" : code.to_string() },
            None,
        )
        .await
    {
        Ok(None) => Err((
            Status::Conflict,
            Json(ErrorRes {
                cause: "User or code doesn't exist".to_string(),
            }),
        )),
        Ok(_) => {
            let hash_password = hash_text(password.to_string(), 4).unwrap();
            match user_collection
                .update_one(
                    doc! { "email" : email.to_string() },
                    doc! {"$set" : {
                       "password"  : hash_password.to_string()
                    }},
                    None,
                )
                .await
            {
                Ok(_) => Ok(Status::Ok),
                Err(e) => {
                    // when update user collection, error occur
                    log::error!(
                        "when update user collection, error occur : {}",
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
        Err(e) => {
            // When find data in user collection, Error
            log::error!(
                "When find data in user collection, Error : {}",
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

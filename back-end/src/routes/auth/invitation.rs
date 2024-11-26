use crate::config::email_expire;
use crate::config::is_test;
use crate::db_models::{invitation::InvitationCode, user::User};
use crate::middleware::recaptcha_verify::Recaptcha;
use crate::models::error_response::ErrorRes;
use crate::services::email::send_email;
use crate::utils::util::convert_str_to_i32;
use crate::utils::util::generate_otp;
use chrono::{Duration, Utc};
use mongodb::{bson::doc, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvitationReqModel {
    pub email: String,
    pub code: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvitationResendReqModel {
    pub email: String,
}

/**
 * Router for checking invitation code.
 */
#[post("/invitation", format = "json", data = "<invitation_data>")]
pub async fn invitation_checked(
    recaptcha: Recaptcha,
    database: &State<Database>,
    invitation_data: Option<Json<InvitationReqModel>>,
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
    // Check request parameter
    // if request parameter is not correct, return bad request status
    let req_data = match invitation_data {
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
    let code = req_data.code;
    let invitation_collection = database.collection::<InvitationCode>("invitation_code");
    let invitation_filter = doc! { "email" : email.to_string(), "code" : code.to_string() };

    log::info!("invitation request accept with recaptcha verify");

    match invitation_collection
        .find_one(invitation_filter, None)
        .await
    {
        Ok(None) => Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: format!("Email is not registered"),
            }),
        )),
        Ok(Some(sel_user)) => {
            let email_expire_time = convert_str_to_i32(email_expire().as_str());
            if sel_user.time_stamp
                > (Utc::now() - Duration::hours(email_expire_time.into())).timestamp()
            {
                let user_collection = database.collection::<User>("user");
                let user_filter =
                    doc! { "email" : email.to_string(), "invitation_code" : code.to_string() };
                let update_query = doc! {"$set" : {
                    "account_status" : "registered"
                }};
                match user_collection
                    .update_one(user_filter.clone(), update_query, None)
                    .await
                {
                    Ok(_) => {
                        match invitation_collection
                            .delete_one(
                                doc! { "email" : email.to_string(), "code" : code.to_string() },
                                None,
                            )
                            .await
                        {
                            Ok(_) => Ok(Status::Ok),
                            Err(e) => {
                                log::error!(
                                    "When delete data in invitation collection, Error : {}",
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
                            "invitation failed because of interact with database, Error : {}",
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
            } else {
                Err((
                    Status::GatewayTimeout,
                    Json(ErrorRes {
                        cause: format!("Code expired"),
                    }),
                ))
            }
        }

        Err(e) => {
            log::error!(
                "invitation failed because of interact with database Error : {}",
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

/**
 * Router for resend invitation code
 */
#[post("/invitation/resend", format = "json", data = "<invitation_data>")]
pub async fn reinvitation(
    database: &State<Database>,
    recaptcha: Recaptcha,
    invitation_data: Option<Json<InvitationResendReqModel>>,
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
    // Check request parameter
    // if request parameter is not correct, return bad request status
    let req_data = match invitation_data {
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
    let new_invitation_code = generate_otp();
    let email = req_data.email;
    let invitation_collection = database.collection::<InvitationCode>("invitation_code");
    let user_collection = database.collection::<User>("user");
    let invitation_filter = doc! { "email" : email.to_string() };
    let invitation_update = doc! { "$set" : {"code" : new_invitation_code.to_string()} };
    log::info!("Reinvitation request accepted with recaptcha verify");
    match invitation_collection
        .find_one(doc! { "email" : email.to_string() }, None)
        .await
    {
        Ok(_) => {
            match invitation_collection
                .update_one(invitation_filter, invitation_update, None)
                .await
            {
                Ok(_) => {
                    let email_clone = email.clone();
                    let new_invitation_code_clone = new_invitation_code.clone();
                    match user_collection
                        .update_one(
                            doc! { "email": email_clone },
                            doc! { "$set": {"invitation_code" : new_invitation_code_clone} },
                            None,
                        )
                        .await
                    {
                        Ok(_) => {
                            let body = format!(
                                "<p> Please paste this code to get started: {}</p>\n<p>This code will expire in 24 hours.</p>\n<p>Remember to read our policies on the bottom of the login before experiencing this app. Thank you.</p>\n<p>\"What's your style?\"</p>",
                                new_invitation_code.to_string()
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
                                "reinvitation code filed because of interact with database ( update invitation code in invitation collection ), Error :  {}",
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
                        "reinvitation code filed because of interact with database ( update invitation code in invitation collection ), Error : {}",
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
                "reinvitation code filed because of interact with database ( find email in invitation collection ), Error :  {}",
                e.to_string()
            );
            Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Email does not exist"),
                }),
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetInvitationCode {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetInvitationCodeRes {
    pub code: String,
}

/**
 * Router for get the invitation code (this router will work on only test version)
 */
#[post("/invitation/get-invitation-e2e", format = "json", data = "<req_data>")]
pub async fn test_get_invitation_code(
    database: &State<Database>,
    req_data: Option<Json<GetInvitationCode>>,
) -> Result<(Status, Json<GetInvitationCodeRes>), (Status, Json<ErrorRes>)> {
    // Check this is test version
    if is_test() != "true" {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: format!("This is not test environment`"),
            }),
        ));
    }

    let invitation_collection = database.collection::<InvitationCode>("invitation_code");
    // Check request parameter
    // if request parameter is not correct, return bad request status
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
    // get the invitation code base on user's email
    match invitation_collection
        .find_one(doc! {"email" : req_data.email.to_string()}, None)
        .await
    {
        Ok(None) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Can't find invitation code"),
                }),
            ));
        }
        Ok(Some(val)) => {
            return Ok((
                Status::Ok,
                Json(GetInvitationCodeRes {
                    code: val.code.to_string(),
                }),
            ))
        }
        Err(e) => {
            log::error!(
                "when trying to get the e2e test, Error :  {}",
                e.to_string()
            );
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Server Error"),
                }),
            ));
        }
    }
}

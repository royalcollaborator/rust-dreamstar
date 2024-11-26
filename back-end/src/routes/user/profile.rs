use crate::config::email_expire;
use crate::db_models::user::User;
use crate::db_models::InvitationCode;
use crate::middleware::{recaptcha_verify::Recaptcha, verify_token::AuthorizedUser};
use crate::models::error_response::ErrorRes;
use crate::services::email::send_email;
use crate::utils::util::convert_str_to_i32;
use crate::utils::util::{encode_jwt, generate_otp, hash_text};
use chrono::{Duration, Utc};
use log;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PasswordChangeReqModel {
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsernameChangeReqModel {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResModel {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResModel {
    pub username: String,
    pub email: String,
    pub google_email: String,
    pub instagram_name: String,
    pub voter : bool,
    pub battler : bool,
    pub judger : bool,
    pub admin : bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResModel {
    token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetEmailReqModel {
    email: String,
    code: String,
}

/**
 * Router for email reset
 */

#[post("/reset-email", format = "json", data = "<forget_data>")]
pub async fn reset_email(
    recaptcha: Recaptcha,
    auth_user: AuthorizedUser,
    database: &State<Database>,
    forget_data: Option<Json<ResetEmailReqModel>>,
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
    // if it is invalid, return bad request status
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
    let code = req_data.code;
    let user_id = ObjectId::parse_str(auth_user.user_id).unwrap();

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
                    cause: format!("Email or Code doesn't match"),
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
    // Find user with temp_email and code
    match
        user_collection.find_one(
            doc! { "_id" : user_id, "email_reset_code" : code.to_string(), "temp_email" : email.to_string() },
            None
        ).await
    {
        Ok(None) =>
            Err((
                Status::Conflict,
                Json(ErrorRes {
                    cause: "User or code doesn't exist".to_string(),
                }),
            )),
        Ok(_) => {
            let user_update =
                doc! {"$set" : {
                "temp_email" : "".to_string(),
                "email" : email.to_string(),
                "email_reset_code" : "".to_string()
            }};
            match user_collection.update_one(doc! { "_id" : user_id }, user_update, None).await {
                Ok(_) => { Ok(Status::Ok) }
                Err(e) => {
                    log::error!(
                        "When update user collection for reset email, Error : {}",
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
            log::error!("When find data in user collection, Error : {}", e.to_string());
            Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error".to_string(),
                }),
            ))
        }
    }
}

/**
 * Email change send invitation code
 */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailChangeCodeReqModel {
    new_email: String,
}

#[post("/email-change-code", format = "json", data = "<data>")]
pub async fn email_change_code(
    recaptcha: Recaptcha,
    auth_user: AuthorizedUser,
    database: &State<Database>,
    data: Option<Json<EmailChangeCodeReqModel>>,
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
    // if it is valid, return bad request status
    let data_info = match data {
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
    let email = data_info.new_email.to_string();
    let user_id = ObjectId::parse_str(auth_user.user_id).unwrap();
    log::info!("Email change code request accept with recaptcha verify from ");
    let generated_code = generate_otp();
    // Check email vailidation
    let _ = match user_collection
        .find_one(doc! { "email" : email.to_string() }, None)
        .await
    {
        Ok(None) => true,
        Ok(Some(_)) => {
            return Err((
                Status::AlreadyReported,
                Json(ErrorRes {
                    cause: "This email already used".to_string(),
                }),
            ));
        }
        Err(e) => {
            log::error!(
                "Can't update invitation collection in email change : {}",
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
    match user_collection
        .update_one(
            doc! { "_id" : user_id },
            doc! {"$set" :
                {"email_reset_code" : generated_code.to_string(), "temp_email" : email.to_string()}
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
                            // Error because, Can't insert invitation collection in email change
                            log::error!(
                                "Can't insert invitation collection in email change : {}",
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
                            // Error because, Can't update invitation collection in email change.
                            log::error!(
                                "Can't update invitation collection in email change : {}",
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

/**
 * Router for password change in user profile page
 */
#[post("/passwordChange", format = "json", data = "<change_data>")]
pub async fn password_change(
    auth_user: AuthorizedUser,
    recaptcha: Recaptcha,
    database: &State<Database>,
    change_data: Option<Json<PasswordChangeReqModel>>,
) -> Result<(Status, Json<TokenResModel>), (Status, Json<ErrorRes>)> {
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
    let req_data = match change_data {
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
    let password = req_data.password.to_string();
    let user_id = auth_user.user_id;
    let user_collection = database.collection::<User>("user");
    log::info!("password change request accept with recaptcha verify and token verify");
    match ObjectId::parse_str(user_id) {
        Ok(id) => match hash_text(password.to_string(), 4) {
            Ok(hash_pass) => {
                match user_collection
                    .update_one(
                        doc! { "_id" : id },
                        doc! { "$set" : {"password" : hash_pass.to_string()} },
                        None,
                    )
                    .await
                {
                    Ok(_) => match encode_jwt(id) {
                        Ok(token) => Ok((
                            Status::Ok,
                            Json(TokenResModel {
                                token: token.to_string(),
                            }),
                        )),
                        Err(e) => {
                            log::error!("When encode jwt, Error : {}", e.to_string());
                            Err((
                                Status::InternalServerError,
                                Json(ErrorRes {
                                    cause: format!("Server Error"),
                                }),
                            ))
                        }
                    },
                    Err(e) => {
                        log::error!(
                            "When update data in user collection, Error : {}",
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
            Err(_) => {
                log::error!("When hash password, Error");
                Err((
                    Status::InternalServerError,
                    Json(ErrorRes {
                        cause: format!("Server Error"),
                    }),
                ))
            }
        },
        Err(e) => {
            log::error!("When parse Object id, Error : {}", e.to_string());
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
 * Router for username change in user profile page
 */
#[post("/usernameChange", format = "json", data = "<change_data>")]
pub async fn username_change(
    auth_user: AuthorizedUser,
    recaptcha: Recaptcha,
    database: &State<Database>,
    change_data: Option<Json<UsernameChangeReqModel>>,
) -> Result<(Status, Json<TokenResModel>), (Status, Json<ErrorRes>)> {
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
    // if it is  invalid, return bad request status
    let req_data = match change_data {
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
    let username = req_data.username.to_string();
    let user_id = auth_user.user_id.to_string();
    let user_collection = database.collection::<User>("user");
    log::info!("username change request accept with recaptcha verify and token verify");
    match user_collection
        .find_one(doc! { "username" : username.to_string() }, None)
        .await
    {
        Ok(None) => match ObjectId::parse_str(user_id) {
            Ok(id) => {
                match user_collection
                    .update_one(
                        doc! { "_id" : id },
                        doc! { "$set" : {"username" : username.to_string()} },
                        None,
                    )
                    .await
                {
                    Ok(_) => match encode_jwt(id) {
                        Ok(token) => Ok((
                            Status::Ok,
                            Json(TokenResModel {
                                token: token.to_string(),
                            }),
                        )),
                        Err(_) => {
                            log::error!("Error when encode jwt");
                            Err((
                                Status::InternalServerError,
                                Json(ErrorRes {
                                    cause: format!("Server Error"),
                                }),
                            ))
                        }
                    },
                    Err(e) => {
                        log::error!("When update user collection, Error : {}", e.to_string());
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
                    "When update data in user collection, Error : {}",
                    e.to_string()
                );
                Err((
                    Status::InternalServerError,
                    Json(ErrorRes {
                        cause: format!("Server Error"),
                    }),
                ))
            }
        },
        Ok(_) => Err((
            Status::AlreadyReported,
            Json(ErrorRes { cause: format!("") }),
        )),
        Err(e) => {
            log::error!(
                "When find data in user collection, Error : {}",
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

#[get("/userInfo")]
pub async fn get_user_info(
    auth_user: AuthorizedUser,
    database: &State<Database>,
) -> Result<(Status, Json<UserResModel>), (Status, Json<ErrorRes>)> {

    let user_id = auth_user.user_id.to_string();
    let user_collection = database.collection::<User>("user");
    match ObjectId::parse_str(user_id) {
        Ok(id) => match user_collection.find_one(doc! { "_id" : id }, None).await {
            Ok(Some(sel_user)) => Ok((
                Status::Ok,
                Json(UserResModel {
                    username: sel_user.username,
                    email: sel_user.email,
                    google_email: sel_user.google_email,
                    instagram_name: sel_user.instagram_name,
                    voter : if sel_user.voter == 1 {
                        true
                    } else {
                        false
                    },
                    battler : if sel_user.battler == 1 {
                        true
                    } else {
                        false
                    },
                    judger : if sel_user.judge == 1 {
                        true
                    } else {
                        false
                    },
                    admin : if sel_user.admin == 1 {
                        true
                    } else {
                        false
                    }
                }),
            )),
            Ok(None) => Err((
                Status::Forbidden,
                Json(ErrorRes {
                    cause: format!("You are robot"),
                }),
            )),
            Err(e) => {
                log::error!(
                    "When find data in user collection, Error : {}",
                    e.to_string()
                );
                Err((
                    Status::InternalServerError,
                    Json(ErrorRes {
                        cause: format!("Server Error"),
                    }),
                ))
            }
        },
        Err(e) => {
            log::error!(
                "When parse Object id from string, Error : {}",
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
 * Router for check login status
 */
#[get("/check-login", format = "json")]
pub async fn get_user_information(
    auth_user: AuthorizedUser,
    database: &State<Database>,
) -> Result<(Status, String), Status> {
    // User Collection
    let user_collection = database.collection::<User>("user");
    // Get User id from middleware
    let user_id = ObjectId::parse_str(auth_user.user_id).unwrap();
    // Get username using user_id
    let username = match user_collection
        .find_one(doc! { "_id" : user_id }, None)
        .await
    {
        Ok(None) => {
            return Err(Status::Unauthorized);
        }
        Ok(Some(res)) => res.username,
        Err(e) => {
            log::error!(
                "When get username from user collection with user_id, Error : {}",
                e.to_string()
            );
            return Err(Status::InternalServerError);
        }
    };
    Ok((Status::Ok, username.to_string()))
}

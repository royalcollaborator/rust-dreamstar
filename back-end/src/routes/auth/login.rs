use crate::db_models::user::User;
use crate::middleware::recaptcha_verify::Recaptcha;
use crate::models::error_response::ErrorRes;
use crate::utils::util::encode_jwt;
use crate::utils::util::{decode_jwt, DecodeJwtHelper};
use bcrypt::verify;
use chrono::Utc;
use log;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginReqModel {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResModel {
    pub token: String,
}
/**
 * Router for user login (user will login using username and password)
 */
#[post("/login", format = "json", data = "<login_data>")]
pub async fn login(
    recaptcha: Recaptcha,
    database: &State<Database>,
    login_data: Option<Json<LoginReqModel>>,
) -> Result<(Status, Json<LoginResModel>), (Status, Json<ErrorRes>)> {
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
    // if it is invalid, return bad request
    let req_data = match login_data {
        Some(val) => val.into_inner(),
        None => {
            return Err((Status::BadRequest, Json(ErrorRes { cause: format!("") })));
        }
    };
    let email = req_data.email.to_string();
    let password = req_data.password.to_string();
    let user_collection = database.collection::<User>("user");

    log::info!("login request accept with recaptcha verify");

    match user_collection
        .find_one(
            doc! { "$or" : [
            {"email" : email.to_string()},
             {"username" : email.to_string()}
             ], "account_status" : "registered"},
            None,
        )
        .await
    {
        Ok(Some(sel_user)) => match verify(password.to_string(), &sel_user.password) {
            Ok(true) => match encode_jwt(sel_user._id) {
                Ok(tokens) => Ok((Status::Ok, Json(LoginResModel { token: tokens }))),
                Err(e) => {
                    log::error!(
                        "login failed because this error : jwt token create failed, Error :  {}",
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
            Ok(false) => Err((
                Status::Unauthorized,
                Json(ErrorRes {
                    cause: format!("Email or Password does not match"),
                }),
            )),
            Err(_) => Err((
                Status::Unauthorized,
                Json(ErrorRes {
                    cause: format!("Email or Password does not match"),
                }),
            )),
        },
        Ok(None) => Err((
            Status::Unauthorized,
            Json(ErrorRes {
                cause: format!("Email or Password doesn't match"),
            }),
        )),
        Err(e) => {
            log::error!("error in database management, Error : {}", e.to_string());
            Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: format!("Server Error"),
                }),
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthCheckReqModel {
    token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthCheckResModel {
    result: bool,
    admin: bool,
}

/**
 * Router for auth checking (using this router, we can know user is logged in user or not)
 */
#[post("/auth-check", format = "json", data = "<auth_check>")]
pub async fn auth_check(
    auth_check: Option<Json<AuthCheckReqModel>>,
    database: &State<Database>,
) -> Result<(Status, Json<AuthCheckResModel>), (Status, Json<ErrorRes>)> {
    // Check request parameter
    // if request parameter is not correct, return bad request
    let req_data = match auth_check {
        Some(val) => val.into_inner(),
        None => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("Email is not registered"),
                }),
            ));
        }
    };
    let user_collection = database.collection::<User>("user");
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
    // Check user id is exist and it is not empty
    if auth_user_id.is_empty() {
        Ok((
            Status::Ok,
            Json(AuthCheckResModel {
                result: false,
                admin: false,
            }),
        ))
    } else {
        // Parse userID to ObjectID
        match ObjectId::parse_str(auth_user_id.as_str()) {
            // If convert is success
            Ok(id) => match user_collection.find_one(doc! {"_id" : id}, None).await {
                // If this user is not exist, return false
                Ok(None) => Ok((
                    Status::Ok,
                    Json(AuthCheckResModel {
                        result: false,
                        admin: false,
                    }),
                )),
                // If this user is  exist, return true
                Ok(Some(user)) => {
                    if user.admin == 1 {
                        Ok((
                            Status::Ok,
                            Json(AuthCheckResModel {
                                result: true,
                                admin: true,
                            }),
                        ))
                    } else {
                        Ok((
                            Status::Ok,
                            Json(AuthCheckResModel {
                                result: true,
                                admin: false,
                            }),
                        ))
                    }
                }
                // If faced error, return false
                Err(_) => Ok((
                    Status::Ok,
                    Json(AuthCheckResModel {
                        result: false,
                        admin: false,
                    }),
                )),
            },
            // Error
            Err(_) => Ok((
                Status::Ok,
                Json(AuthCheckResModel {
                    result: false,
                    admin: false,
                }),
            )),
        }
    }
}

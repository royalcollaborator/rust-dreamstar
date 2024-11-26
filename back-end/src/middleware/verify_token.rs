use crate::db::get_database;
use crate::db_models::user::User;
use crate::utils::util::{decode_jwt, DecodeJwtHelper};
use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Database};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct AuthorizedUser {
    pub user_id: String,
}

/**
 * check data from request auth (Bearer token)
 */
pub fn check_data_from_auth_header(auth_header: Option<&str>) -> Result<Vec<&str>, ()> {
    return if let Some(auth_string) = auth_header {
        let vec_header = auth_string.split_whitespace().collect::<Vec<_>>();
        if vec_header.len() != 2
            || vec_header[0].is_empty()
            || vec_header[0] != "Bearer"
            || vec_header[1].is_empty()
        {
            Err(())
        } else {
            Ok(vec_header)
        }
    } else {
        Err(())
    };
}

/**
 * Passport function
 * Every request mush go though this function
 * It pass when token existed
 */
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        match check_data_from_auth_header(auth_header) {
            Ok(vec_header) => match decode_jwt(vec_header[1].to_string()) {
                DecodeJwtHelper::Ok(token_data) => {
                    let expire_time = token_data.claims.exp;
                    let now_time = Utc::now().timestamp() as usize;
                    if expire_time > now_time {
                        Outcome::Success(AuthorizedUser {
                            user_id: token_data.claims.user_id,
                        })
                    } else {
                        Outcome::Error((Status::Unauthorized, ()))
                    }
                }
                DecodeJwtHelper::Err => Outcome::Error((Status::Unauthorized, ())),
            },
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

/**
 * For Admin
 */

pub struct AuthorizedAdmin {
    pub user_id: String,
}

/**
 * Passport function
 * Every request mush go though this function
 * It pass when token existed
 */
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedAdmin {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let database = get_database().await;
        let user_collection = database.collection::<User>("user");
        let auth_header = request.headers().get_one("Authorization");
        match check_data_from_auth_header(auth_header) {
            Ok(vec_header) => match decode_jwt(vec_header[1].to_string()) {
                DecodeJwtHelper::Ok(token_data) => {
                    let expire_time = token_data.claims.exp;
                    let now_time = Utc::now().timestamp() as usize;
                    if expire_time > now_time {
                        match ObjectId::parse_str(token_data.claims.user_id.to_string()) {
                            Ok(id) => {
                                match user_collection
                                    .find_one(doc! {"_id" : id.clone(), "admin" : 1}, None)
                                    .await
                                {
                                    Ok(None) => Outcome::Error((Status::Unauthorized, ())),
                                    Ok(Some(_)) => Outcome::Success(AuthorizedAdmin {
                                        user_id: token_data.claims.user_id,
                                    }),
                                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                                }
                            }
                            Err(_) => Outcome::Error((Status::Unauthorized, ())),
                        }
                    } else {
                        Outcome::Error((Status::Unauthorized, ()))
                    }
                }
                DecodeJwtHelper::Err => Outcome::Error((Status::Unauthorized, ())),
            },
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

use crate::db_models::user::User;
use crate::middleware::verify_token::AuthorizedAdmin;
use crate::models::error_response::ErrorRes;

use log;
use mongodb::{bson::doc, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRoleChange {
    pub username: String,
    pub role: String,
    pub status: bool,
}

/**
 * Admin Router for  user role change
 */
#[post("/role-setup", format = "json", data = "<req_data>")]
pub async fn user_role_setup(
    database: &State<Database>,
    req_data: Option<Json<UserRoleChange>>,
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
    // Get user collection
    let user_collection = database.collection::<User>("user");
    // Update doc
    // Check request paramter
    let update_doc = if req_parse.role.to_string() == "admin".to_string() {
        // if role is admin, it will update admin as request status
        doc! {"$set" : {
            "admin" : if req_parse.status {
                1
            } else {
                0
            }
        }}
    } else if req_parse.role.to_string() == "judge".to_string() {
        // if role is judge, it will update judge as request status
        doc! {"$set" : {
            "judge" : if req_parse.status {
                1
            } else {
                0
            }
        }}
    } else if req_parse.role.to_string() == "battler".to_string() {
        // if role is battler, it will update battler as request status
        doc! {"$set" : {
           "battler" : if req_parse.status {
                1
            } else {
                0
            }
        }}
    } else if req_parse.role.to_string() == "voter".to_string() {
        // if role is voter, it will update voter as request status
        doc! {"$set" : {
           "voter" : if req_parse.status {
                1
            } else {
                0
            }
        }}
    } else {
        // if role is bad, it will return bad status
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: format!("Bad Request"),
            }),
        ));
    };
    // update user status as registered
    match user_collection
        .find_one_and_update(
            doc! {"username" : req_parse.username.to_string(), "account_status" : "registered"},
            update_doc,
            None,
        )
        .await
    {
        // if user doesn't exist, return error
        Ok(None) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: format!("User doesn't exist"),
                }),
            ));
        }
        // if user exist and update success, return success
        Ok(Some(_)) => Ok(Status::Ok),
        Err(e) => {
            log::error!("When admin setup user status, Error :  {}", e.to_string());
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: format!("Server Error"),
                }),
            ));
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserReq {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserRes {
    pub exist: bool,
    pub admin: bool,
    pub voter: bool,
    pub battler: bool,
    pub judge: bool,
}

/**
 * Router for admin get user information
 */
#[post("/admin-get-user-info", format = "json", data = "<req_data>")]
pub async fn admin_get_user_info(
    database: &State<Database>,
    req_data: Option<Json<GetUserReq>>,
    admin: AuthorizedAdmin,
) -> Result<(Status, Json<GetUserRes>), (Status, Json<ErrorRes>)> {
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
    // get the user collection
    let user_collection = database.collection::<User>("user");
    // get information
    match user_collection
        .find_one(doc! {"username" : req_parse.username.to_string()}, None)
        .await
    {
        Ok(None) => {
            return Ok((
                Status::Ok,
                Json(GetUserRes {
                    exist: false,
                    voter: false,
                    admin: false,
                    battler: false,
                    judge: false,
                }),
            ));
        }
        Ok(Some(val)) => {
            return Ok((
                Status::Ok,
                Json(GetUserRes {
                    exist: false,
                    voter: if val.voter ==  1 {
                        true
                    } else {
                        false
                    },
                    admin: if val.admin == 1  {
                        true
                    } else {
                        false
                    },
                    battler: if val.battler  == 1{
                        true
                    } else {
                        false
                    },
                    judge: if val.judge == 1 {
                        true
                    } else {
                        false
                    },
                }),
            ))
        }
        Err(_) => {
            return Ok((
                Status::Ok,
                Json(GetUserRes {
                    exist: false,
                    voter: false,
                    admin: false,
                    battler: false,
                    judge: false,
                }),
            ))
        }
    }
}

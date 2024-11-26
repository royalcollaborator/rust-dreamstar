use crate::config::{google_img_bucket, google_key_json};
use crate::db_models::matches::Match;
use crate::db_models::user::User;
use crate::db_models::votes::Vote;
use crate::middleware::{recaptcha_verify::Recaptcha, verify_token::AuthorizedUser};
use crate::models::error_response::ErrorRes;
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
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSignImgUrlReqModel {
    pub match_id: String,
    pub a_camp_votes: i32,
    pub b_camp_votes: i32,
    pub vote_type: i32,
    pub statement: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSignURLResModel {
    pub img_id: String,
    pub img_sign_url: String,
}
/**
 * Check voting information's  validation and send the  image uuid and get the cloud sign urls and send it into client
 */
#[post("/get-sign-img-url", format = "json", data = "<req_data>")]
pub async fn get_sign_img_url(
    auth_user: AuthorizedUser,
    recaptcha: Recaptcha,
    database: &State<Database>,
    req_data: Option<Json<GetSignImgUrlReqModel>>,
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

    log::info!("Get Sign image request accepted with recaptcha and token verify");

    let image_file_id = Uuid::new_v4().to_string();
    let img_bucket_name = google_img_bucket();
    let auth_user_id = ObjectId::parse_str(auth_user.user_id).unwrap();
    let user_collection = database.collection::<User>("user");
    let match_collection = database.collection::<Match>("match");
    let vote_collection = database.collection::<Vote>("vote");
    // get a camp username
    let auth_user_name = match user_collection
        .find_one(doc! { "_id" : auth_user_id }, None)
        .await
    {
        Ok(None) => "".to_string(),
        Ok(Some(sel_user)) => sel_user.username,
        Err(_) => "".to_string(),
    };
    // Check auth user exist
    if auth_user_name.is_empty() {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "You are not battler".to_string(),
            }),
        ));
    }
    // Get match information using match_id
    let match_info = match match_collection
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
        Ok(Some(sel_data)) => sel_data,
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

    // Check voting is expired
    if match_info.closed && req_parse.vote_type != 0 {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "Voting ended, Only available unofficial voting.".to_string(),
            }),
        ));
    }
    // Check judge
    let judge_list = match_info.judges;
    if !judge_list
        .iter()
        .any(|e| e.to_string() == auth_user_name.to_string())
        && req_parse.vote_type == 2
    {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "You are not judge.".to_string(),
            }),
        ));
    }
    // Check this user already did voting
    match
        vote_collection.find_one(
            doc! { "match_id" : req_parse.match_id.to_string(), "voter_name" : auth_user_name.to_string() },
            None
        ).await
    {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "You 've already done voting for this battle.".to_string(),
                }),
            ));
        }
        Err(e) => {
            log::error!(
                "In voting get sign url, try to check double voting, Error : {}",
                e.to_string()
            );
            return Err((
                Status::InternalServerError,
                Json(ErrorRes { cause: "Server Error".to_string() }),
            ));
        }
    }
    // Check user is not camp
    if auth_user_name.to_string() == match_info.a_camp_username
        || auth_user_name.to_string() == match_info.b_camp_username
    {
        return Err((
            Status::BadRequest,
            Json(ErrorRes {
                cause: "You are camp for this battle.".to_string(),
            }),
        ));
    }
    //
    // Get google credential from config
    match CredentialsFile::new_from_str(google_key_json().as_str()).await {
        // If exist
        Ok(cre) => {
            // Make google config  for new google client
            match ClientConfig::default().with_credentials(cre).await {
                Ok(config_client) => {
                    let client = Client::new(config_client);
                    let expires = std::time::Duration::from_secs(90 * 60); // 15 minutes
                                                                           // Create Signed URL for image upload
                    let img_url = client
                        .signed_url(
                            img_bucket_name.as_str(),
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

                    return Ok((
                        Status::Ok,
                        Json(GetSignURLResModel {
                            img_id: image_file_id.to_string(),
                            img_sign_url: img_url.to_string(),
                        }),
                    ));
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
pub struct SetVoteReqModel {
    pub match_id: String,
    pub a_camp_votes: i32,
    pub b_camp_votes: i32,
    pub vote_type: i32,
    pub statement: String,
    pub img_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetVoteResModel {}

#[post("/set-vote", format = "json", data = "<req_data>")]
pub async fn set_vote(
    auth_user: AuthorizedUser,
    recaptcha: Recaptcha,
    database: &State<Database>,
    req_data: Option<Json<SetVoteReqModel>>,
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
    // if it is invalid, return bad request data
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
    let auth_user_id = ObjectId::parse_str(auth_user.user_id).unwrap();
    log::info!("Set vote request accepted with recaptcha and token verify");
    let vote_collection = database.collection::<Vote>("vote");
    let user_collection = database.collection::<User>("user");
    // get a camp username
    let auth_user_name = match user_collection
        .find_one(doc! { "_id" : auth_user_id }, None)
        .await
    {
        Ok(None) => "".to_string(),
        Ok(Some(sel_user)) => sel_user.username,
        Err(_) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "You are not allowed.".to_string(),
                }),
            ));
        }
    };
    let new_vote = Vote {
        match_id: req_parse.match_id.to_string(),
        voter_name: auth_user_name.to_string(),
        voter_youtube_channel_name: Default::default(),
        voter_youtube_channel_id: Default::default(),
        voter_instagram_name: Default::default(),
        voter_twitter_name: Default::default(),
        voter_twitter: Default::default(),
        timestamp: Utc::now().timestamp(),
        a_camp_votes: req_parse.a_camp_votes,
        b_camp_votes: req_parse.b_camp_votes,
        statement: req_parse.statement,
        vote_type: req_parse.vote_type,
        thumbnail: Default::default(),
        bitcoin_transaction_id: Default::default(),
        satoshi_amount: Default::default(),
        dollar_amount: Default::default(),
        signature_img_file_id: req_parse.img_id,
    };

    match vote_collection.insert_one(new_vote, None).await {
        Ok(_) => {
            return Ok(Status::Ok);
        }
        Err(e) => {
            log::error!("When set voting, Error  : {}", e.to_string());
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error.".to_string(),
                }),
            ));
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVoteListReqModel {
    pub match_id: String,
    pub search: String,
    pub count: i32,
    pub pagination: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVoteListResModel {
    pub data: Vec<Vote>,
    pub max_page: i32,
    pub battle_info: BattleScore,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BattleScore {
    pub winner_name: String,
    pub loser_name: String,
    pub winner_final_vote: i32,
    pub loser_final_vote: i32,
    pub winner_official_vote: i32,
    pub loser_official_vote: i32,
    pub winner_judge_vote: i32,
    pub loser_judge_vote: i32,
}

/**
 * Router for get the voting list using match_id
 */
#[post("/voting-list", format = "json", data = "<req_data>")]
pub async fn get_voting_list(
    database: &State<Database>,
    req_data: Option<Json<GetVoteListReqModel>>,
) -> Result<(Status, Json<GetVoteListResModel>), (Status, Json<ErrorRes>)> {
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
    // Get the vote collection
    let vote_collection = database.collection::<Vote>("vote");
    let match_collection = database.collection::<Match>("match");
    log::info!("get voting list request accepted with verify token");
    // Check this request is valid and also get the battle score
    let battle_info = match match_collection
        .find_one(doc! { "match_id" : req_parse.match_id.to_string() }, None)
        .await
    {
        Ok(None) => {
            return Err((
                Status::BadRequest,
                Json(ErrorRes {
                    cause: "Battle doesn't exist ".to_string(),
                }),
            ));
        }
        Ok(Some(res)) => {
            if res.a_camp_final_vote_count > res.b_camp_final_vote_count {
                BattleScore {
                    winner_name: res.a_camp_username,
                    loser_name: res.b_camp_username,
                    winner_final_vote: res.a_camp_final_vote_count,
                    loser_final_vote: res.b_camp_final_vote_count,
                    winner_official_vote: res.a_camp_vote_count,
                    loser_official_vote: res.b_camp_vote_count,
                    winner_judge_vote: res.a_camp_judge_vote_count,
                    loser_judge_vote: res.b_camp_judge_vote_count,
                }
            } else {
                BattleScore {
                    winner_name: res.b_camp_username,
                    loser_name: res.a_camp_username,
                    winner_final_vote: res.b_camp_final_vote_count,
                    loser_final_vote: res.a_camp_final_vote_count,
                    winner_official_vote: res.b_camp_vote_count,
                    loser_official_vote: res.a_camp_vote_count,
                    winner_judge_vote: res.b_camp_judge_vote_count,
                    loser_judge_vote: res.a_camp_judge_vote_count,
                }
            }
        }
        Err(e) => {
            log::error!(
                "When check match_id valid and also get the info, Error : {}",
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
    // Make filter condition base on search
    let vote_filter = if req_parse.search.is_empty() {
        doc! { "match_id" : req_parse.match_id.to_string() }
    } else {
        doc! { "voter" : {"$regex": format!(".*{}.*", req_parse.search.to_string()), "$options": "i" }, "match_id" : req_parse.match_id.to_string() }
    };
    // Make custom options base on request parameter
    let voting_option = FindOptions::builder()
        .skip(Some((req_parse.count * (req_parse.pagination - 1)) as u64))
        .limit(Some(req_parse.count as i64))
        .build();
    // Get the total count
    let total_count = match vote_collection
        .count_documents(vote_filter.clone(), None)
        .await
    {
        Ok(con) => con as i32,
        Err(e) => {
            log::error!("When count data, Error : {}", e.to_string());
            return Err((
                Status::InternalServerError,
                Json(ErrorRes {
                    cause: "Server Error ".to_string(),
                }),
            ));
        }
    };
    // Calculation max page
    let max_page = if total_count % req_parse.count > 0 {
        total_count / req_parse.count + 1
    } else {
        total_count / req_parse.count
    };
    // Get the vote_list base on conditions
    let vote_list = match vote_collection.find(vote_filter, voting_option).await {
        Ok(res) => match res.try_collect::<Vec<Vote>>().await {
            Ok(result) => result
                .into_iter()
                .map(|vote| Vote {
                    match_id: vote.match_id,
                    voter_name: vote.voter_name,
                    voter_youtube_channel_id: vote.voter_youtube_channel_id,
                    voter_youtube_channel_name: vote.voter_youtube_channel_name,
                    voter_instagram_name: vote.voter_instagram_name,
                    voter_twitter: vote.voter_twitter,
                    voter_twitter_name: vote.voter_twitter_name,
                    timestamp: vote.timestamp,
                    a_camp_votes: vote.a_camp_votes,
                    b_camp_votes: vote.b_camp_votes,
                    statement: vote.statement,
                    vote_type: vote.vote_type,
                    thumbnail: vote.thumbnail,
                    bitcoin_transaction_id: vote.bitcoin_transaction_id,
                    satoshi_amount: vote.satoshi_amount,
                    dollar_amount: vote.dollar_amount,
                    signature_img_file_id: format!(
                        "https://storage.googleapis.com/{}/images/{}.jpeg",
                        google_img_bucket(),
                        vote.signature_img_file_id.to_string()
                    ),
                })
                .collect(),
            Err(e) => {
                log::error!("When try_collect in vote list, Error : {}", e.to_string());
                return Err((
                    Status::InternalServerError,
                    Json(ErrorRes {
                        cause: "Server Error ".to_string(),
                    }),
                ));
            }
        },
        Err(e) => {
            log::error!(
                "get the vote list in vote collection, Error : {}",
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
    Ok((
        Status::Ok,
        Json(GetVoteListResModel {
            data: vote_list,
            max_page: max_page,
            battle_info: battle_info,
        }),
    ))
}

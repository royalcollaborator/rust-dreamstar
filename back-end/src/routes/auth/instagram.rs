use crate::config::{instagram_client_id, instagram_client_secret, instagram_redirect_url};
use crate::db_models::user::User;
use crate::middleware::{recaptcha_verify::Recaptcha, verify_token::AuthorizedUser};
use crate::utils::util::encode_jwt;
use chrono::Utc;
use log;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Database};
use reqwest::Client;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;
// Get instagram auth url
#[get("/instagram")]
pub fn instagram_login(recaptcha: Recaptcha) -> Json<Value> {
    // Check recaptcha
    if !recaptcha.recaptcha_result {
        return Json(json!({"status": "error", "message": "ReCAPTCHA Error"}));
    }
    let auth_url = format!(
        "https://api.instagram.com/oauth/authorize?client_id={}&redirect_uri={}&scope=user_profile,user_media&response_type=code",
        instagram_client_id(),
        instagram_redirect_url()
    );
    log::info!("Instagram login request accept with recaptcha verify");
    Json(json!({"url": auth_url.to_string()}))
}

// Router for signup via instagram
#[get("/instagram/callback?<code>")]
pub async fn instagram_callback(code: String, database: &State<Database>) -> Json<Value> {
    let client_id = instagram_client_id(); // Changed to let binding
    let client_secret = instagram_client_secret(); // Changed to let binding
    let redirect_uri = instagram_redirect_url(); // Changed to let binding

    let client = Client::new();
    log::info!("Instagram request callback request accepted");
    // log::debug!(format!("Instagram access code  : {}", code.to_string()));
    // Exchange code for access token
    let res = client
        .post("https://api.instagram.com/oauth/access_token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri.as_str()),
            ("code", &code),
        ])
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = res.json().await.unwrap();

    // Fetch user info
    let access_token = json["access_token"].as_str().unwrap();
    // log::debug!(format!(
    //     "Instagram access token : {}",
    //     String::from(&access_token)
    // ));
    let user_info_res = client
        .get(format!(
            "https://graph.instagram.com/me?fields=id,username&access_token={}",
            access_token
        ))
        .send()
        .await
        .unwrap();

    let user_info: serde_json::Value = user_info_res.json().await.unwrap();
    let instagram_user_id = user_info["id"].as_str().unwrap().to_string();
    let instagram_username = user_info["username"].as_str().unwrap().to_string();
    // log::debug!(format!(
    //     "Instagram UserID: {}",
    //     instagram_user_id.to_string()
    // ));
    // log::debug!(format!(
    //     "Instagram UserName : {}",
    //     instagram_username.to_string()
    // ));
    let collection = database.collection::<User>("user");
    match collection
        .find_one(
            doc! { "instagram_id" : instagram_user_id.to_string() },
            None,
        )
        .await
    {
        Ok(None) => {
            // We don't need to make it as function.
            // Because ObjectId::new() function already return ObjectId and we can use it using clone .
            // Please let me know your thoughts.
            let new_register_id = ObjectId::new();
            let requestor = User {
                _id: new_register_id.clone(),
                username: instagram_username.to_string(),
                email: Default::default(),
                temp_email: Default::default(),
                password: Default::default(),
                registration_timestamp: Utc::now().timestamp(),
                invitation_code: Default::default(),
                account_status: "registered".to_string(),
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
                instagram_id: instagram_user_id.to_string(),
                instagram_name: instagram_username.to_string(),
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
            match collection.insert_one(requestor.clone(), None).await {
                Ok(_) => match encode_jwt(new_register_id.clone()) {
                    Ok(token) => Json(json!({"status": "success", "token": token})),
                    Err(_) => Json(json!({"status": "error", "message": "Can't create token"})),
                },
                Err(_) => Json(json!({"status": "error", "token": "Can't interact with database"})),
            }
        }
        Ok(Some(sel_user)) => {
            if sel_user.instagram_id == instagram_user_id.to_string() {
                match encode_jwt(sel_user._id) {
                    Ok(token) => Json(json!({"status": "success", "token": token})),
                    Err(_) => Json(json!({"status": "error", "message": "Can't create token"})),
                }
            } else {
                Json(json!({"status": "error", "token": "Same email already registered"}))
            }
        }
        Err(_) => Json(json!({"status": "error", "token": "Can't interact with database"})),
    }
}

// Router for enable instagram login
#[put("/instagram/callback?<code>")]
pub async fn enable_instagram_callback(
    code: String,
    database: &State<Database>,
    auth_user: AuthorizedUser,
) -> Json<Value> {
    let client_id = instagram_client_id(); // Changed to let binding
    let client_secret = instagram_client_secret(); // Changed to let binding
    let redirect_uri = instagram_redirect_url(); // Changed to let binding
    log::info!("Instagram enable request callback request accepted");
    let client = Client::new();
    let auth_user_id = auth_user.user_id;
    // Exchange code for access token
    let res = client
        .post("https://api.instagram.com/oauth/access_token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri.as_str()),
            ("code", &code),
        ])
        .send()
        .await
        .unwrap();

    let json: serde_json::Value = res.json().await.unwrap();

    // Fetch user info
    let access_token = json["access_token"].as_str().unwrap();
    let user_info_res = client
        .get(format!(
            "https://graph.instagram.com/me?fields=id,username&access_token={}",
            access_token
        ))
        .send()
        .await
        .unwrap();

    let user_info: serde_json::Value = user_info_res.json().await.unwrap();
    let instagram_user_id = user_info["id"].as_str().unwrap().to_string();
    let instagram_username = user_info["username"].as_str().unwrap().to_string();
    let collection = database.collection::<User>("user");

    // Check Instagram id exist
    match collection
        .find_one(
            doc! { "instagram_id" : instagram_user_id.to_string() },
            None,
        )
        .await
    {
        Ok(None) => match ObjectId::parse_str(auth_user_id) {
            Ok(id) => {
                let update_doc = doc! {
                    "$set" : {
                        "instagram_id" : instagram_user_id.to_string(),
                        "instagram_username" : instagram_username.to_string(),
                    }
                };
                match collection
                    .update_one(doc! { "_id" : id }, update_doc, None)
                    .await
                {
                    Ok(_) => match encode_jwt(id) {
                        Ok(token) => Json(json!({"status": "success", "token": token})),
                        Err(_) => Json(json!({"status": "error", "message": "Can't create token"})),
                    },
                    Err(_) => {
                        Json(json!({"status": "error", "message": "Database connection error"}))
                    }
                }
            }
            Err(_) => Json(json!({"status": "error", "message": "Failed to parse auth user id"})),
        },
        Ok(_) => Json(json!({"status": "error", "message": "Instagram account already used"})),
        Err(_) => Json(json!({"status": "error", "message": "Server Error"})),
    }
}

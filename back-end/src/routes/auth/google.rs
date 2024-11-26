use crate::config::{
    google_auth_url, google_client_id, google_client_secret, google_redirect_url, google_token_url,
};
use crate::db_models::user::User;
use crate::middleware::{recaptcha_verify::Recaptcha, verify_token::AuthorizedUser};
use crate::utils::util::encode_jwt;
use crate::OAuth2Client;
use chrono::Utc;
use log;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, Database};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

// Get google auth url
#[post("/google")]
pub fn google_login(recaptcha: Recaptcha) -> Json<Value> {
    // Check recaptcha
    if !recaptcha.recaptcha_result {
        return Json(json!({"status": "error", "message": "ReCAPTCHA Error"}));
    }
    let client = BasicClient::new(
        ClientId::new(google_client_id().to_string()),
        Some(ClientSecret::new(google_client_secret().to_string())),
        AuthUrl::new(google_auth_url().to_string()).expect("Invalid authorization URL"),
        Some(TokenUrl::new(google_token_url().to_string()).expect("Invalid token URL")),
    )
    .set_redirect_uri(
        RedirectUrl::new(google_redirect_url().to_string()).expect("Invalid redirect URL"),
    );

    let authorization_request = client
        .authorize_url(|| CsrfToken::new_random())
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()));

    let (auth_url, _csrf_token) = authorization_request.url();
    log::info!("Google login request accept with recaptcha verify");
    Json(json!({"url": auth_url.to_string()}))
}

// Router for signup via google
#[get("/google/callback?<code>")]
pub async fn google_callback(
    code: String,
    oauth_client: &State<OAuth2Client>,
    database: &State<Database>,
) -> Json<Value> {
    let token_result = oauth_client
        .client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await;
    log::info!("Google login callback request accepted");

    // Check for token returned from google
    match token_result {
        // IF token exist
        Ok(token) => {
            let access_token = token.access_token().secret();
            let user_info_response = reqwest::Client::new()
                .get("https://www.googleapis.com/oauth2/v1/userinfo")
                .bearer_auth(access_token)
                .send()
                .await;
            // Get Response from google using token
            match user_info_response {
                // Response exist
                Ok(response) => {
                    if response.status().is_success() {
                        let user_info: Result<Value, _> = response.json().await;
                        // Get user information from response
                        match user_info {
                            // If user information exist
                            Ok(user_info) => {
                                let user_id =
                                    user_info["id"].as_str().unwrap_or_default().to_string();
                                let collection = database.collection::<User>("user");
                                let google_id = user_id;
                                let user_email = user_info["email"].as_str().unwrap_or_default();
                                let user_name = user_info["name"].as_str().unwrap_or_default();
                                // Check google id and username already exist or else
                                match collection
                                    .find_one(
                                        doc! { "google_email" : user_email.to_string() },
                                        None,
                                    )
                                    .await
                                {
                                    // Same google id and exist
                                    Ok(Some(sel_user)) => {
                                        if sel_user.google_id == google_id.to_string() {
                                            // Generate token and send to front-end
                                            match encode_jwt(sel_user._id) {
                                                Ok(token) => Json(
                                                    json!({"status": "success", "token": token}),
                                                ),
                                                Err(_) => Json(
                                                    json!({"status": "error", "message": "Can't create token"}),
                                                ),
                                            }
                                        } else {
                                            Json(
                                                json!({"status": "error", "token": "Same email already registered"}),
                                            )
                                        }
                                    }
                                    // Google id doesn't exist
                                    Ok(None) => {
                                        // Insert user and generate token and send to front-end
                                        let new_register_id = ObjectId::new();
                                        let requestor = User {
                                            _id: new_register_id.clone(),
                                            username: user_name.to_string(),
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
                                            instagram_id: Default::default(),
                                            instagram_name: Default::default(),
                                            instagram_state_code: Default::default(),
                                            instagram_thumbnail: Default::default(),
                                            google_id: google_id.to_string(),
                                            google_email: user_email.to_string(),
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
                                            // session_timestamp: dateTime.create().epoch(),
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
                                                Ok(token) => Json(
                                                    json!({"status": "success", "token": token}),
                                                ),
                                                Err(_) => Json(
                                                    json!({"status": "error", "message": "Can't create token"}),
                                                ),
                                            },
                                            Err(_) => Json(
                                                json!({"status": "error", "token": "Can't interact with database"}),
                                            ),
                                        }
                                    }
                                    // Error in Database interaction
                                    Err(_) => Json(
                                        json!({"status": "error", "token": "Can't interact with database"}),
                                    ),
                                }
                            }
                            // else user information doesn't exist
                            Err(_) => Json(
                                json!({"status": "error", "message": "Failed to parse user info"}),
                            ),
                        }
                    } else {
                        Json(json!({"status": "error", "message": "Failed to fetch user info"}))
                    }
                }
                // Response doesn't exist
                Err(_) => Json(
                    json!({"status": "error", "message": "Failed to connect to user info API"}),
                ),
            }
        }
        // IF token doesn't exist
        Err(_e) => {
            Json(json!({"status": "error", "message": "Failed to authenticate with Google"}))
        }
    }
}

// Router for enable via google
#[put("/google/callback?<code>")]
pub async fn enable_google_callback(
    code: String,
    oauth_client: &State<OAuth2Client>,
    database: &State<Database>,
    auth_user: AuthorizedUser,
) -> Json<Value> {
    let auth_user_id = auth_user.user_id;
    let token_result = oauth_client
        .client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await;
    // Get token from google using code
    match token_result {
        // If token exist
        Ok(token) => {
            let access_token = token.access_token().secret();
            let user_info_response = reqwest::Client::new()
                .get("https://www.googleapis.com/oauth2/v1/userinfo")
                .bearer_auth(access_token)
                .send()
                .await;
            log::info!("Google enable callback request accepted");
            // Get Response
            match user_info_response {
                // If Response doesn't exist
                Ok(response) => {
                    if response.status().is_success() {
                        let user_info: Result<Value, _> = response.json().await;
                        // Ger user information from response
                        match user_info {
                            // If user information exist
                            Ok(user_info) => {
                                let user_id =
                                    user_info["id"].as_str().unwrap_or_default().to_string();
                                let collection = database.collection::<User>("user");
                                let google_id = user_id;
                                let user_email = user_info["email"].as_str().unwrap_or_default();
                                match ObjectId::parse_str(auth_user_id) {
                                    Ok(id) => {
                                        let update_doc = doc! {
                                            "$set" : {
                                                "google_id" : google_id.to_string(),
                                                "google_email" : user_email.to_string(),
                                            }
                                        };
                                        // Check google id already exist
                                        match collection
                                            .find_one(
                                                doc! { "google_id" : google_id.to_string() },
                                                None,
                                            )
                                            .await
                                        {
                                            // same google id doesn't exist
                                            Ok(None) => {
                                                match collection
                                                    .update_one(
                                                        doc! { "_id" : id },
                                                        update_doc,
                                                        None,
                                                    )
                                                    .await
                                                {
                                                    // Generate token and send to client
                                                    Ok(_) => match encode_jwt(id) {
                                                        Ok(token) => Json(
                                                            json!({"status": "success", "token": token}),
                                                        ),
                                                        Err(_) => Json(
                                                            json!({"status": "error", "message": "Can't create token"}),
                                                        ),
                                                    },
                                                    Err(_) => Json(
                                                        json!({"status": "error", "message": "Database connection error"}),
                                                    ),
                                                }
                                            }
                                            // same google id exist
                                            Ok(Some(_)) => match encode_jwt(id) {
                                                Ok(token) => Json(
                                                    json!({"status": "success", "token": token}),
                                                ),
                                                Err(_) => Json(
                                                    json!({"status": "error", "message": "Can't create token"}),
                                                ),
                                            },
                                            // Error in database interaction
                                            Err(_) => Json(
                                                json!({"status": "error", "message": "Database connection error"}),
                                            ),
                                        }
                                    }
                                    Err(_) => Json(
                                        json!({"status": "error", "message": "Failed to parse auth user id"}),
                                    ),
                                }
                            }
                            // Else user information doesn't exist
                            Err(_) => Json(
                                json!({"status": "error", "message": "Failed to parse user info"}),
                            ),
                        }
                    } else {
                        Json(json!({"status": "error", "message": "Failed to fetch user info"}))
                    }
                }
                // Else Response doesn't exist
                Err(_) => Json(
                    json!({"status": "error", "message": "Failed to connect to user info API"}),
                ),
            }
        }
        // Else token doesn't exist
        Err(_e) => {
            Json(json!({"status": "error", "message": "Failed to authenticate with Google"}))
        }
    }
}

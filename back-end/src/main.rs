/**
 * @author      Andrii
 * @published   May 30, 2024
 * @description Library to config all variables used in the server
 * @email : fight0903@outlook.com, solomon21century@outlook.com, kunaievandrii@gmail.com
 */

#[macro_use]
extern crate rocket;
extern crate log;
extern crate log4rs;

extern crate dotenv;

use dotenv::dotenv;
use mongodb::bson::doc;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use tokio_cron_scheduler::{Job, JobScheduler};

use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

use crate::config::{
    google_auth_url, google_client_id, google_client_secret, google_redirect_url, google_token_url,
};
use crate::models::error_response::ErrorRes;
use crate::routes::admin::{
    battle::callout_setup, battle::get_battle_list, battle::reply_setup, user::admin_get_user_info,
    user::user_role_setup,
};
use crate::routes::auth::google::{google_callback, google_login};
use crate::routes::auth::{
    google::enable_google_callback,
    instagram::enable_instagram_callback,
    instagram::instagram_callback,
    instagram::instagram_login,
    invitation::{invitation_checked, reinvitation, test_get_invitation_code},
    login::auth_check,
    login::login,
    reset_password::reset_pass,
    reset_password::reset_pass_send_email,
    signup::signup,

};
use crate::routes::battle::battle_main::{show_battle_list, show_select_battle};
use crate::routes::battle::callout::{get_sign_url, get_user_list, set_callout};
use crate::routes::battle::live_battle::{live_battle_code_check, live_battle_setup};
use crate::routes::battle::live_battle_main::live_battle_show;
use crate::routes::battle::response::{get_response_user_list, get_sign_url_for_reply, set_reply};
use crate::routes::user::profile::{
    email_change_code, get_user_info, password_change, reset_email, username_change,
};
use crate::routes::vote::vote::{get_sign_img_url, get_voting_list, set_vote};
use crate::utils::{cron::voting_calc, live_battle_cron::live_battle_cron};

mod db;
mod db_models;
mod fairings;
mod middleware;
mod models;
mod utils;
mod routes {
    pub mod admin;
    pub mod auth;
    pub mod battle;
    pub mod user;
    pub mod vote;
}
mod config;
mod services;

#[get("/<file..>")]
async fn index(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("src/dist/index.html")).await.ok()
}

#[get("/static/<file..>")]
async fn file_server(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("src/dist").join(file)).await.ok()
}

pub struct OAuth2Client {
    client: BasicClient,
}

#[launch]
pub async fn rocket() -> _ {
    // init config variable
    dotenv().ok();
    //  DB clone
    let db = db::get_database().await;
    // Cors setting
    let cors = (CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    })
    .to_cors()
    .expect("Error while building CORS");

    // Initialize logging
    // Determine which configuration file to use based on an environment variable
    let port = match std::env::var("PORT").map(|v| v.parse::<u16>()) {
        Ok(Ok(port)) => port,
        _ => 8000, // default port or handle the error as you see fit
    };
    let env = std::env::var("ENV").unwrap_or_else(|_| "development".to_string()); // Add this line to define `env`
    let config_file = match env.as_str() {
        "production" => "log4rs_prod.yml",
        _ => "log4rs_dev.yml",
    };

    // Initialize log4rs with the chosen configuration file
    let _ = log4rs::init_file(config_file, Default::default());

    // Google Client setup
    let oauth_client = BasicClient::new(
        ClientId::new(google_client_id().to_string()),
        Some(ClientSecret::new(google_client_secret().to_string())),
        AuthUrl::new(google_auth_url().to_string()).expect("Invalid authorization URL"),
        Some(TokenUrl::new(google_token_url().to_string()).expect("Invalid token URL")),
    )
    .set_redirect_uri(
        RedirectUrl::new(google_redirect_url().to_string()).expect("Invalid redirect URL"),
    );
    // This section initializes the JobScheduler and schedules the voting_calc function to run every 5 minutes.
    // The JobScheduler is responsible for managing and executing scheduled jobs.
    // The voting_calc function is defined in the utils/cron.rs file and performs the voting calculation.
    let sched = JobScheduler::new().await.unwrap();

    // Schedule the voting_calc function to run every 5 seconds
    let job = Job::new_async("0/5 * * * * *", move |_id, _lock| {
        Box::pin(async move {
            voting_calc().await;
        })
    })
    .unwrap();
    // Schedule the live_battle function to run every 5 seconds
    let live_battle_job = Job::new_async("0/5 * * * * *", move |_id, _lock| {
        Box::pin(async move {
            live_battle_cron().await;
        })
    })
    .unwrap();
    // Add the job to the scheduler and start it
    sched.add(job).await.unwrap();
    sched.add(live_battle_job).await.unwrap();
    sched.start().await.unwrap();

    // This section of the code is responsible for building and configuring the Rocket web server.
    // It attaches various fairings, manages the database and OAuth2 client, and mounts routes for different API endpoints.
    // The Rocket instance is built and configured with CORS settings, logging, and scheduled jobs.
    rocket::build()
        .configure(rocket::Config {
            port,
            address: "0.0.0.0".parse().unwrap(),
            ..rocket::Config::default()
        })
        .attach(cors)
        .attach(fairings::DbFairing)
        .attach(AdHoc::on_liftoff("Logger", |_| {
            Box::pin(async move {
                log::info!("Rocket is launching!");
            })
        }))
        .manage(db)
        .manage(OAuth2Client {
            client: oauth_client,
        })
        .mount("/", routes![file_server, index])
        .mount(
            "/api/v0/auth/",
            routes![
                login,
                signup,
                invitation_checked,
                test_get_invitation_code,
                reinvitation,
                google_login,
                google_callback,
                enable_google_callback,
                instagram_login,
                instagram_callback,
                enable_instagram_callback,
                reset_pass_send_email,
                reset_pass,
                auth_check
            ],
        )
        .mount(
            "/api/v0/user",
            routes![
                email_change_code,
                reset_email,
                password_change,
                username_change,
                get_user_info
            ],
        )
        .mount(
            "/api/v0/battle/live-battle",
            routes![live_battle_setup, live_battle_code_check, live_battle_show],
        )
        .mount(
            "/api/v0/battle/callout",
            routes![get_user_list, get_sign_url, set_callout],
        )
        .mount(
            "/api/v0/battle/response",
            routes![get_response_user_list, get_sign_url_for_reply, set_reply],
        )
        .mount(
            "/api/v0/battle/battle-main",
            routes![show_battle_list, show_select_battle],
        )
        .mount(
            "/api/v0/vote/",
            routes![get_sign_img_url, set_vote, get_voting_list],
        )
        .mount(
            "/admin/api/v0/battle/",
            routes![callout_setup, reply_setup, get_battle_list],
        )
        .mount(
            "/admin/api/v0/user/",
            routes![user_role_setup, admin_get_user_info],
        )
        .register(
            "/",
            catchers![unauthorized, not_found, internal_sever_error, forbidden],
        )
}

/**
 * Error handler
 */
#[catch(401)]
pub fn unauthorized() -> Json<ErrorRes> {
    Json(ErrorRes {
        cause: "Unauthorized".to_string(),
    })
}

/**
 * Error handler
 */
#[catch(404)]
pub fn not_found() -> Json<ErrorRes> {
    Json(ErrorRes {
        cause: "Not Found".to_string(),
    })
}

/**
 * Error handler
 */
#[catch(403)]
pub fn forbidden() -> Json<ErrorRes> {
    Json(ErrorRes {
        cause: "ReCaptcha Error, Please try again if you are not bot".to_string(),
    })
}

/**
 * Error handler
 */
#[catch(500)]
pub fn internal_sever_error() -> Json<ErrorRes> {
    Json(ErrorRes {
        cause: "Wrong request".to_string(),
    })
}

#[cfg(test)]
mod tests {
    mod routes {
        mod admin;
        mod auth;
        mod battle;
        mod middleware;
        mod user;
        mod vote;
    }
    mod utils;
}

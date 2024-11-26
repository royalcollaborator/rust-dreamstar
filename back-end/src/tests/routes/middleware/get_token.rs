use crate::config::test_recaptcha_key;
use crate::rocket;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;
use serde_json::Value;

/*
 * reset password setup function
 * In previous router, we already sent opt
 * To reset password, second we have to send these parameters
 * email : user's email
 * password : user's password
 * code : this is opt code (To test this function, you have to get the opt from database)
 * if you run this test function, user will get the opt into you email.. In test, you can get the opt from database
 */

pub async fn get_token(user: String, password: String) -> String {
    let rocket = rocket().await;
    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");

    let data = json!({
        "email" : user.to_string(),
        "password" : password.to_string()
    });
    // Make a POST request to the `/api/v0/auth/login` route
    let response = client
        .post("/api/v0/auth/login")
        .header(ContentType::JSON)
        .header(Header::new(
            "RecaptchaKey",
            test_recaptcha_key().to_string(),
        ))
        .header(Header::new("recaptchaName", "TEST".to_string()))
        .body(serde_json::to_string(&data).unwrap())
        .dispatch()
        .await;
    if response.status() == Status::Ok {
        let body = response.into_string().await.expect("response body");
        // Parse the response body as JSON
        let body_json: Value = serde_json::from_str(&body).expect("valid JSON response");
        // Extract the token from the JSON response
        let token = match body_json.get("token") {
            None => "".to_string(),
            Some(val) => val.to_string(),
        };

        token
    } else {
        "".to_string()
    }
}

use crate::config::test_recaptcha_key;
use crate::rocket;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;
use serde_json::Value;

/**
 * Test for login success
 */
#[rocket::async_test]
async fn login() {
    // Build the Rocket instance
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * login router (user will login using these parameters)
     * email : user' email
     * password : user's password
     * if you run this test function, you will get the token.
     */
    let data = json!({
        "email" : "solomon21century@outlook.com",
        "password" : "123qweasdASDQWE!@#"
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

    // Assert that the response status is OK
    assert_eq!(response.status(), Status::Ok);

    // Get the response body as a string
    let body = response.into_string().await.expect("response body");

    // Parse the response body as JSON
    let body_json: Value = serde_json::from_str(&body).expect("valid JSON response");

    // Extract the token from the JSON response
    let _ = body_json
        .get("token")
        .expect("token field exists")
        .as_str()
        .expect("token is a string");
}

/**
 * Test for auth check
 */
#[rocket::async_test]
async fn auth_check() {
    // Build the Rocket instance
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * auth check router (if user send request with token,
     * back-end will verify this token is correct and token is not expired)
     * token : user's token (it is jwt token)
     * it will return bool type 's result
     */
    let data = json!({
        "token" : "your token"
    });

    // Make a POST request to the `/api/v0/auth/login` route
    let response = client
        .post("/api/v0/auth/auth-check")
        .header(ContentType::JSON)
        .header(Header::new(
            "RecaptchaKey",
            test_recaptcha_key().to_string(),
        ))
        .header(Header::new("recaptchaName", "TEST".to_string()))
        .body(serde_json::to_string(&data).unwrap())
        .dispatch()
        .await;

    // Assert that the response status is OK
    assert_eq!(response.status(), Status::Ok);
}

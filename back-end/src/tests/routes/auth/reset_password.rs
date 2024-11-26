use crate::config::test_recaptcha_key;
use crate::rocket;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

/**
 * Test for reset password email send
 */
#[rocket::async_test]
async fn reset_password_send_email() {
    // Build the Rocket instance
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * Reset password send email router
     * parameters :
     * email : user's email
     * if run this test, this email user will be get the opt via email
     */
    let data = json!({
        "email" : "",  // Your email
    });

    let response = client
        .post("/api/v0/auth/reset-pass-send-email")
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

/**
 * Test for reset password set
 */
#[rocket::async_test]
async fn reset_password() {
    // Build the Rocket instance
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * Reset password router
     * parameters :
     * email : user's email
     * password : user's new password
     * code : it is opt code that received via email
     * if run this test, password will be changed
     */
    let data = json!({
        "email" : "",  // Your email
        "password" : "",  // Your new password
        "code" : ""        // Your code
    });

    let response = client
        .post("/api/v0/auth/reset-pass")
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

use crate::config::test_recaptcha_key;
use crate::rocket;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

/**
 * Test for invitation code success
 */
#[rocket::async_test]
async fn invitation() {
    // Build the Rocket instance
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * confirm invitation router test (after signup, user must verify email... )
     *  these parameters needed
     * email : user' email
     * code : (otp when signup, user will get the opt)
     * if you run this test function, confirm email, it means user account status will be changed as register.
     */
    let data = json!({
        "email" : "test111@gmail.com", // please set parameters
        "code" : "19242"  // please set parameters
    });

    // Make a POST request to the `/api/v0/auth/login` route
    let response = client
        .post("/api/v0/auth/invitation")
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
 * Test for resend invitation code success
 */
#[rocket::async_test]
async fn resend_invitation() {
    // Build the Rocket instance
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * this test will send invitation code into user's email
     *  these parameters needed
     * email : user' email
     * if you run this test function, user will receive invitation code. (you can check it via database)
     */
    let data = json!({
        "email" : "test11@gmail.com", // Please set parameter
    });

    // Make a POST request to the `/api/v0/auth/login` route
    let response = client
        .post("/api/v0/auth/invitation/resend")
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

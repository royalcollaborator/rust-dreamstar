use crate::config::test_recaptcha_key;
use crate::rocket;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

/**
 * Test for signup router
 */
#[rocket::async_test]
async fn signup() {
    // Build the Rocket instance
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * if you run this test function,
     * it will signup user using this parameter
     * email : user' email
     * password : user's password
     * username : user's username
     * if you run this test function,
     * you can see that one user added in mongodb with this parameters
     */
    let data = json!({
        "email" : "test1@gmail.com",
        "password" : "123qweasdASDQWE!@#",
        "username" : "test1"
    });

    // Make a POST request to the `/api/v0/auth/login` route
    let response = client
        .post("/api/v0/auth/signup")
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

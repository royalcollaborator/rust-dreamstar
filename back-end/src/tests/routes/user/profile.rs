use crate::config::test_recaptcha_key;
use crate::rocket;
use crate::tests::routes::middleware::get_token::get_token;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

/**
 * Test for email change.
 */
#[rocket::async_test]
async fn change_email() {
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * First, we have to get the auth token to verify user
     * user : user's name or email
     * password : user's password
     * Get_token function will return auth token
     */
    let token = get_token(
        "solomon21century@outlook.com".to_string(),
        "123qweasdASDQWE!@#".to_string(),
    )
    .await;
    // Token check
    /*
     * If token is empty, it means auth is failed
     * Here, check token
     */
    if token.is_empty() {
        assert!(false, "Token must not be empty");
    }
    {
        /*
         * In profile page
         * email : user's new email
         * code : otp code
         * If you run this test, email will be changed
         */
        let email_change_data = json!({
            "email" : "dancers@gmail.com"
        });
        let change_email_response = client
            .post("/api/v0/user/reset-email")
            .header(ContentType::JSON)
            .header(Header::new(
                "RecaptchaKey",
                test_recaptcha_key().to_string(),
            ))
            .header(Header::new("recaptchaName", "TEST".to_string()))
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", token.to_string()),
            ))
            .body(serde_json::to_string(&email_change_data).unwrap())
            .dispatch()
            .await;
        assert_eq!(change_email_response.status(), Status::Ok);
    }
}

/**
 * Test for email change invitation code
 * To change email, first user has to verify old email
 */
#[rocket::async_test]
async fn change_email_code() {
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");

    /*
     * First, we have to get the auth token to verify user
     * user : user's name or email
     * password : user's password
     */
    let token = get_token(
        "solomon21century@outlook.com".to_string(),
        "123qweasdASDQWE!@#".to_string(),
    )
    .await;
    // Token check
    /*
     * If token is empty, it means auth is failed
     * Here, check token
     */
    if token.is_empty() {
        assert!(false, "Token must not be empty");
    }
    {
        /*
         * In profile page
         * new_email : user new email
         * user information can get the from token in back-end
         * because token has user object id
         */
        let email_change_data = json!({
            "new_email" : "dancers@gmail.com"
        });
        let change_email_response = client
            .post("/api/v0/user/email-change-code")
            .header(ContentType::JSON)
            .header(Header::new(
                "RecaptchaKey",
                test_recaptcha_key().to_string(),
            ))
            .header(Header::new("recaptchaName", "TEST".to_string()))
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", token.to_string()),
            ))
            .body(serde_json::to_string(&email_change_data).unwrap())
            .dispatch()
            .await;
        assert_eq!(change_email_response.status(), Status::Ok);
    }
}

/**
 * Test for username change.
 */
#[rocket::async_test]
async fn change_username() {
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * First, we have to get the auth token to verify user
     * user : user's name or email
     * password : user's password
     * Get_token function will return auth token
     */
    let token = get_token(
        "solomon21century@outlook.com".to_string(),
        "123qweasdASDQWE!@#".to_string(),
    )
    .await;
    // Token check
    if token.is_empty() {
        assert!(false, "Token must not be empty");
    }
    {
        /*
         * First, In this router, also include token
         * so back-end can get the user information from token
         * because token include userID.
         * so back-end only need new_username
         * username : new_username
         * if you run this test, username will be changed
         */
        let change_username_data = json!({
            "username" : "new_username"
        });
        let change_username_response = client
            .post("/api/v0/user/usernameChange")
            .header(ContentType::JSON)
            .header(Header::new(
                "RecaptchaKey",
                test_recaptcha_key().to_string(),
            ))
            .header(Header::new("recaptchaName", "TEST".to_string()))
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", token.to_string()),
            ))
            .body(serde_json::to_string(&change_username_data).unwrap())
            .dispatch()
            .await;
        assert_eq!(change_username_response.status(), Status::Ok);
    }
}

/**
 * Test for password change.
 */
#[rocket::async_test]
async fn change_password() {
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");

    /*
     * First, we have to get the auth token to verify user
     * user : user's name or email
     * password : user's password
     * Get_token function will return auth token
     */
    let token = get_token(
        "solomon21century@outlook.com".to_string(),
        "123qweasdASDQWE!@#".to_string(),
    )
    .await;
    // Token check
    if token.is_empty() {
        assert!(false, "Failed auth");
    }
    {
        /*
         * First, In this router, also include token
         * so back-end can get the user information from token
         * because token include userID.
         * so back-end only need new_username
         * password : new_password
         * if you run this test, password will be changed
         */
        let change_password_data = json!({
            "password" : "123qweasdASDQWE!@#"
        });
        let change_password_response = client
            .post("/api/v0/user/passwordChange")
            .header(ContentType::JSON)
            .header(Header::new(
                "RecaptchaKey",
                test_recaptcha_key().to_string(),
            ))
            .header(Header::new("recaptchaName", "TEST".to_string()))
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", token.to_string()),
            ))
            .body(serde_json::to_string(&change_password_data).unwrap())
            .dispatch()
            .await;
        assert_eq!(change_password_response.status(), Status::Ok);
    }
}

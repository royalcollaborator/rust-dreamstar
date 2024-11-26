use crate::config::test_recaptcha_key;
use crate::rocket;
use crate::routes::admin::user::UserRoleChange;
use crate::tests::routes::middleware::get_token::get_token;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

/**
 * Test for user_role_setup
 */
#[rocket::async_test]
async fn user_role_setup() {
    // Build the Rocket instance
    let rocket = rocket().await;
    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");

    /*
     * First, we have to get the auth token to verify admin
     * user : admin's name or email
     * password : admin's password
     * Get_token function will return auth token
     */
    let token = get_token(
        "solomon21century@outlook.com".to_string(),
        "123qweasdASDQWE!@#".to_string(),
    )
    .await;
    // token check
    if token.is_empty() {
        assert!(false, "Failed auth")
    } else {
        /*
         * If user registered, user will be voter in our site
         * This router is for change user's role
         * username : user's username
         * role : it is role (admin, voter, battler, judge)
         * status : true or false (it means make role as 1 or 0)
         */
        let data = json!(UserRoleChange {
            username: "solomon".to_string(),
            role: "battler".to_string(),
            status: true
        });

        let response = client
            .post("/admin/api/v0/user/set-role")
            .header(ContentType::JSON)
            .header(Header::new(
                "RecaptchaKey",
                test_recaptcha_key().to_string(),
            ))
            .header(Header::new("recaptchaName", "TEST".to_string()))
            .header(Header::new("Authorization", token.to_string()))
            .body(serde_json::to_string(&data).unwrap())
            .dispatch()
            .await;

        // Assert that the response status is OK
        assert_eq!(response.status(), Status::Ok);
    }
}

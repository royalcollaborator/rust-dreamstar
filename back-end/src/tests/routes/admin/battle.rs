use crate::config::test_recaptcha_key;
use crate::rocket;
use crate::routes::admin::battle::GetSetUserReq;
use crate::tests::routes::middleware::get_token::get_token;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

/**
 * Test for a_camp callout setup verify
 */
#[rocket::async_test]
async fn callout_setup() {
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
         * After admin verify a_camp video, admin must set status a_callout status as true
         * match_id : battle's match id
         */
        let data = json!(GetSetUserReq {
            match_id: "match_id".to_string()
        });

        let response = client
            .post("/admin/api/v0/battle/callout-setup")
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

/**
 * Test for b_camp reply setup verify
 */
#[rocket::async_test]
async fn reply_setup() {
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
         * After admin verify b_camp video, admin must set status b_camp reply status as true
         * match_id : battle's match id
         */
        let data = json!(GetSetUserReq {
            match_id: "match_id".to_string()
        });

        let response = client
            .post("/admin/api/v0/battle/reply-setup")
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

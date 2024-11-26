use crate::config::test_recaptcha_key;
use crate::rocket;
use crate::routes::battle::response::{GetSignURLReqModel, GetSignURLResModel, SetReplyReqModel};
use crate::tests::routes::middleware::get_token::get_token;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;
use serde_json::Value;

/**
 * Test for sign url for b_camp reply
 */
#[rocket::async_test]
async fn get_sign_url_for_reply() {
    // Build the Rocket instance
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
    // token check
    if token.is_empty() {
        assert!(false, "Failed auth")
    } else {
        /*
         * To reply for a_camp, we have to pass 2 step
         * one step is to get the sign url and verify reply information
         * second step is to setup reply
         * this is for one step
         * a_camp_id : this is a_camp id
         * match_id : this is battle id
         * if you run this test, you will get the sign urls
         */
        let data = json!(GetSignURLReqModel {
            a_camp_id: "".to_string(),
            match_id: "".to_string(),
        });

        let response = client
            .post("/api/v0/battle/response/get-sign-url")
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
        let body = response.into_string().await.expect("response body");
        let body_json: Value = serde_json::from_str(&body).expect("valid JSON response");
        let _: GetSignURLResModel =
            serde_json::from_value(body_json).expect("valid GetSignURLResModel");
        assert!(true, "success");
    }
}

/**
 * Test for setup reply for b_camp
 */
#[rocket::async_test]
async fn set_reply() {
    // Build the Rocket instance
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
    // token check
    if token.is_empty() {
        assert!(false, "Failed auth")
    } else {
        /*
         * To reply for a_camp, we have to pass 2 step
         * one step is to get the sign url and verify reply information
         * second step is to setup reply
         * this is for second step
         * match_id : this is battle id
         * a_camp_id : this is a_camp id
         * video_id : this is b_camp video id
         * image_id : this is image id
         * video type : this is b_camp video type
         * responder_reply : this is b_camp saying
         * if you run this test, you will get the sign urls
         */
        let data = json!(SetReplyReqModel {
            match_id: "".to_string(),
            a_camp_id: "".to_string(),
            video_id: "".to_string(),
            image_id: "".to_string(),
            video_type: "".to_string(),
            responder_reply: "".to_string(),
        });

        let response = client
            .post("/api/v0/battle/response/set-reply")
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
 * Test for get all b_camp user list
 */
#[rocket::async_test]
async fn get_response_user_list() {
    // Build the Rocket instance
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
    // token check
    if token.is_empty() {
        assert!(false, "Failed auth")
    } else {
        /*
         * if user logged in,
         * user can see their reply list
         * if you run this test,
         * you will get the all response for this user
         */
        let response = client
            .post("/api/v0/battle/response/get-response-list")
            .header(ContentType::JSON)
            .header(Header::new(
                "RecaptchaKey",
                test_recaptcha_key().to_string(),
            ))
            .header(Header::new("recaptchaName", "TEST".to_string()))
            .header(Header::new("Authorization", token.to_string()))
            .dispatch()
            .await;
        // Assert that the response status is OK
        assert_eq!(response.status(), Status::Ok);
    }
}

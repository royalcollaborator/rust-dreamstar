use crate::config::test_recaptcha_key;
use crate::rocket;
use crate::routes::battle::callout::{
    GetSignURLReqModel, GetSignURLResModel, GetUserListResModel, SetCalloutReqModel,
};
use crate::tests::routes::middleware::get_token::get_token;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;
use serde_json::Value;
/**
 * Test for get callout user list
 */
#[rocket::async_test]
async fn get_user_list() {
    // Build the Rocket instance
    let rocket = rocket().await;

    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * Get battler list
     * this router is for get the all battler list
     * search : filter search string
     * count : it is number that show list
     * pagination : front-end pagination
     * token : user's token
     * if it run, return all battler list and max pages
     */
    let data = json!({
        "search" : "",
        "count" : 5,
        "pagination" : 1,
        "token" : ""
    });

    let response = client
        .post("/api/v0/battle/callout/get-user-list")
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
    let body = response.into_string().await.expect("response body");
    let body_json: Value = serde_json::from_str(&body).expect("valid JSON response");
    let _: GetUserListResModel =
        serde_json::from_value(body_json).expect("valid GetUserListResModel");
    assert!(true, "success");
}

/**
 * Test for sign url
 */
#[rocket::async_test]
async fn get_sign_url() {
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
         * To callout, we have to pass 2 step
         * one step is verify judge and also ge the google bucket sign url
         * second step is setting callout
         * this test is for first stop
         * a_1 : 1 judge
         * a_2 : 2 judge
         * a_3 : 3 judge
         * a_4 : 4 judge
         * a_5 : 5 judge
         * opponent : this is b_camp user id
         * if you run this, you will get the google bucket sign url
         */
        let data = json!(GetSignURLReqModel {
            a_1: "".to_string(),
            a_2: "".to_string(),
            a_3: "".to_string(),
            a_4: "".to_string(),
            a_5: "".to_string(),
            opponent: "".to_string(),
        });

        let response = client
            .post("/api/v0/battle/callout/verify-get-sign-url")
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
 * Test for callout setup
 */
#[rocket::async_test]
async fn call_setup() {
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
    // Check token
    if token.is_empty() {
        assert!(false, "Auth Failed")
    } else {
        /*
         * To callout, we have to pass 2 step
         * one step is verify judge and also ge the google bucket sign url
         * second step is setting callout
         * this test is for second stop
         * a_1 : 1 judge
         * a_2 : 2 judge
         * a_3 : 3 judge
         * a_4 : 4 judge
         * a_5 : 5 judge
         * opponent_id : this is b_camp user id
         * video_id : a_camp video id
         * image_id : a_camp image id
         * video_type : video type
         * rules : this value is set as default in front-end.
         * voting_duration : this range is 24~720 hours
         * if you run this, voting will be setup
         */
        let data = json!(SetCalloutReqModel {
            a_1: "".to_string(),
            a_2: "".to_string(),
            a_3: "".to_string(),
            a_4: "".to_string(),
            a_5: "".to_string(),
            opponent_id: "".to_string(),
            video_id: "".to_string(),
            image_id: "".to_string(),
            video_type: "".to_string(),
            rules: "".to_string(),
            voting_duration: 0,
        });

        let response = client
            .post("/api/v0/battle/callout/set-callout")
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

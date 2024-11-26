use crate::config::test_recaptcha_key;
use crate::rocket;
use crate::routes::vote::vote::{
    GetSignImgUrlReqModel, GetSignURLResModel, GetVoteListReqModel, GetVoteListResModel,
    SetVoteReqModel,
};
use crate::tests::routes::middleware::get_token::get_token;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;
use serde_json::Value;

/**
 * Test for sign url for vote sign
 */
#[rocket::async_test]
async fn get_sign_img_url() {
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
         * To do voting,
         * first we have to get the google bucket sign url,
         *  also verify voting information
         * match_id : this is battle 's match id
         * a_camp_votes : this is voting amount of a_camp
         * b_camp_votes : this is voting amount of b_camp
         *              NOTE a_camp_votes + b_camp_votes = 100
         *              (must satisfy this condition)
         * vote_type : this is voting type
         *              (0 is unofficial vote, 1 is official vote, 2 is judge vote)
         * if you run this test, you will get the sign url
         * NOTE, this router is not to only get the sign url...
         *       Here, I also implement voting information verify
         */
        let data = json!(GetSignImgUrlReqModel {
            match_id: "".to_string(),
            a_camp_votes: 50,
            b_camp_votes: 50,
            vote_type: 0,
            statement: "".to_string(),
        });

        let response = client
            .post("/api/v0/vote/get-sign-img-url")
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
 * Test for voting setup
 */
#[rocket::async_test]
async fn set_vote() {
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
         * To do voting, first we have to get the google bucket sign url also verify voting information
         * And second we can set up voting.. this function is for second process
         * match_id : this is battle 's match id
         * a_camp_votes : this is voting amount of a_camp
         * b_camp_votes : this is voting amount of b_camp NOTE a_camp_votes + b_camp_votes = 100 (must satisfy this condition)
         * vote_type : this is voting type (0 is unofficial vote, 1 is official vote, 2 is judge vote)
         * statement : this is voting statement
         * img_id : this is image id
         * if you run this test, voting will be applied
         * you can check it via database
         */
        let data = json!(SetVoteReqModel {
            match_id: "".to_string(),
            a_camp_votes: 50,
            b_camp_votes: 50,
            vote_type: 0,
            statement: "".to_string(),
            img_id: "".to_string(),
        });

        let response = client
            .post("/api/v0/vote/set-vote")
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
 * Test for voting setup
 */
#[rocket::async_test]
async fn get_voting_list() {
    // Build the Rocket instance
    let rocket = rocket().await;
    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * this test is for get the voting list
     * if voting period is ended, we have to get the voting list
     * match_id : this is battle 's match id
     * search : this is text for search
     * count : this is amount that show in front-end
     * pagination : this is pagination in front-end
     * if you run this test, you will get the voting list
     * you can check it via database
     */
    let data = json!(GetVoteListReqModel {
        match_id: "".to_string(),
        search: "".to_string(),
        count: 5,
        pagination: 1,
    });

    let response = client
        .post("/api/v0/vote/voting-list")
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
    let _: GetVoteListResModel =
        serde_json::from_value(body_json).expect("valid GetVoteListResModel");
    assert!(true, "success");
}

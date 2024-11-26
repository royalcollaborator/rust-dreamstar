use crate::config::test_recaptcha_key;
use crate::rocket;
use crate::routes::battle::battle_main::{
    GetShowListReqModel, GetShowListResModel, ShowSelectedBattleReqModel,
    ShowSelectedBattleResModel,
};
use crate::tests::routes::middleware::get_token::get_token;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;
use serde_json::Value;

/**
 * Test for show selected battle
 */
#[rocket::async_test]
async fn show_select_battle() {
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

    /*
     * This is for show battle
     * match_id : battle's match id
     * token : this is token, if user is not logged in, token will be empty
     * if you run this test, you will get the several information
     * first, voting is available for this user
     * second, voting list is available for this user
     * this user is not b_camp
     */
    let data = json!(ShowSelectedBattleReqModel {
        match_id: "".to_string(),
        token: token,
    });

    let response = client
        .post("/api/v0/battle/battle-main/show-select-battle")
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
    let _: ShowSelectedBattleResModel =
        serde_json::from_value(body_json).expect("valid ShowSelectedBattleResModel");
    assert!(true, "success");
}

/**
 * Test for show all battles
 */
#[rocket::async_test]
async fn show_battle_list() {
    // Build the Rocket instance
    let rocket = rocket().await;
    // Create a client for interacting with the Rocket instance
    let client = Client::untracked(rocket)
        .await
        .expect("Valid rocket instance");
    /*
     * This is for show battle list (for main page)
     * search : battle search
     * count : amount for show battle list in front-end
     * pagination : pagination in front-end
     * show_take_backs : this is flag that user back or not
     * show_incomplete : same
     * show_close : same
     */
    let data = json!(GetShowListReqModel {
        search: "".to_string(),
        count: 5,
        pagination: 1,
        show_take_backs: false,
        show_incomplete: false,
        show_close: true,
    });

    let response = client
        .post("/api/v0/battle/battle-main/show-battle-list")
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
    let _: GetShowListResModel =
        serde_json::from_value(body_json).expect("valid GetShowListResModel");
    assert!(true, "success");
}

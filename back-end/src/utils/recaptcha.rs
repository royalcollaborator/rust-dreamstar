use crate::config::{recaptcha_key, test_recaptcha_key};
use reqwest::Client;
use serde_json::Value;

/**
 * Check recaptcha result
 */
pub async fn is_valid_recaptcha(recaptcha_response: String, action: String) -> bool {
    let client = Client::new();
    let url = format!(
        "https://www.google.com/recaptcha/api/siteverify?secret={}&response={}",
        recaptcha_key(),
        recaptcha_response
    );
    let response = client.get(&url).send().await;
    match response {
        Ok(res) => {
            let json: Value = res.json().await.unwrap();
            if json["success"].as_bool().unwrap_or(false)
                && json["score"].as_f64().unwrap_or(0.0) >= 0.5
                && json["action"].to_string() == format!("\"{}\"", action.to_string())
            {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

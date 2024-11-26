use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Response,
};
use serde_json::Value;

use crate::config::{HOST_URL, RECAPTCHA_SITE_KEY};
use crate::utils::js_binding::recaptcha;
use crate::utils::{storage::get_local_storage, util::go_to_link};

pub async fn send_request(
    method: &str,
    url: &str,
    data: Value,
    auth_flag: bool,
    action: &str,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let recaptcha_token = recaptcha(RECAPTCHA_SITE_KEY, action).await;
    let recaptcha_tokens: String = recaptcha_token.into_serde().unwrap();
    // Set header in request
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "RecaptchaKey",
        HeaderValue::from_str(recaptcha_tokens.as_str()).unwrap(),
    );
    headers.insert(
        "recaptchaName",
        HeaderValue::from_str(action.to_string().as_str()).unwrap(),
    );
    if auth_flag {
        match get_local_storage("token") {
            Some(token) => {
                headers.insert(
                    "Authorization",
                    HeaderValue::from_str(token.as_str()).unwrap(),
                );
            }
            None => {
                go_to_link(format!("{}/login", HOST_URL).as_str());
            }
        }
    }
    if method == "get" {
        match client.get(url).headers(headers).send().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    } else if method == "post" {
        match client.post(url).headers(headers).json(&data).send().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    } else if method == "put" {
        match client.put(url).headers(headers).json(&data).send().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    } else {
        match client.delete(url).headers(headers).json(&data).send().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}

pub async fn request_without_recaptcha(
    method: &str,
    url: &str,
    data: Value,
    auth_flag: bool,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    if auth_flag {
        match get_local_storage("token") {
            Some(token) => {
                headers.insert(
                    "Authorization",
                    HeaderValue::from_str(token.as_str()).unwrap(),
                );
            }
            None => {
                go_to_link(format!("{}/login", HOST_URL).as_str());
            }
        }
    }
    if method == "get" {
        match client.get(url).headers(headers).send().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    } else if method == "post" {
        match client.post(url).headers(headers).json(&data).send().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    } else if method == "put" {
        match client.put(url).headers(headers).json(&data).send().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    } else {
        match client.delete(url).headers(headers).json(&data).send().await {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}

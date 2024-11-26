use crate::config::SERVER_URL;
use crate::pages::layout::layout::SharedData;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::router::Route;
use crate::utils::storage::{get_local_storage, set_local_storage};
use dioxus::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    status: String,
    token: String,
}

#[component]
pub fn GoogleLogin(
    state: String,
    code: String,
    scope: String,
    author: String,
    prompt: String,
) -> Element {
    let navigation = use_navigator();
    let code_string = use_signal(|| String::from(format!("{}", code)));
    let mut shared_data = use_context::<Signal<SharedData>>();
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();

    // show loading page
    let mut show_loading = move || {
        shared_data.set(SharedData {
            auth_flag: shared_data().auth_flag,
            loading_flag: true,
        })
    };

    // Exit loading page
    let mut exit_loading = move || {
        shared_data.set(SharedData {
            auth_flag: shared_data().auth_flag,
            loading_flag: false,
        })
    };

    // For google auth
    use_future(move || async move {
        let client = reqwest::Client::new();
        let code = code_string.to_string();
        show_loading();
        let response = match get_local_storage("enable_google") {
            Some(value) => {
                if value.to_string() == "false".to_string() {
                    client
                        .get(format!(
                            "{}/api/v0/auth/google/callback?code={}",
                            SERVER_URL, code
                        ))
                        .send()
                        .await
                } else {
                    let token = get_local_storage("token").unwrap();
                    set_local_storage("enable_google", "");
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        "Authorization",
                        HeaderValue::from_str(token.to_string().as_str()).unwrap(),
                    );
                    client
                        .put(format!(
                            "{}/api/v0/auth/google/callback?code={}",
                            SERVER_URL, code
                        ))
                        .headers(headers)
                        .send()
                        .await
                }
            }
            None => {
                client
                    .get(format!(
                        "{}/api/v0/auth/google/callback?code={}",
                        SERVER_URL, code
                    ))
                    .send()
                    .await
            }
        };

        if let Ok(res) = response {
            if res.status().is_success() {
                if res.headers().get(CONTENT_TYPE).map_or(false, |v| {
                    v.to_str().unwrap_or_default().contains("application/json")
                }) {
                    let body = res.text().await.unwrap();

                    match serde_json::from_str::<ApiResponse>(&body) {
                        Ok(response) if response.status == "success" => {
                            set_local_storage(
                                "token",
                                format!("Bearer {}", response.token.to_string()).as_str(),
                            );
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Google login success".to_string(),
                                    notification_type: NotificationType::Success,
                                });
                                data // Return the updated data
                            });
                            exit_loading();
                            navigation.push(Route::Profile);
                        }
                        Ok(_) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Google login failed".to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                            exit_loading();
                            navigation.push(Route::Login);
                        }
                        Err(_) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Failed to parse user info".to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                            exit_loading();
                            navigation.push(Route::Login);
                        }
                    }
                } else {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Response did not contain JSON content".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                    exit_loading();
                    navigation.push(Route::Login);
                }
            } else {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Failed to fetch user details".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
                
                exit_loading();  
                navigation.push(Route::Login);
            }
        } else {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Network request failed.".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            exit_loading();
            navigation.push(Route::Login);
        }
    });

    rsx! {
        section {
            class : "page-body-min-h"
        }
    }
}

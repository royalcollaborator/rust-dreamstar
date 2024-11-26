use super::footer::Footer;
use super::header::Header;
use super::loading::Loading;
use super::notification::Notification;
use crate::{config::SERVER_URL, utils::storage::set_local_storage};
use crate::pages::layout::progress::Progress;
use crate::router::Route;
use crate::utils::request::request_without_recaptcha;
use crate::utils::storage::get_local_storage;
use crate::utils::ErrResModel;
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
// shared data in all routers
#[derive(Clone, Copy)]
pub struct SharedData {
    pub auth_flag: bool,
    pub loading_flag: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NotificationType {
    Warn,
    Success,
    Error,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NotificationData {
    pub notification_type: NotificationType,
    pub content: String,
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthCheckResModel {
    result: bool,
    admin : bool
}
#[component]
pub fn Layout() -> Element {
    // Share data all components using use_context_provider()
    use_context_provider(|| {
        Signal::new(SharedData {
            auth_flag: false,
            loading_flag: false,
        })
    });
    // use context for notification
    use_context_provider(|| Signal::new(Vec::<NotificationData>::new()));
    let mut share_data = use_context::<Signal<SharedData>>();
    let mut admin_flag = use_signal(|| false);
    let notification_data = use_context::<Signal<Vec<NotificationData>>>();

    // Get loading flag from use context
    let loading_flag = use_memo(move || {
        share_data.read();
        share_data().loading_flag
    });
    // Get auth flag
    let auth_flag = use_memo(move || {
        share_data.read();
        share_data().auth_flag
    });

    // Check user auth status, like user already login or not
    use_effect(move || {
        match get_local_storage("token") {
            // Check local storage had token
            // if it has, we have to send request to check this token.
            None => {
                set_local_storage("user-role", "");
                share_data.set(SharedData {
                    auth_flag: false,
                    loading_flag: share_data().loading_flag,
                });
            }
            Some(tokens) => {
                // If token exist, send request
                spawn(async move {
                    // Send token verify request to server.
                    let response = request_without_recaptcha(
                        "post",
                        format!("{}/api/v0/auth/auth-check", SERVER_URL).as_str(),
                        json!({
                            "token" : tokens.to_string()
                        }),
                        false,
                    )
                    .await;
                    // Get the response
                    match response {
                        Ok(res) => {
                            // If response 's status is not success
                            if res.status() != StatusCode::OK {
                                match res.json::<ErrResModel>().await {
                                    Ok(_) => {
                                        // Set auth status
                                        share_data.set(SharedData {
                                            auth_flag: false,
                                            loading_flag: share_data().loading_flag,
                                        });
                                        set_local_storage("user-role", "");
                                        admin_flag.set(false);
                                    }
                                    Err(_) => {
                                        // Set auth status
                                        share_data.set(SharedData {
                                            auth_flag: false,
                                            loading_flag: share_data().loading_flag,
                                        });
                                        set_local_storage("user-role", "");
                                        admin_flag.set(false);
                                    }
                                }
                                return;
                            }
                            // If response status is success
                            match res.json::<AuthCheckResModel>().await {
                                // Set auth status
                                Ok(results) => {
                                    share_data.set(SharedData {
                                    auth_flag: results.result,
                                    loading_flag: share_data().loading_flag,
                                });
                                if results.admin {
                                    set_local_storage("user-role", "admin");
                                    admin_flag.set(true);
                                }
                            }
                                // if Response is error, auth status will be false
                                Err(_) => {
                                    // Set auth status
                                    share_data.set(SharedData {
                                        auth_flag: false,
                                        loading_flag: share_data().loading_flag,
                                    });
                                    set_local_storage("user-role", "");
                                    admin_flag.set(false);
                                }
                            }
                        }
                        Err(_) => share_data.set(SharedData {
                            auth_flag: true,
                            loading_flag: share_data().loading_flag,
                        }),
                    }
                });
            }
        };
    });

    rsx! {
        div {
            class : "fixed z-[9] flex flex-col top-[100px] max-[300px]:w-full w-[280px] sm:w-[350px] min-[300px]:right-[10px] min-[300px]:px-[5px]",
            {
                notification_data().iter().enumerate().map(move |(index, notification)| {
                    rsx!{
                        Notification {
                            id : index as i32,
                            notification_type : notification.notification_type.clone(),
                            title : notification.title.clone(),
                            content : notification.content.clone(),
                            notification_instance : notification_data.clone(),
                            notification_data : notification_data().clone()
                        }
                    }
                })
            }
        }
        Progress {}
        if loading_flag() {
            Loading {}
        }
        Header { auth_flag : auth_flag(), admin : admin_flag()}
        Outlet::<Route> {}
        Footer {}
    }
}

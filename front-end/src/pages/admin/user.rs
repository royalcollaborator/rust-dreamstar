use crate::config::SERVER_URL;
use crate::pages::layout::layout::SharedData;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::router::router::Route;
use crate::utils::request::{request_without_recaptcha, send_request};
use crate::utils::util::go_unauthorized;
use crate::utils::ErrResModel;

use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserRes {
    pub exist: bool,
    pub admin: bool,
    pub voter: bool,
    pub battler: bool,
    pub judge: bool,
}

#[component]
pub fn AdminUser() -> Element {
    let mut navigation = use_navigator();
    let mut username = use_signal(|| String::from(""));
    let mut role = use_signal(|| String::from("voter"));
    let mut status = use_signal(|| true);
    let mut user_check_flag = use_signal(|| false);
    let mut exist_flag = use_signal(|| false);
    let mut user_info = use_signal(|| GetUserRes {
        exist : false,
        admin : false,
        voter : false,
        battler : false,
        judge : false
    });
    // loading context
    let mut shared_data = use_context::<Signal<SharedData>>();
    // notification context
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

    // get the user info base on username and role
    let check_user = move |username : String, role : String| async move {
        if username.is_empty() {
            return;
        }
        match request_without_recaptcha(
            "post",
            format!("{}/admin/api/v0/user/admin-get-user-info", SERVER_URL).as_str(),
            json!({
               "username" : username.to_string(),
            }),
            true,
        )
        .await
        {
            Ok(res) => {
                if res.status() != StatusCode::OK {
                    if res.status() == StatusCode::UNAUTHORIZED {
                        exit_loading();
                        user_check_flag.set(false);
                        status.set(false);
                        go_unauthorized(navigation.clone());
                        return;
                    }
                    match res.json::<ErrResModel>().await {
                        Ok(results) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: format!(
                                        "Response Error : {}",
                                        results.cause.to_string()
                                    ),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                            exist_flag.set(false);
                        }
                        Err(e) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: format!("Response Error : {}", e.to_string()),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                            exist_flag.set(false);
                        }
                    }
                    user_info.set(GetUserRes {
                        exist : false,
                        admin : false,
                        voter : false,
                        battler : false,
                        judge : false
                    });
                    exist_flag.set(false);
                    return;
                }
                match res.json::<GetUserRes>().await {
                    Ok(res) => {
                        user_info.set(res.clone());
                        exist_flag.set(res.exist);
                    }
                    Err(_) => {
                        user_info.set(GetUserRes {
                            exist : false,
                            admin : false,
                            voter : false,
                            battler : false,
                            judge : false
                        });
                        exist_flag.set(false);
                    }
                }
            }
            Err(_) => {
                user_check_flag.set(false);
                status.set(false);
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: format!("Check your internet connection"),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
            }
        }
    };

    // Send user role change request to server
    let setup = move |role : String, status : bool| async move {
        // Check Event Code
        if username().is_empty() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Input Username".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        show_loading();
        match send_request(
            "post",
            format!("{}/admin/api/v0/user/role-setup", SERVER_URL).as_str(),
            json!({
               "username" : username().to_string(),
               "role" : role.to_string(),
               "status" : status
            }),
            true,
            "liveBattleCheck",
        )
        .await
        {
            Ok(res) => {
                if res.status() != StatusCode::OK {
                    if res.status() == StatusCode::UNAUTHORIZED {
                        exit_loading();
                        go_unauthorized(navigation.clone());
                        return;
                    }
                    match res.json::<ErrResModel>().await {
                        Ok(results) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: results.cause.to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                        }
                        Err(e) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: format!("Response Error : {}", e.to_string()),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                        }
                    }
                    exit_loading();
                    return;
                }
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: format!("User role setup success"),
                        notification_type: NotificationType::Success,
                    });
                    data // Return the updated data
                });
                exit_loading();
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: format!("Check your internet connection"),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
            }
        }
    };

    rsx! {
        div {
            class : "page-body-min-h pt-10 pb-12 w-4/5 mx-auto relative",
            div {
                class : "mt-10 w-[70%] md:w-2/4 mx-auto bg-gray-800 rounded-md p-5 ",
                h2 {
                    class : "text-xl text-center mb-5",
                    "Change User role"
                }
                div {
                    class : "flex flex-col gap-3",
                    input {
                        "type": "text",
                        oninput : move |e| {
                            username.set(e.value().to_string());
                            check_user(e.value().to_string(), role().to_string())
                        },
                        placeholder: "Username",
                        class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                    }
                    div {
                        class : "flex flex-row w-full p-2",
                        // Admin
                        label {
                            class: "inline-flex items-center cursor-pointer w-[50%]",
                            input {
                                r#type: "checkbox",
                                class: "sr-only peer",
                                value: "",
                                checked: user_info().admin,
                                disabled : !exist_flag(),
                                onchange : move |_| {
                                    let flag = !user_info().admin;
                                    user_info.set( GetUserRes {
                                        exist : user_info().exist,
                                        admin : flag,
                                        voter : user_info().voter,
                                        battler : user_info().battler,
                                        judge : user_info().judge
                                    });
                                    setup("admin".to_string(), flag)
                                },
                            }
                            div {
                                class: "relative w-11 h-6 bg-gray-700 peer-focus:ring-blue-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-600 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600",
                            }
                            span {
                                class: "ms-3 text-sm font-medium text-gray-300 ml-5",
                                "Admin",
                            }
                        }
                        // Battler
                        label {
                            class: "inline-flex items-center cursor-pointer w-[50%]",
                            input {
                                r#type: "checkbox",
                                class: "sr-only peer",
                                value: "",
                                checked: user_info().battler,
                                disabled : !exist_flag(),
                                onchange : move |_| {
                                    let flag = !user_info().battler;
                                    user_info.set( GetUserRes {
                                        exist : user_info().exist,
                                        admin : user_info().admin,
                                        voter : user_info().voter,
                                        battler : flag,
                                        judge : user_info().judge
                                    });
                                    setup("battler".to_string(), flag)
                                },
                            }
                            div {
                                class: "relative w-11 h-6 bg-gray-700 peer-focus:ring-blue-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-600 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600",
                            }
                            span {
                                class: "ms-3 text-sm font-medium text-gray-300 ml-5",
                                "Battler",
                            }
                        }
                        
                    }
                    div {
                        class : "flex flex-row w-full p-2",
                        // Judge
                        label {
                            class: "inline-flex items-center cursor-pointer w-[50%]",
                            input {
                                r#type: "checkbox",
                                class: "sr-only peer",
                                value: "",
                                checked: user_info().judge,
                                disabled : !exist_flag(),
                                onchange : move |_| {
                                    let flag = !user_info().judge;
                                    user_info.set( GetUserRes {
                                        exist : user_info().exist,
                                        admin : user_info().admin,
                                        voter : user_info().voter,
                                        battler : user_info().battler,
                                        judge : flag
                                    });
                                    setup("judge".to_string(), flag)
                                },
                            }
                            div {
                                class: "relative w-11 h-6 bg-gray-700 peer-focus:ring-blue-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-600 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600",
                            }
                            span {
                                class: "ms-3 text-sm font-medium text-gray-300 ml-5",
                                "Judge",
                            }
                        }
                        // Voter
                        label {
                            class: "inline-flex items-center cursor-pointer w-[50%]",
                            input {
                                r#type: "checkbox",
                                class: "sr-only peer",
                                value: "",
                                checked: user_info().voter,
                                disabled : !exist_flag(),
                                onchange : move |_| {
                                    let flag = !user_info().voter;
                                    user_info.set( GetUserRes {
                                        exist : user_info().exist,
                                        admin : user_info().admin,
                                        voter : flag,
                                        battler : user_info().battler,
                                        judge : user_info().judge
                                    });
                                    setup("voter".to_string(), flag)
                                },
                            }
                            div {
                                class: "relative w-11 h-6 bg-gray-700 peer-focus:ring-blue-800 rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-600 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600",
                            }
                            span {
                                class: "ms-3 text-sm font-medium text-gray-300 ml-5",
                                "Voter",
                            }
                        }             
                    }
            }
        }
    }
}
}

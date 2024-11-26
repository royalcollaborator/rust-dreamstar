use crate::config::SERVER_URL;
use crate::pages::layout::layout::SharedData;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::router::Route;
use crate::utils::request::send_request;
use crate::utils::storage::set_local_storage;
use crate::utils::util::go_to_link;
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tracing::error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResModel {
    pub token: String,
}

#[component]
pub fn Login() -> Element {
    let navigation = use_navigator();
    let mut shared_data = use_context::<Signal<SharedData>>();
    let mut email = use_signal(|| String::from(""));
    let mut warning = use_signal(|| String::from(""));
    let mut password = use_signal(|| String::from(""));
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
    let sign_in = move |_| async move {
        show_loading();
        // Send request
        let result = send_request(
            "post",
            format!("{}/api/v0/auth/google", SERVER_URL).as_str(),
            json!({}),
            false,
            "enableGoogle",
        )
        .await;
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<HashMap<String, String>>().await {
                        Ok(data) => {
                            if let Some(url) = data.get("url") {
                                exit_loading();
                                set_local_storage("enable_google", "false");
                                go_to_link(url);
                            } else {
                                error!("URL not found in response");
                            }
                        }
                        Err(e) => {
                            exit_loading();
                            error!("Failed to deserialize response: {}", e);
                        }
                    }
                } else {
                    exit_loading();
                    error!("Failed to sign in. Status: {}", response.status());
                }
            }
            Err(e) => {
                exit_loading();
                error!("Network request had a problem: {}", e);
            }
        }
    };

    // For Instagram auth
    // Enable Instagram login
    let sign_instagram = move |_| async move {
        show_loading();
        match send_request(
            "get",
            format!("{}/api/v0/auth/instagram", SERVER_URL).as_str(),
            json!({}),
            false,
            "enableInstagram",
        )
        .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<HashMap<String, String>>().await {
                        Ok(data) => {
                            if let Some(url) = data.get("url") {
                                exit_loading();
                                set_local_storage("enable_instagram", "false");
                                go_to_link(url);
                            } else {
                                exit_loading();
                                notification_data.set({
                                    let mut data = notification_data().clone(); // Clone existing data
                                    data.push(NotificationData {
                                        title: "".to_string(),
                                        content: "URL not found in response".to_string(),
                                        notification_type: NotificationType::Error,
                                    });
                                    data // Return the updated data
                                });
                            }
                        }
                        Err(_) => {
                            exit_loading();
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Failed to deserialize response".to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                        }
                    }
                } else {
                    exit_loading();
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Failed to sign in".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                }
            }
            Err(_) => {
                exit_loading();
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Please check your internet connection".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
            }
        }
    };

    // Fetch data to server;
    let submit = move |_| async move {
        show_loading();
        // Send signup request into server
        let res = send_request(
            "post",
            format!("{}/api/v0/auth/login", SERVER_URL).as_str(),
            json!({
                "email" : email.to_string(),
                "password" : password.to_string()
            }),
            false,
            "login",
        )
        .await;
        match res {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    match response.json::<LoginResModel>().await {
                        Ok(result) => {
                            set_local_storage(
                                "token",
                                format!("Bearer {}", result.token.to_string()).as_str(),
                            );
                            shared_data.set(SharedData {
                                auth_flag: true,
                                loading_flag: false,
                            });
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Login success".to_string(),
                                    notification_type: NotificationType::Success,
                                });
                                data // Return the updated data
                            });
                            navigation.push(Route::Profile);
                        }
                        Err(_) => {
                            exit_loading();
                            warning.set("Server Error".to_string());
                        }
                    }
                } else if response.status() == StatusCode::FORBIDDEN {
                    exit_loading();
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "reCAPTCHA error, please resend".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    exit_loading();
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Server error".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::UNAUTHORIZED {
                    exit_loading();
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Email or Password doesn't match".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::BAD_REQUEST {
                    exit_loading();
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Bad request".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else {
                    exit_loading();
                }
            }
            Err(_) => {
                exit_loading();
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Server Error".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
            }
        }
    };

    rsx! {

        section {
            class: "page-body-min-h flex items-center justify-center bg-black text-white",
            div {
                id: "login-box",
                class: "flex rounded-2xl shadow-lg max-w-3xl p-5 items-center",
                div {
                    class: "md:w-1/2 px-8 md:px-16",
                    h2 {
                        class: "font-bold text-3xl text-center",
                        "DanceBattleZ"
                    }
                    if "{warning}" != String::from("") {
                        div {
                            class : "text-red-400 w-[100%] text-center px-[5%] break-all",
                            span {
                                class : "break-all",
                                "{warning}"
                            }
                        }
                    }
                    form {
                        class: "flex flex-col gap-4 text-block",
                        onsubmit : move |_| submit(()),
                        input {
                            class: "mt-8 rounded-xl border outline-none text-white bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                            "type": "text",
                            "test-id" : "login-email-input",
                            value : email,
                            oninput : move |e| email.set(e.value()),
                            placeholder: "Email or Nickname",
                            required: true
                        }

                            input {
                                class: "p-2 rounded-xl w-full border text-white outline-none bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                                "type": "password",
                                "test-id" : "login-password-input",
                                value : password,
                                oninput : move |e| password.set(e.value()),
                                placeholder: "Password",
                                required: true
                            }
                        button {
                            id: "spin",
                            "type"  : "submit",
                            "test-id" : "login-submit-button",
                            class: "bg-[#002D74] rounded-xl text-white py-2 hover:scale-105 duration-300",
                            "Login"
                        }
                    }
                    div {
                        class: "mt-6 grid grid-cols-3 items-center text-white-400",
                        hr {
                            class: "border-white-400"
                        }
                        p {
                            class: "text-center text-sm",
                            "OR"
                        }
                        hr {
                            class: "border-white-400"
                        }
                    }
                        button {
                            class: "bg-white border py-2 w-full rounded-xl mt-5 flex justify-center items-center text-sm hover:scale-105 duration-300 text-black",
                            "test-id" : "login-button",
                            onclick : move |_| sign_in(()),
                            img {
                                src: "/static/image/google_logo.png",
                                class : "w-[15%]"
                            }
                            "Login with Google"
                        }
                        button {
                            class: "bg-white border py-2 w-full rounded-xl mt-5 flex justify-center items-center text-sm hover:scale-105 duration-300 text-black",
                            onclick : move |_| sign_instagram(()),
                            "test-id" : "login-register",
                            img {
                                src: "/static/image/instagram_logo.jpg",
                                class : "w-[15%]"
                            }
                            "Login with Instagram"
                        }
                    div {
                        class: "mt-3 text-xs flex justify-between items-center",
                        p {
                            "Don't have an account?"
                        }
                        Link {to : Route::Signup,
                        button {
                            class: "py-2 px-5 bg-white border rounded-xl hover:scale-110 duration-300 text-black",
                            "Register"
                        }
                        }
                    }
                    div {
                        class: "mt-3 text-xs flex justify-between items-center",
                        Link {to : Route::ForgetPass, class : "w-full",
                        button {
                            class: "py-2 px-5 bg-white w-full border rounded-xl hover:scale-110 duration-300 text-black",
                            "test-id" : "login-reset-password",
                            "Reset Password"
                        }
                        }
                    }
                }
                div {
                    class: "md:block hidden w-1/2",
                    img {
                        class: "rounded-2xl",
                        src: "/static/image/bg_2.webp"
                    }
                }

            }
        }
        // AuthFooter {}
    }
}

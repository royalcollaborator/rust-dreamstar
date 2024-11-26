use crate::config::SERVER_URL;
use crate::pages::layout::layout::SharedData;
use crate::router::Route;
use crate::utils::request::send_request;
use crate::utils::storage::set_local_storage;
use crate::utils::validation::validate_password;

use crate::pages::layout::layout::{NotificationData, NotificationType};
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde_json::json;

#[component]
pub fn Signup() -> Element {
    let navigation = use_navigator();
    let mut shared_data = use_context::<Signal<SharedData>>();
    let mut email = use_signal(|| String::from(""));
    let mut username = use_signal(|| String::from(""));
    let mut password = use_signal(|| String::from(""));
    let mut repeat_password = use_signal(|| String::from(""));
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

    // Fetch data to server;
    let submit = move |_| async move {
        // Check password match
        if password.to_string() != repeat_password.to_string() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Password does not match".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }

        // Check password is strong
        let pass_check = validate_password(password.to_string());
        if !pass_check {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Passwords must be at least 8 characters, and must have capital, lowercase, number and special characters".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        show_loading();
        // Send signup request into server
        let res = send_request(
            "post",
            format!("{}/api/v0/auth/signup", SERVER_URL).as_str(),
            json!({
                "email" : email.to_string(),
                "username" : username.to_string(),
                "password" : password.to_string()
            }),
            false,
            "sign",
        )
        .await;

        match res {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    exit_loading();
                    set_local_storage("invitation_email", email.to_string().as_str());
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "User registered successfully".to_string(),
                            notification_type: NotificationType::Success,
                        });
                        data // Return the updated data
                    });
                    navigation.push(Route::Invitation);
                } else if response.status() == StatusCode::FORBIDDEN {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "ReCAPTCHA error, please resend".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Server error".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::CONFLICT {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Email or Username already registered".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::ALREADY_REPORTED {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content:
                                "You 've already registered but your account need email verify."
                                    .to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::BAD_REQUEST {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Bad request.".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                }
                exit_loading();
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Server Error.".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
                exit_loading();
            }
        }
    };

    rsx! {
        section { class: "page-body-min-h flex items-center justify-center bg-black ",
            div {
                id: "login-box",
                class: "flex rounded-2xl shadow-lg max-w-3xl p-5 items-center",
                div { class: "md:w-1/2 px-8 md:px-16",
                    h2 { class: "font-bold text-3xl text-center text-white", "DanceBattleZ" }
                    form {
                        class: "flex flex-col gap-4 text-black",
                        onsubmit: move |_| submit(()),
                        input {
                            class: "p-3 mt-1 rounded-xl text-white outline-none bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                            "type": "email",
                            "test-id"  : "signup-email-input",
                            value: email,
                            oninput: move |e| email.set(e.value()),
                            placeholder: "Email",
                            required: true
                        }
                        input {
                            class: "p-3 mt-1 rounded-xl text-white outline-none bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                            "type": "text",
                            "test-id"  : "signup-username-input",
                            value: username,
                            oninput: move |e| username.set(e.value()),
                            placeholder: "Nickname",
                            required: true
                        }

                        input {
                            class: "p-3 rounded-xl w-full  outline-none text-white bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                            "type": "password",
                            "test-id"  : "signup-password-input",
                            value: password,
                            oninput: move |e| password.set(e.value()),
                            placeholder: "Password",
                            required: true
                        }

                        input {
                            class: "p-3 rounded-xl w-full outline-none text-white bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                            "type": "password",
                            "test-id"  : "signup-repassword-input",
                            value: repeat_password,
                            oninput: move |e| repeat_password.set(e.value()),
                            placeholder: "Repeat Password",
                            required: true
                        }

                        button {
                            id: "spin",
                            class: "bg-[#002D74] rounded-xl text-white py-2 hover:scale-105 duration-300",
                            "test-id"  : "signup-confirm-button",
                            "type": "submit",
                            "Sign Up"
                        }
                    }
                    div { class: "mt-6 grid grid-cols-3 items-center text-white-400",
                        hr { class: "border-white-400" }
                        p { class: "text-center text-sm text-white", "OR" }
                        hr { class: "border-white-400" }
                    }
                    Link { to: Route::Invitation,
                        button { class: "bg-white border py-2 w-full rounded-xl mt-5 flex justify-center items-center text-sm hover:scale-105 duration-300 text-black",
                        "test-id"  : "signup-invitation-button",
                            "Have an invite code, Click here"
                        }
                    }
                    Link { to: Route::Login,
                        div { class: "mt-3 text-xs flex justify-between items-center text-white ",
                            p { "Have an account?" }
                            button { class: "py-2 px-5 bg-white border rounded-xl  hover:scale-110 duration-300 text-black",
                            "test-id"  : "signup-login-button",
                                "Login"
                            }
                        }
                    }
                }
                div { class: "md:block hidden w-1/2",
                    img { class: "rounded-2xl", src: "/static/image/bg_2.webp" }
                }
            }
        }
    }
}

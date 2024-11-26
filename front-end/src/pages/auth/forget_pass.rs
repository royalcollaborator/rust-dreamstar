use crate::config::SERVER_URL;
use crate::pages::layout::layout::SharedData;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::router::Route;
use crate::utils::request::send_request;
use crate::utils::validation::validate_password;
use crate::utils::ErrResModel;
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde_json::json;

#[component]
pub fn ForgetPass() -> Element {
    let navigation = use_navigator();
    let mut shared_data = use_context::<Signal<SharedData>>();
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();
    let mut email = use_signal(|| String::from(""));
    let mut code = use_signal(|| String::from(""));
    let mut password = use_signal(|| String::from(""));
    let mut repeat_password = use_signal(|| String::from(""));
    let mut email_flag = use_signal(|| false);

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

    // Send Forget Request to back-end
    let submit = move |_| async move {
        // Check password is same with repeat password.
        if password().to_string() != repeat_password().to_string() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Password doesn't match".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        // check password validation
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
        // Send request to server
        match send_request(
            "post",
            format!("{}/api/v0/auth/reset-pass", SERVER_URL).as_str(),
            json!({
                "email" : email().to_string(),
                "password" : password().to_string(),
                "code" : code().to_string()
            }),
            false,
            "resetPass",
        )
        .await
        {
            Ok(res) => {
                // Status is OK
                if res.status() == StatusCode::OK {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Password Changed".to_string(),
                            notification_type: NotificationType::Success,
                        });
                        data // Return the updated data
                    });
                    //Go to login
                    exit_loading();
                    navigation.push(Route::Login);
                } else {
                    // Status is not OK
                    match res.json::<ErrResModel>().await {
                        Ok(result) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: result.cause.to_string(),
                                    notification_type: NotificationType::Error,
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
                                    content: "Response Error".to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                            exit_loading();
                        }
                    }
                }
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Check your internet connection".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
                exit_loading();
            }
        }
    };

    let email_submit = move |_| async move {
        show_loading();
        match send_request(
            "post",
            format!("{}/api/v0/auth/reset-pass-send-email", SERVER_URL).as_str(),
            json!({
                "email" : email().to_string()
            }),
            false,
            "resetPass",
        )
        .await
        {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Email Sent successfully".to_string(),
                            notification_type: NotificationType::Success,
                        });
                        data // Return the updated data
                    });
                    email_flag.set(true);
                    exit_loading();
                    // navigation.push(Route::Invitation);
                } else {
                    match res.json::<ErrResModel>().await {
                        Ok(result) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: result.cause.to_string(),
                                    notification_type: NotificationType::Error,
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
                                    content: "Response Error".to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                            exit_loading();
                        }
                    }
                }
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Check Internet Connection".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
                exit_loading();
            }
        }
    };

    rsx! {
        section { class: "page-body-min-h flex items-center justify-center bg-black text-black",
            div {
                id: "login-box",
                class: "flex rounded-2xl shadow-lg max-w-3xl p-5 items-center",
                div { class: "px-8 md:px-16",
                    h2 { class: "font-bold text-3xl text-center text-white", "Forget Password" }
                    if email_flag() {
                        form {
                            class: "flex flex-col gap-4",
                            onsubmit: move |_| submit(()),
                            input {
                                class: "p-2 mt-8 rounded-xl border text-white bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                                "type": "email",
                                disabled: true,
                                value: email,
                                placeholder: "Email",
                                required: true
                            }
                            input {
                                class: "p-2 rounded-xl border text-white bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                                "type": "Password",
                                "test-id"  : "reset-password-new-password-input",
                                value: password,
                                oninput: move |e| password.set(e.value()),
                                placeholder: "New Password",
                                required: true
                            }
                            input {
                                class: "p-2 rounded-xl border text-white bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                                "type": "Password",
                                "test-id"  : "reset-password-new-repassword-input",
                                value: repeat_password,
                                oninput: move |e| repeat_password.set(e.value()),
                                placeholder: "Repeat Password",
                                required: true
                            }
                            input {
                                class: "p-2 rounded-xl border text-white bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                                "type": "text",
                                "test-id"  : "reset-password-code-input",
                                value: code,
                                oninput: move |e| code.set(e.value()),
                                placeholder: "Code",
                                required: true
                            }
                            div { class: "relative text-white",
                                p { "Your email will receive forget password code." }
                            }
                            button {
                                "type": "submit",
                                "test-id"  : "reset-password-confirm-button",
                                class: "bg-[#002D74] rounded-xl text-white py-2 hover:scale-105 duration-300",
                                "Go ahead"
                            }
                        }
                    } else {
                        form {
                            class: "flex flex-col gap-4",
                            onsubmit: move |_| email_submit(()),
                            input {
                                class: "p-2 mt-8 rounded-xl border text-white bg-gray-700 px-2 py-[0.8rem] rounded-md border",
                                "type": "email",
                                "test-id"  : "reset-password-email-input",
                                oninput: move |e| email.set(e.value()),
                                value: email,
                                placeholder: "Email",
                                required: true
                            }
                            button {
                                "type": "submit",
                                "test-id"  : "reset-password-email-submit-button",
                                class: "bg-[#002D74] rounded-xl text-white py-2 hover:scale-105 duration-300",
                                "Send Code"
                            }
                        }
                    }

                    Link { to: Route::Login,
                        div { class: "mt-3 text-xs flex justify-between items-center",
                            button { class: "py-2 px-5 bg-white border rounded-xl hover:scale-110 duration-300 text-black",
                                "Go Back"
                            }
                        }
                    }
                }
            }
        }
    }
}

use crate::config::SERVER_URL;
use crate::pages::layout::layout::SharedData;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::router::Route;
use crate::utils::request::send_request;
use crate::utils::storage::{get_local_storage, set_local_storage};
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde_json::json;

#[component]
pub fn Invitation() -> Element {
    let navigation = use_navigator();
    let mut shared_data = use_context::<Signal<SharedData>>();
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();
    let mut email = use_signal(|| match get_local_storage("invitation_email") {
        Some(value) => value,
        None => "".to_string(),
    });
    let mut code = use_signal(|| String::from(""));

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

    // Send invitation code to back-end
    let submit = move |_| async move {
        // Get recaptcha token
        show_loading();
        // send invitation request to server
        let res = send_request(
            "post",
            format!("{}/api/v0/auth/invitation", SERVER_URL).as_str(),
            json!({
                "email" : email.to_string(),
                "code" : code.to_string()
            }),
            false,
            "invitation",
        )
        .await;

        match res {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    exit_loading();
                    set_local_storage("invitation_email", "");
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Email verify successfully".to_string(),
                            notification_type: NotificationType::Success,
                        });
                        data // Return the updated data
                    });
                    navigation.push(Route::Login);
                } else if response.status() == StatusCode::FORBIDDEN {
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
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Server Error".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::BAD_REQUEST {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Email or Invitation code doesn't match".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::REQUEST_TIMEOUT {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Invitation code expired".to_string(),
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
                        content: "Server Error".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
                exit_loading();
            }
        };
    };

    // Request new invitation code to back-end
    let code_send = move |_| async move {
        if email.to_string() == String::from("") {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Input email".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        exit_loading();
        // Send invitation code to server
        let res = send_request(
            "post",
            format!("{}/api/v0/auth/invitation/resend", SERVER_URL).as_str(),
            json!({
                "email" : email.to_string(),
            }),
            false,
            "invitationCode",
        )
        .await;

        match res {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Resend invitation code".to_string(),
                            notification_type: NotificationType::Success,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::FORBIDDEN {
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
                } else if response.status() == StatusCode::BAD_REQUEST {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Email doesn't exist".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else if response.status() == StatusCode::REQUEST_TIMEOUT {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Invitation code expired".to_string(),
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
                        content: "Server Error".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
                exit_loading();
            }
        };
    };

    rsx! {
        section { class: "page-body-min-h flex items-center justify-center bg-black text-black",
            div {
                id: "login-box",
                class: "flex rounded-2xl shadow-lg max-w-3xl p-5 items-center",
                div { class: "px-8 md:px-16",
                    h2 { class: "font-bold text-3xl text-center text-white", "Invitation" }
                    form {
                        class: "flex flex-col gap-4",
                        onsubmit: move |_| submit(()),
                        input {
                            class: "p-2 mt-8 rounded-xl border bg-gray-700 px-2 py-[0.8rem] rounded-md border text-white",
                            "type": "email",
                            "test-id" : "invitation-email-input",
                            value: email,
                            oninput: move |e| email.set(e.value()),
                            placeholder: "Email",
                            required: true
                        }
                        input {
                            class: "p-2 rounded-xl border bg-gray-700 px-2 py-[0.8rem] rounded-md border text-white",
                            "type": "text",
                            "test-id" : "invitation-code-input",
                            value: code,
                            oninput: move |e| code.set(e.value()),
                            placeholder: "Code",
                            required: true
                        }
                        div { class: "relative text-white",
                            p { "Your email will receive invitation code" }
                        }
                        button {
                            "type": "submit",
                            class: "bg-[#002D74] rounded-xl text-white py-2 hover:scale-105 duration-300",
                            "test-id" : "invitation-confirm-button",
                            "Go ahead"
                        }
                        button {
                            "type": "button",
                            class: "bg-[#002D74] rounded-xl text-white py-2 hover:scale-105 duration-300",
                            onclick: move |_| code_send(()),
                            "Resend Code Into your email"
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

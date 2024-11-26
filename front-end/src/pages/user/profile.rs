use crate::config::SERVER_URL;
use crate::pages::layout::layout::SharedData;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::router::Route;
use crate::utils::request::send_request;
use crate::utils::storage::set_local_storage;
use crate::utils::util::go_to_link;
use crate::utils::util::go_unauthorized;
use crate::utils::validation::validate_password;
use crate::utils::ErrResModel;
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResModel {
    pub username: String,
    pub email: String,
    pub google_email: String,
    pub instagram_name: String,
    pub voter: bool,
    pub battler: bool,
    pub judger: bool,
    pub admin: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResModel {
    token: String,
}

#[component]
pub fn Profile() -> Element {
    let navigation = use_navigator();
    let mut shared_data = use_context::<Signal<SharedData>>();
    let mut userInfo = use_signal(|| UserResModel {
        username: Default::default(),
        email: Default::default(),
        google_email: Default::default(),
        instagram_name: Default::default(),
        voter: false,
        battler: false,
        judger: false,
        admin: false,
    });
    let mut email_flag = use_signal(|| false);
    let mut username_flag = use_signal(|| false);
    let mut password_flag = use_signal(|| false);
    let mut username = use_signal(|| String::from(""));
    let mut password = use_signal(|| String::from(""));
    let mut email = use_signal(|| String::from(""));
    let mut invitation_code = use_signal(|| String::from(""));
    let mut invitation_code_flag = use_signal(|| false);
    let mut load_data_flag = use_signal(|| false);
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

        // Handle click events to prevent state change
        let on_click = move |event: MouseEvent| {
            // Prevent any action that might change the checkbox state
            event.stop_propagation();
        };

    // Get load data
    use_future(move || async move {
        show_loading();
        if !load_data_flag() {
            match send_request(
                "get",
                format!("{}/api/v0/user/userInfo", SERVER_URL).as_str(),
                json!({}),
                true,
                "verifyToken",
            )
            .await
            {
                Ok(res) => {
                    if res.status() == StatusCode::OK {
                        match res.json::<UserResModel>().await {
                            Ok(result) => {
                                userInfo.set(result);
                                load_data_flag.set(true);
                                exit_loading();
                            }
                            Err(_) => {
                                exit_loading();
                                navigation.push(Route::Login);
                            }
                        }
                    } else if res.status() == StatusCode::UNAUTHORIZED {
                        notification_data.set({
                            let mut data = notification_data().clone(); // Clone existing data
                            data.push(NotificationData {
                                title: "".to_string(),
                                content: "You are not logged in".to_string(),
                                notification_type: NotificationType::Error,
                            });
                            data // Return the updated data
                        });
                        exit_loading();
                        go_unauthorized(navigation.clone());
                    } else if res.status() == StatusCode::FORBIDDEN {
                        exit_loading();
                        notification_data.set({
                            let mut data = notification_data().clone(); // Clone existing data
                            data.push(NotificationData {
                                title: "".to_string(),
                                content: "reCAPTCHA Error".to_string(),
                                notification_type: NotificationType::Error,
                            });
                            data // Return the updated data
                        });
                        navigation.push(Route::Login);
                    } else {
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
                        navigation.push(Route::Login);
                    }
                    exit_loading();
                }
                Err(_) => {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Please check your internet connection".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                    exit_loading();
                }
            }
        }
    });

    // Send username change request to server
    let username_change = move |_| async move {
        if username() == "".to_string() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Please input username".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        show_loading();
        match send_request(
            "post",
            format!("{}/api/v0/user/usernameChange", SERVER_URL).as_str(),
            json!({
                "username" : username().to_string()
            }),
            true,
            "nameChange",
        )
        .await
        {
            Ok(res) => {
                if res.status() == StatusCode::ALREADY_REPORTED {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "This username already used".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                    exit_loading();
                } else if res.status() == StatusCode::OK {
                    match res.json::<TokenResModel>().await {
                        Ok(result) => {
                            set_local_storage(
                                "token",
                                format!("Bearer {}", result.token.to_string()).as_str(),
                            );
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Username changed".to_string(),
                                    notification_type: NotificationType::Success,
                                });
                                data // Return the updated data
                            });
                            username_flag.set(false);
                            let new_user_info = userInfo().clone();
                            userInfo.set(UserResModel {
                                username: username().to_string(),
                                email: new_user_info.email.to_string(),
                                google_email: new_user_info.google_email,
                                instagram_name: new_user_info.instagram_name.to_string(),
                                voter: new_user_info.voter,
                                battler: new_user_info.battler,
                                judger: new_user_info.judger,
                                admin: new_user_info.admin,
                            });
                            exit_loading();
                        }
                        Err(_) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Server Error".to_string(),
                                    notification_type: NotificationType::Success,
                                });
                                data // Return the updated data
                            });
                            exit_loading();
                        }
                    }
                } else if res.status() == StatusCode::UNAUTHORIZED {
                    go_unauthorized(navigation.clone());
                } else if res.status() == StatusCode::FORBIDDEN {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "reCAPTCHA Error, please try once more".to_string(),
                            notification_type: NotificationType::Success,
                        });
                        data // Return the updated data
                    });
                } else {
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
                exit_loading();
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

    // Send email change request to server
    let email_change = move |_| async move {
        if email() == "".to_string() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Please input email".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        show_loading();
        match send_request(
            "post",
            format!("{}/api/v0/user/email-change-code", SERVER_URL).as_str(),
            json!({
                "new_email" : email().to_string()
            }),
            true,
            "mailChange",
        )
        .await
        {
            Ok(res) => {
                if res.status() != StatusCode::OK {
                    if res.status() == StatusCode::UNAUTHORIZED {
                        exit_loading();
                        notification_data.set({
                            let mut data = notification_data().clone(); // Clone existing data
                            data.push(NotificationData {
                                title: "".to_string(),
                                content: "Please loin fist".to_string(),
                                notification_type: NotificationType::Error,
                            });
                            data // Return the updated data
                        });
                        go_unauthorized(navigation.clone());
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
                        }
                    }
                    exit_loading();
                    return;
                }
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Invitation code sent to your email".to_string(),
                        notification_type: NotificationType::Success,
                    });
                    data // Return the updated data
                });
                invitation_code_flag.set(true);
                email_flag.set(false);
                let new_user_info = userInfo().clone();
                userInfo.set(UserResModel {
                    username: new_user_info.username.to_string(),
                    email: email().to_string(),
                    google_email: new_user_info.google_email,
                    instagram_name: new_user_info.instagram_name.to_string(),
                    voter: new_user_info.voter,
                    battler: new_user_info.battler,
                    judger: new_user_info.judger,
                    admin: new_user_info.admin,
                });
                exit_loading();
                return;
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Check your internet".to_string(),
                        notification_type: NotificationType::Success,
                    });
                    data // Return the updated data
                });
                exit_loading();
                return;
            }
        }
    };

    // Send password change request to server
    let password_change = move |_| async move {
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
        match send_request(
            "post",
            format!("{}/api/v0/user/passwordChange", SERVER_URL).as_str(),
            json!({
                "password" : password().to_string()
            }),
            true,
            "passwordChange",
        )
        .await
        {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    match res.json::<TokenResModel>().await {
                        Ok(result) => {
                            set_local_storage(
                                "token",
                                format!("Bearer {}", result.token.to_string()).as_str(),
                            );
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Password changed successfully".to_string(),
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
                                    content: "Server Error".to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                            exit_loading();
                        }
                    }
                } else if res.status() == StatusCode::UNAUTHORIZED {
                    go_unauthorized(navigation.clone());
                } else if res.status() == StatusCode::FORBIDDEN {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "reCAPTCHA Error, please try once more".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                } else {
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
                exit_loading();
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Check internet connection".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
            }
        }
    };

    // Send invitation code to server
    let send_invitation_code = move |_| async move {
        show_loading();
        match send_request(
            "post",
            format!("{}/api/v0/user/reset-email", SERVER_URL).as_str(),
            json!({
                "email" : email().to_string(),
                "code" : invitation_code().to_string()
            }),
            true,
            "invitationCode",
        )
        .await
        {
            Ok(res) => {
                if res.status() != StatusCode::OK {
                    if res.status() == StatusCode::UNAUTHORIZED {
                        exit_loading();
                        go_unauthorized(navigation.clone());
                    }
                    match res.json::<ErrResModel>().await {
                        Ok(results) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: results.cause.to_string().to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                        }
                        Err(_) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Response Error".to_string().to_string(),
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
                        content: "Email changed successfully".to_string().to_string(),
                        notification_type: NotificationType::Success,
                    });
                    data // Return the updated data
                });
                invitation_code_flag.set(false);
                exit_loading();
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Server Error".to_string().to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
                exit_loading();
            }
        }
    };

    // Enable google login
    let enable_google = move |_| async move {
        match send_request(
            "post",
            format!("{}/api/v0/auth/google", SERVER_URL).as_str(),
            json!({}),
            false,
            "enableGoogle",
        )
        .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<HashMap<String, String>>().await {
                        Ok(data) => {
                            if let Some(url) = data.get("url") {
                                set_local_storage("enable_google", "true");
                                go_to_link(url);
                            } else {
                                notification_data.set({
                                    let mut data = notification_data().clone(); // Clone existing data
                                    data.push(NotificationData {
                                        title: "".to_string(),
                                        content: "URL not found in response"
                                            .to_string()
                                            .to_string(),
                                        notification_type: NotificationType::Error,
                                    });
                                    data // Return the updated data
                                });
                            }
                        }
                        Err(_) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Failed to deserialize response"
                                        .to_string()
                                        .to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                        }
                    }
                } else {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Failed to sign in".to_string().to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                }
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Check your internet connection".to_string().to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
            }
        }
    };

    // Enable Instagram login
    let enable_instagram = move |_| async move {
        match send_request(
            "post",
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
                                set_local_storage("enable_instagram", "true");
                                go_to_link(url)
                            } else {
                                notification_data.set({
                                    let mut data = notification_data().clone(); // Clone existing data
                                    data.push(NotificationData {
                                        title: "".to_string(),
                                        content: "URL not found in response"
                                            .to_string()
                                            .to_string(),
                                        notification_type: NotificationType::Error,
                                    });
                                    data // Return the updated data
                                });
                            }
                        }
                        Err(_) => {
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Failed to deserialize response"
                                        .to_string()
                                        .to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                        }
                    }
                } else {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Failed to sign in".to_string().to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                }
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Check internet connection".to_string().to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
            }
        }
    };

    rsx! {
        div { class: "page-body-min-h bg-black text-white",
            section { class: "pt-[3rem] pb-[3rem]",
                div { class: "w-4/5 mx-auto",
                    form { class: "flex flex-col gap-5 lg:w-1/2 md:w-1/2 sm:w-full mx-auto",
                        
                        if username_flag() {
                            div { class: "flex flex-col gap-2",
                                label { class: "text-gray-300 text-sm flex items-center gap-2",
                                    i { class: "fa fa-user" }
                                    " Username"
                                }
                                input {
                                    "type": "text",
                                    placeholder: "Enter username...",
                                    "test-id" : "profile-username-change-input",
                                    value: username(),
                                    oninput: move |e| username.set(e.value()),
                                    class: "bg-gray-700 px-2 py-2 rounded-md"
                                }
                                div { class: "flex items-center gap-2",
                                    button {
                                        class: "px-2 py-1 flex items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                        "test-id"  : "profile-username-change-confirm",
                                        onclick: move |_| username_change(()),
                                        "type": "button",
                                        i { class: "fa fa-chevron-right" }
                                        " Enter"
                                    }
                                    button {
                                        onclick: move |_| username_flag.set(false),
                                        class: "px-2 py-1 flex items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                        i { class: "fa fa-times" }
                                        " Cancel"
                                    }
                                }
                            }
                        } else {
                            div { class: "flex flex-col gap-2",
                                label { class: "text-gray-300 text-sm flex items-center gap-2",
                                    i { class: "fa fa-user" }
                                    " Username"
                                }
                                input {
                                    "type": "text",
                                    value: userInfo().username,
                                    disabled: true,
                                    placeholder: "Enter username...",
                                    class: "bg-gray-700 px-2 py-2 rounded-md"
                                }
                                div { class: "flex items-center gap-2",
                                    button {
                                        onclick: move |_| username_flag.set(true),
                                        "test-id"  : "profile-username-change-button",
                                        class: "px-2 py-1 flex items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                        i { class: "fa fa-chevron-right" }
                                        "Reset"
                                    }
                                }
                            }
                        }
                        
                        if invitation_code_flag() {
                            div { class: "flex flex-col gap-2",
                                label { class: "text-gray-300 text-sm flex items-center gap-2",
                                    i { class: "fa fa-envelope" }
                                    " Invitation code"
                                }
                                input {
                                    "type": "text",
                                    value: invitation_code(),
                                    "test-id" : "profile-invitation-code-input",
                                    oninput: move |e| invitation_code.set(e.value()),
                                    placeholder: "Enter code...",
                                    class: "bg-gray-700 px-2 py-2 rounded-md"
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    onclick: move |_| send_invitation_code(()),
                                    button {
                                        class: "px-2 py-1 
                                        flex items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                        "type": "button",
                                        "test-id"  : "profile-invitation-code-confirm",
                                        i { class: "fa fa-chevron-right" }
                                        " Enter"
                                    }
                                }
                            }
                        } else {
                            if email_flag() {
                                div { class: "flex flex-col gap-2",
                                    label { class: "text-gray-300 text-sm flex items-center gap-2",
                                        i { class: "fa fa-envelope" }
                                        " Email"
                                    }
                                    input {
                                        "type": "email",
                                        value: email(),
                                        "test-id" : "profile-email-change-input",
                                        oninput: move |e| email.set(e.value()),
                                        placeholder: "Enter email...",
                                        class: "bg-gray-700 px-2 py-2 rounded-md"
                                    }
                                    div {
                                        class: "flex items-center gap-2",
                                        onclick: move |_| email_change(()),
                                        button {
                                            class: "px-2 py-1 flex items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                            "type": "button",
                                            "test-id"  : "profile-email-change-confirm",
                                            i { class: "fa fa-chevron-right" }
                                            " Enter"
                                        }
                                        button {
                                            onclick: move |_| email_flag.set(false),
                                            class: "px-2 py-1 flex items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                            i { class: "fa fa-times" }
                                            " Cancel"
                                        }
                                    }
                                }
                            } else {
                                div { class: "flex flex-col gap-2",
                                    label { class: "text-gray-300 text-sm flex items-center gap-2",
                                        i { class: "fa fa-envelope" }
                                        " Email"
                                    }
                                    input {
                                        "type": "email",
                                        value: userInfo().email,
                                        disabled: true,
                                        placeholder: "Enter email...",
                                        class: "bg-gray-700 px-2 py-2 rounded-md disabled:opacity-75 disabled"
                                    }
                                    div { class: "flex items-center gap-2",
                                        button {
                                            onclick: move |_| email_flag.set(true),
                                            "test-id"  : "profile-email-change-button",
                                            class: "px-2 py-1 flex items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                            i { class: "fa fa-chevron-right" }
                                            "Reset"
                                        }
                                    }
                                }
                            }
                        }
                        
                        if password_flag() {
                            div { class: "flex flex-col gap-2",
                                label { class: "text-gray-300 text-sm flex items-center gap-2",
                                    i { class: "fa fa-eye" }
                                    " Password"
                                }
                                input {
                                    "type": "password",
                                    value: password(),
                                    "test-id" : "profile-password-change-input",
                                    oninput: move |e| password.set(e.value()),
                                    placeholder: "Enter password...",
                                    class: "bg-gray-700 px-2 py-2 rounded-md"
                                }
                                div { class: "flex items-center gap-2",
                                    button {
                                        class: "px-2 py-1 flex items items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                        "type": "button",
                                        "test-id"  : "profile-password-change-confirm",
                                        onclick: move |_| password_change(()),
                                        i { class: "fa fa-chevron-right" }
                                        " Enter"
                                    }
                                    button {
                                        onclick: move |_| password_flag.set(false),
                                        class: "px-2 py-1 flex items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                        i { class: "fa fa-times" }
                                        " Cancel"
                                    }
                                }
                            }
                        } else {
                            div { class: "flex flex-col gap-2",
                                label { class: "text-gray-300 text-sm flex items-center gap-2",
                                    i { class: "fa fa-eye" }
                                    " Password"
                                }
                                input {
                                    "type": "password",
                                    value: String::from(""),
                                    disabled: true,
                                    placeholder: "Enter password...",
                                    class: "bg-gray-700 px-2 py-2 rounded-md"
                                }
                                div { class: "flex items-center gap-2",
                                    button {
                                        onclick: move |_| password_flag.set(true),
                                        "test-id"  : "profile-password-change-button",
                                        class: "px-2 py-1 flex items items-center gap-2 text-sm border border-white rounded-md hover:bg-white hover:text-black transition-all duration-500",
                                        i { class: "fa fa-chevron-right" }
                                        "Reset"
                                    }
                                }
                            }
                        }
                        
                        div { class: "flex flex-col gap-2",
                            label { class: "text-gray-300 text-sm flex items-center gap-2",
                                i { class: "fa fa-user" }
                                "Role"
                            }
                            div {
                                class: "flex flex-col md:flex-row gap-4",
                                if userInfo().voter {
                                    div {
                                        class: "flex items-center",
                                        i {
                                            class: "fas fa-check-square text-green-500",
                                        }
                                        label {
                                            class: "ms-2 text-sm font-medium text-gray-300",
                                            "Voter"
                                        }
                                    }
                                } else {
                                    div {
                                        class: "flex items-center",
                                        i {
                                            class: "fas fa-times-circle text-red-500",
                                        }
                                        label {
                                            class: "ms-2 text-sm font-medium text-gray-300",
                                            "Voter"
                                        }
                                    }
                                }
                                if userInfo().battler {
                                    div {
                                        class: "flex items-center",
                                        i {
                                            class: "fas fa-check-square text-green-500",
                                        }
                                        label {
                                            class: "ms-2 text-sm font-medium text-gray-300",
                                            "Battler"
                                        }
                                    }
                                } else {
                                    div {
                                        class: "flex items-center",
                                        i {
                                            class: "fas fa-times-circle text-red-500",
                                        }
                                        label {
                                            class: "ms-2 text-sm font-medium text-gray-300",
                                            "Battler"
                                        }
                                    }
                                }
                                if userInfo().judger {
                                    div {
                                        class: "flex items-center",
                                        i {
                                            class: "fas fa-check-square text-green-500",
                                        }
                                        label {
                                            class: "ms-2 text-sm font-medium text-gray-300",
                                            "Judge"
                                        }
                                    }
                                } else {
                                    div {
                                        class: "flex items-center",
                                        i {
                                            class: "fas fa-times-circle text-red-500",
                                        }
                                        label {
                                            class: "ms-2 text-sm font-medium text-gray-300",
                                            "Judge"
                                        }
                                    }
                                }
                                if userInfo().admin {
                                    div {
                                        class: "flex items-center",
                                        i {
                                            class: "fas fa-check-square text-green-500",
                                        }
                                        label {
                                            class: "ms-2 text-sm font-medium text-gray-300",
                                            "Admin"
                                        }
                                    }
                                } else {
                                    div {
                                        class: "flex items-center",
                                        i {
                                            class: "fas fa-times-circle text-red-500",
                                        }
                                        label {
                                            class: "ms-2 text-sm font-medium text-gray-300",
                                            "Admin"
                                        }
                                    }
                                }
                            }
                            
                        }
                        div { class: "flex items-center gap-2",
                            button {
                                class: "flex w-1/2 sm:w-full sm:flex-1 items-center justify-center gap-2 bg-pink-400 px-4 py-2 rounded-md",
                                onclick: move |_| enable_instagram(()),
                                i { class: "fa fa-brands fa-instagram" }
                                "Link Instagram"
                            }
                            if userInfo().google_email.is_empty() {
                                button {
                                    class: "flex w-1/2 sm:w-full sm:flex-1 items-center justify-center gap-2 bg-red-500 px-4 py-2 rounded-md",
                                    onclick: move |_| enable_google(()),
                                    "test-id" : "profile-enable-google-button",
                                    "type": "button",
                                    i { class: "fa  fa-brands fa-google" }
                                    "Enable Google"
                                }
                            } else {
                                button {
                                    class: "flex w-1/2 sm:w-full sm:flex-1 items-center justify-center gap-2 bg-red-500 px-4 py-2 rounded-md",
                                    onclick: move |_| enable_google(()),
                                    "type": "button",
                                    i { class: "fa  fa-brands fa-google" }
                                    "Enabled Already"
                                }
                            }
                        }
                        div { class: "flex flex-col gap-3" }
                    }
                }
            }
        }
    }
}

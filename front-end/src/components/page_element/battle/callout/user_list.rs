use crate::components::element::pagination::Pagination;
use crate::components::element::user_card::UserCard;
use crate::config::SERVER_URL;
use crate::pages::battle::callout::UserSelect;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::utils::request::request_without_recaptcha;
use crate::utils::storage::get_local_storage;
use crate::utils::ErrResModel;
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserListResModel {
    pub data: Vec<UserSelect>,
    pub max_pages: i32,
    pub battler_check: bool,
}

#[component]
pub fn UserList(
    warning: Signal<String>,
    page_flag: Signal<i32>,
    selected_user: Signal<Option<UserSelect>>,
    auth_flag: bool,
) -> Element {
    let mut user_list = use_signal(|| Vec::<UserSelect>::new());
    let mut search_text = use_signal(|| String::from(""));
    let mut pagination = use_signal(|| 1);
    let mut max_page = use_signal(|| 1);
    let user_show_count = use_signal(|| 5);
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();
    let mut battler_check = use_signal(|| false);

    // Use Effect will be occur when search and pagination changed
    use_effect(move || {
        let _ = pagination.read();
        to_owned![pagination, user_show_count, search_text];
        spawn(async move {
            match request_without_recaptcha(
                "post",
                format!("{}/api/v0/battle/callout/get-user-list", SERVER_URL).as_str(),
                json!({
                    "search" : search_text(),
                    "count" : user_show_count(),
                     "pagination" : pagination(),
                     "token" : match get_local_storage("token"){
                            Some(token)=> token,
                            None => "".to_string()
                        }
                }),
                false,
            )
            .await
            {
                Ok(res) => {
                    if res.status() != StatusCode::OK {
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
                                        content: "Response is not correct".to_string(),
                                        notification_type: NotificationType::Error,
                                    });
                                    data // Return the updated data
                                });
                            }
                        }
                    } else {
                        match res.json::<GetUserListResModel>().await {
                            Ok(results) => {
                                user_list.set(results.data);
                                battler_check.set(results.battler_check);
                                max_page.set(results.max_pages);
                            }
                            Err(_) => {
                                notification_data.set({
                                    let mut data = notification_data().clone(); // Clone existing data
                                    data.push(NotificationData {
                                        title: "".to_string(),
                                        content: "Response is not correct".to_string(),
                                        notification_type: NotificationType::Error,
                                    });
                                    data // Return the updated data
                                });
                            }
                        }
                    }
                }
                Err(_) => {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Check you internet".to_string(),
                            notification_type: NotificationType::Error,
                        });
                        data // Return the updated data
                    });
                }
            };
        });
    });

    // Go to aim and setup some signals
    let mut selected = move |user: UserSelect| {
        if battler_check() {
            selected_user.set(Some(user));
            page_flag.set(1);
        }
    };

    rsx! {
        // ----------- Search part ---------------//
        div {
            class: "mb-10",
            div {
                class: "flex items-center justify-center",
                form {
                    class: "w-full flex gap-2 items-center justify-center",
                    input {
                        "type": "text",
                        placeholder: "Who's it gonna be?",
                        class: "px-2 h-10 bg-gray-800 rounded-md w-1/2",
                        oninput : move |e| {
                            search_text.set(e.value());
                            pagination.set(1);
                        }
                    }
                    button {
                        class: "rounded-md w-10 h-10 flex items-center justify-center bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                        "type" : "button",
                        onclick : move |_| {
                            search_text.set("".to_string());
                            pagination.set(1);
                        },
                        i {
                            class: "fa-solid fa-list",
                        }
                    }
                }
            }
        }
        // ------------- End Search Part --------------- //
        // ------------- Start User List Part ---------------- //
        div {
            class: "user-grid px-5 ",
            style: "overflow-y: scroll",
            //  ------------------ Here ------------------------ //
            {
                user_list()
        .into_iter()
        .map(|user| {
            rsx!(
                UserCard {user : user.clone(), onclick : move |user : UserSelect| selected(user.clone())})
        })
            },
            // -------------------- End ------------------------//
        }
        // ------------- End User List Part ---------------- //

        // ------------- Start Pagination List Part ---------------- //
        Pagination {pagination : pagination.clone(), max_page : max_page.clone()}
        // ------------- End Pagination List Part ---------------- //

    }
}

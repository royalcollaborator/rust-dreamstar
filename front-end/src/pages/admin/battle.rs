use dioxus::prelude::*;

use crate::config::SERVER_URL;
use crate::pages::layout::layout::SharedData;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::router::router::Route;
use crate::utils::request::send_request;
use crate::utils::util::go_unauthorized;
use crate::utils::ErrResModel;

use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AdminSelectMatch {
    pub match_id: String,
    pub a_camp_username: String,
    pub b_camp_username: String,
    pub rules: String,
    pub responder_reply: String,
    pub a_video: String,
    pub a_img: String,
    pub b_video: String,
    pub b_img: String,
    pub a_verify: bool,
    pub b_verify: bool,
    pub voting_period: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminBattleListRes {
    pub data: Vec<AdminSelectMatch>,
}

#[component]
pub fn AdminBattle() -> Element {
    let mut navigation = use_navigator();
    let mut component_load = use_signal(|| true);
    let mut battle_list = use_signal(|| Vec::<AdminSelectMatch>::new());
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
    // When component render, get the all matches list that need verification
    use_effect(move || {
        let _ = component_load.read();
        spawn(async move {
            show_loading();
            match send_request(
                "get",
                format!("{}/admin/api/v0/battle/get-battle-list", SERVER_URL).as_str(),
                json!({}),
                true,
                "getAdminBattle",
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
                                exit_loading();
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
                                exit_loading();
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
                    match res.json::<AdminBattleListRes>().await {
                        Ok(res) => {
                            exit_loading();
                            battle_list.set(res.data);
                        }
                        Err(e) => {
                            exit_loading();
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
                }
                Err(_) => {
                    exit_loading();
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
        });
    });

    // Send a_camp verify request to server
    let a_camp_verify = move |match_id : String| async move {
        show_loading();
        match send_request(
            "post",
            format!("{}/admin/api/v0/battle/callout-setup", SERVER_URL).as_str(),
            json!({
                "match_id" : match_id.to_string()
            }),
            true,
            "aCampSetup",
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
                            exit_loading();
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
                            exit_loading();
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
                        content: format!("Verify Success"),
                        notification_type: NotificationType::Success,
                    });
                    component_load.set(!component_load());
                    data // Return the updated data
                });
                exit_loading();
            }
            Err(_) => {
                exit_loading();
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

        // Send a_camp verify request to server
        let b_camp_verify = move |match_id : String| async move {
            show_loading();
            match send_request(
                "post",
                format!("{}/admin/api/v0/battle/reply-setup", SERVER_URL).as_str(),
                json!({
                    "match_id" : match_id.to_string()
                }),
                true,
                "aCampSetup",
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
                                exit_loading();
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
                                exit_loading();
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
                            content: format!("Verify Success"),
                            notification_type: NotificationType::Success,
                        });
                        component_load.set(!component_load());
                        data // Return the updated data
                    });
                    exit_loading();
                }
                Err(_) => {
                    exit_loading();
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
                class: "flex flex-col",
                div {
                    class: "-m-1.5 overflow-x-auto",
                    div {
                        class: "p-1.5 min-w-full inline-block align-middle",
                        div {
                            class: "overflow-hidden",
                            table {
                                class: "min-w-full divide-y divide-gray-200",
                                thead {
                                    tr {
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "a_camp_username"
                                        }
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "a_camp_img"
                                        }
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "a_camp_video"
                                        }
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "a_camp_verify"
                                        }
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "b_camp_username"
                                        }
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "b_camp_img"
                                        }
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "b_camp_video"
                                        }
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "responder_reply"
                                        }
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "b_camp_verify"
                                        }
                                        th {
                                            class: "px-6 py-3 text-start font-medium text-gray-100",
                                            "voting_duration"
                                        }
                                    }
                                }
                                tbody {
                                    class: "divide-y divide-gray-200",
                                    {
                                        battle_list().into_iter().map(move |battle| {
                                            let match_id_clone = battle.match_id.clone();
                                            rsx!{
                                                tr {
                                                    class: "hover:bg-gray-900",
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        "{battle.a_camp_username}"
                                                    }
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        a {
                                                            href: "{battle.a_img}",
                                                            target: "_blank",
                                                            rel: "noopener noreferrer",
                                                            class: "text-blue-400",
                                                            "View Image"
                                                        }
                                                    }
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        a {
                                                            href: "{battle.a_video}",
                                                            target: "_blank",
                                                            rel: "noopener noreferrer",
                                                            class: "text-blue-400",
                                                            "View Video"
                                                        }
                                                    }
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        if !battle.a_verify {
                                                            button {
                                                                class: "rounded-md px-3 w-full h-10 flex items-center justify-center gap-1 bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                                                                onclick : move |_| a_camp_verify(battle.match_id.to_string()),
                                                                "A Verify"
                                                            }
                                                        }
                                                    }
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        "{battle.b_camp_username}"
                                                    }
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        if !battle.b_img.is_empty() {
                                                            a {
                                                                href: "{battle.b_img}",
                                                                target: "_blank",
                                                                rel: "noopener noreferrer",
                                                                class: "text-blue-400",
                                                                "View Image"
                                                            }
                                                        }
                                                    }
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        if !battle.b_video.is_empty() {
                                                            a {
                                                                href: "{battle.b_video}",
                                                                target: "_blank",
                                                                rel: "noopener noreferrer",
                                                                class: "text-blue-400",
                                                                "View Video"
                                                            }
                                                        }
                                                    }
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        if !battle.responder_reply.is_empty() {
                                                            "\"{battle.responder_reply}\""
                                                        }
                                                    }
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        if !battle.b_video.is_empty() && battle.a_verify{
                                                            button {
                                                                class: "rounded-md px-3 w-full h-10 flex items-center justify-center gap-1 bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                                                                onclick : move |_| b_camp_verify(match_id_clone.to_string()),
                                                                "B Verify"
                                                            }
                                                        }
                                                    }
                                                    td {
                                                        class: "px-6 py-4 text-wrap text-gray-200",
                                                        "{battle.voting_period}"
                                                    }
                                                }

                                            }
                                        })
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

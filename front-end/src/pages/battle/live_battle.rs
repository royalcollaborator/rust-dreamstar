use crate::config::{HOST_URL, SERVER_URL};
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
use crate::utils::js_binding::copy_clipboard;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiveBattleSetupRes {
    pub result: Vec<String>,
    id: String,
}

#[component]
pub fn LiveBattle() -> Element {
    let navigation = use_navigator();
    let mut rules = use_signal(|| {
        String::from(
            "Rules: Different songs. We choose our songs. No time limit. Freestyle or choreography. Any props. All styles. Post whenever. No extra rules."
        )
    });
    let mut judge1 = use_signal(|| String::from(""));
    let mut judge2 = use_signal(|| String::from(""));
    let mut judge3 = use_signal(|| String::from(""));
    let mut judge4 = use_signal(|| String::from(""));
    let mut judge5 = use_signal(|| String::from(""));
    let mut a_camp = use_signal(|| String::from(""));
    let mut b_camp = use_signal(|| String::from(""));
    let mut event_code = use_signal(|| String::from(""));
    let mut created_flag = use_signal(|| String::from(""));
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

    // Create live battle
    let create_live_battle = move || async move {
        let mut set = std::collections::HashSet::new();
        let mut length = 0;
        if !judge1().is_empty() {
            set.insert(judge1());
            length += 1;
        }
        if !judge2().is_empty() {
            set.insert(judge2());
            length += 1;
        }
        if !judge3().is_empty() {
            set.insert(judge3());
            length += 1;
        }
        if !judge4().is_empty() {
            set.insert(judge4());
            length += 1;
        }
        if !judge5().is_empty() {
            set.insert(judge5());
            length += 1;
        }

        if set.len() != length && length != 0 {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Username must be uniqe".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        show_loading();
        match send_request(
            "post",
            format!("{}/api/v0/battle/live-battle/live-battle-setup", SERVER_URL).as_str(),
            json!({
               "a_1" : judge1(),
               "a_2" : judge2(),
               "a_3" : judge3(),
               "a_4" : judge4(),
               "a_5" : judge5(),
               "statement" : rules(),
               "a_camp" : a_camp(),
               "b_camp" : b_camp()
            }),
            true,
            "liveBattleCreate",
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
                match res.json::<LiveBattleSetupRes>().await {
                    Ok(results) => {
                        if results.result.len() == 0 {
                            exit_loading();
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: "Live Event Created".to_string(),
                                    notification_type: NotificationType::Success,
                                });
                                data // Return the updated data
                            });
                            created_flag.set(results.id.to_string());
                        } else {
                            let mut show = String::new();
                            for val in results.result {
                                show.push_str(&format!("{}, ", val.to_string()));
                            }
                            show.push_str("are not judges");
                            notification_data.set({
                                let mut data = notification_data().clone(); // Clone existing data
                                data.push(NotificationData {
                                    title: "".to_string(),
                                    content: show.to_string(),
                                    notification_type: NotificationType::Error,
                                });
                                data // Return the updated data
                            });
                            exit_loading();
                            return;
                        }
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
                        exit_loading();
                    }
                }
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
                exit_loading();
            }
        }
    };

    // Join Event function
    let join_event = move || async move {
        // Check Event Code
        if event_code().is_empty() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Input Event Code".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        show_loading();
        match send_request(
            "post",
            format!(
                "{}/api/v0/battle/live-battle/live-battle-code-check",
                SERVER_URL
            )
            .as_str(),
            json!({
               "code" :event_code()
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
                navigation.push(Route::LiveBattleShow {
                    code: event_code().to_string(),
                });
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
                class : "mt-10 md:w-1/2 w-[75%] mx-auto bg-gray-800 rounded-md p-5 ",
                h2 {
                    class : "text-xl text-center mb-5",
                    "Join Event"
                }
                div {
                    class : "flex flex-col gap-3",
                    input {
                        "type": "text",
                        "test-id" : "live-battle-join-event-code-input",
                        oninput : move |e| event_code.set(e.value()),
                        placeholder: "Event Code",
                        class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                    }
                }
                button {
                    class: "mt-5 rounded-md px-3 w-full h-10 flex items-center justify-center gap-1 bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                    "test-id" : "live-battle-join-event-button",
                    onclick : move |_| join_event(),
                    i {
                        class: "fa-solid fa-angles-right"
                    }
                    "Join"
                    i {
                        class: "fa-solid fa-angles-left"
                    }
                }
            }
            if created_flag().is_empty() {
                div {
                    class: "mt-10 md:w-1/2 w-[75%] mx-auto bg-gray-800 rounded-md p-5",
                    h2 {
                        class: "text-xl text-center mb-5",
                        "Please input camps"
                    }
                    div {
                        class : "flex flex-col gap-3",
                        input {
                            "type": "text",
                            "test-id" : "live-battle-a-camp-input",
                            oninput : move |e| a_camp.set(e.value()),
                            placeholder: "A Camp",
                            class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                        }
                        input {
                            "type": "text",
                            "test-id" : "live-battle-b-camp-input",
                            oninput : move |e| b_camp.set(e.value()),
                            placeholder: "B Camp",
                            class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                        }
                    }
                    h2 {
                        class: "text-xl text-center mb-5 mt-5",
                        "Select upto 5 judges. Be sure everyone agrees to it."
                    }
                    div {
                        class: "flex flex-col gap-3",
                        input {
                            "type": "text",
                            oninput : move |e| judge1.set(e.value()),
                            placeholder: "Judge 1",
                            class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                        }
                        input {
                            "type": "text",
                            oninput : move |e| judge2.set(e.value()),
                            placeholder: "Judge 2",
                            class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                        }
                        input {
                            "type": "text",
                            oninput : move |e| judge3.set(e.value()),
                            placeholder: "Judge 3",
                            class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                        }
                        input {
                            "type": "text",
                            oninput : move |e| judge4.set(e.value()),
                            placeholder: "Judge 4",
                            class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                        }
                        input {
                            "type": "text",
                            oninput : move |e| judge5.set(e.value()),
                            placeholder: "Judge 5",
                            class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                        }
                    }
                    textarea {
                        class: "text-sm mt-5 px-2 h-[64px] pt-[10px] bg-gray-700 rounded-md w-full w-full",
                        value : rules(),
                        oninput : move |e| rules.set(e.value()),
                    }
                    button {
                        class: "mt-5 rounded-md px-3 w-full h-10 flex items-center justify-center gap-1 bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                        onclick : move |_| create_live_battle(),
                        "test-id" : "live-battle-fire",
                        i {
                            class: "fa-solid fa-angles-right"
                        }
                        "Fire"
                        i {
                            class: "fa-solid fa-angles-left"
                        }
                    }
                }
            } else {
                div {
                    class: "mt-10 md:w-1/2 w-[75%] mx-auto bg-gray-800 rounded-md p-5",
                    h2 {
                        class: "text-xl text-center mb-5 mt-5",
                        "This battle will be closed in 5 minutes"
                    }
                    h2 {
                        class: "text-xl text-center mb-5 mt-5",
                        "Remember this code"
                    }
                    input {
                        "type": "text",
                        "test-id" : "live-battle-generated-code-input",
                        value : "{created_flag}",
                        disabled : false,
                        class: "px-2 h-10 bg-gray-700 rounded-md w-full outline-none"
                    }
                    button {
                        class: "mt-5 rounded-md px-3 w-full h-10 flex items-center justify-center gap-1 bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                        onclick : move |_| {
                            let url = format!("{}/live-battle/{}", HOST_URL, created_flag());
                            let result : bool = copy_clipboard(url.to_string()).into_serde().unwrap();
                            if result {
                                notification_data.set({
                                    let mut data = notification_data().clone(); // Clone existing data
                                    data.push(NotificationData {
                                        title: "".to_string(),
                                        content: "Link Copied".to_string(),
                                        notification_type: NotificationType::Success,
                                    });
                                    data // Return the updated data
                                });
                            } else {
                                notification_data.set({
                                    let mut data = notification_data().clone(); // Clone existing data
                                    data.push(NotificationData {
                                        title: "".to_string(),
                                        content: "Can't Copy".to_string(),
                                        notification_type: NotificationType::Error,
                                    });
                                    data // Return the updated data
                                });
                            }
                        },
                        "Copy URL"
                    }
                }
            }
        }
    }
}

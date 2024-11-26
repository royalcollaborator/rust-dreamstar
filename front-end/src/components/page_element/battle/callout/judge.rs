use crate::config::SERVER_URL;
use crate::pages::layout::layout::SharedData;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::router::router::Route;
use crate::utils::request::send_request;
use crate::utils::util::go_unauthorized;
use crate::utils::ErrResModel;
use crate::{
    pages::battle::callout::UserSelect, utils::js_binding::captureVideoFrame,
    utils::js_binding::uploadFile,
};
use dioxus::prelude::*;
use js_sys::Array;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::JsCast;
use web_sys::{File, Url};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyJudgesResModel {
    data: Vec<String>,
    video_id: String,
    image_id: String,
    video_url: String,
    image_url: String,
}
// Todo I have to make judges optional
#[component]
pub fn Judge(
    page_flag: Signal<i32>,
    selected_user: Signal<Option<UserSelect>>,
    warning: Signal<String>,
    video_content: Signal<String>,
    video_type: Signal<String>,
    image_content: Signal<String>,
    video_content_data: Signal<Option<web_sys::File>>,
) -> Element {
    let navigation = use_navigator();
    let mut shared_data = use_context::<Signal<SharedData>>();
    let mut vir_image_content = use_signal(|| String::from(""));
    let mut vir_image_content_data = use_signal(|| None as Option<web_sys::File>);
    let mut voting_duration = use_signal(|| 24);
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();
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

    // Go to user_list page
    let mut go_to_user_list = move || {
        video_content.set(String::from(""));
        video_type.set(String::from(""));
        image_content.set(String::from(""));
        selected_user.set(None);
        page_flag.set(0);
    };
    // Go to aim page
    let mut go_to_aim = move || {
        video_content.set(String::from(""));
        video_type.set(String::from(""));
        image_content.set(String::from(""));
        page_flag.set(1);
    };

    // Take a video screen
    let capture = move || async move {
        let blob = captureVideoFrame("showed-video").await;
        let blob: web_sys::Blob = blob.dyn_into().unwrap();
        let blob_url = Url::create_object_url_with_blob(&blob).unwrap();
        vir_image_content.set(blob_url.to_string());
        let file_array = Array::new();
        file_array.push(&blob);
        let file = File::new_with_blob_sequence_and_options(
            &file_array,
            "capture.png", // Filename
            web_sys::FilePropertyBag::new() // Options for the file
                .type_("image/png"),
        )
        .unwrap();
        vir_image_content_data.set(Some(file));
    };

    let set_callout =
        move |video_id: String, image_id: String, video_type: String, rules: String| async move {
            show_loading();
            match send_request(
                "post",
                format!("{}/api/v0/battle/callout/set-callout", SERVER_URL).as_str(),
                json!({
                    "a_1" : judge1(),
                   "a_2" : judge2(),
                   "a_3" : judge3(),
                   "a_4" : judge4(),
                   "a_5" : judge5(),
                   "opponent_id" : selected_user().unwrap()._id,
                   "video_id" : video_id,
                   "image_id" : image_id,
                   "video_type" : video_type,
                   "rules" : rules,
                   "voting_duration" : voting_duration(),
                }),
                true,
                "setCallout",
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
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Nice, please wait admin approve your callout request"
                                .to_string(),
                            notification_type: NotificationType::Success,
                        });
                        data // Return the updated data
                    });
                    exit_loading();
                    navigation.push(Route::MainMenu);
                }
                Err(_) => {
                    notification_data.set({
                        let mut data = notification_data().clone(); // Clone existing data
                        data.push(NotificationData {
                            title: "".to_string(),
                            content: "Check internet connection".to_string(),
                            notification_type: NotificationType::Success,
                        });
                        data // Return the updated data
                    });
                    exit_loading();
                }
            }
        };

    let video_image_upload = move |results: VerifyJudgesResModel| async move {
        let video_result: bool = uploadFile(
            results.video_url.to_string(),
            video_content_data().unwrap(),
            format!("video/mp4").to_string(),
        )
        .await
        .into_serde()
        .unwrap();
        let image_result: bool = uploadFile(
            results.image_url.to_string(),
            vir_image_content_data().unwrap(),
            "image/jpeg".to_string(),
        )
        .await
        .into_serde()
        .unwrap();
        if !video_result || !image_result {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Upload failed".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        set_callout(
            results.video_id,
            results.image_id,
            "mp4".to_string(),
            rules().to_string(),
        )
        .await;
    };

    // Callout
    let callout = move || async move {
        if vir_image_content().is_empty() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Please take a screen".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }

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
            format!("{}/api/v0/battle/callout/verify-get-sign-url", SERVER_URL).as_str(),
            json!({
               "a_1" : judge1(),
               "a_2" : judge2(),
               "a_3" : judge3(),
               "a_4" : judge4(),
               "a_5" : judge5(),
               "opponent" : selected_user().unwrap()._id,
            }),
            true,
            "verifyJudges",
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
                match res.json::<VerifyJudgesResModel>().await {
                    Ok(results) => {
                        if results.data.len() == 0 {
                            exit_loading();
                            video_image_upload(results).await;
                        } else {
                            let mut show = String::new();
                            for val in results.data {
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

    // Video uploaded

    rsx! {
         div {
             class: "mb-10",
             div {
                 class: "flex items-center justify-center",
                 form {
                     class: "w-full md:w-1/2 flex flex-col gap-6 mb-2",
                     div {
                         class: "flex item-center md:flex-row item-center justify-between gap-5 md:gap-10 px-5",

                         {
                             match selected_user() {
                                 Some(val)=>{
                                    rsx!(
                                     h1 {
                                         class: "text-4xl font-bold",
                                         "{val.username}"
                                     }
                                    )
                                 }
                                 None=>{
                                     go_to_user_list();
                                    rsx!(
                                     h1{
                                         ""
                                     }
                                    )
                                 }
                             }
                         }
                         div {
                             class: "flex item-center gap-2",
                             button {
                                 class: "rounded-md px-3 h-10 flex items-center justify-center gap-1 bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                                 "type" : "button",
                                 onclick : move |_| {
                                     go_to_aim();
                                 },
                                 i {
                                     class: "fa-solid fa-circle-dot"
                                 }
                                 "Ready"
                             }
                             button {
                                 class: "rounded-md w-10 h-10 flex items-center justify-center bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                                 "type" : "button",
                                 onclick : move |_| go_to_user_list(),
                                 i {
                                     class: "fa-solid fa-list"
                                 }
                             }
                         }
                     }
                 }
             }
             div {
                 class: "bg-gray-800 p-5 rounded-lg md:flex gap-5",
                 div {
                     class: "md:w-1/2 w-full mb-4 flex flex-col item-center justify-center gap-3",
                     h2 {
                         class: "text-lg text-center",
                         "Your Video"
                     }
                     video {
                         class: "w-full aspect-video rounded-md",
                         id : "showed-video",
                         controls: true,
                         src : "{video_content}"
                     }
                     button {
                         class: "mt-2 rounded-md px-3 h-10 flex items-center justify-center gap-1 bg-slate-500 hover:bg-slate-600 transition-all duration-300",
                         "test-id" : "battle-take-picture",
                         onclick : move |_| capture(),
                         i {
                             class: "fa-solid fa-image"
                         }
                         "Take Snapshot"
                     }
                 }
                 div {
                     class: "md:w-1/2 w-full flex flex-col gap-3",
                     h2 {
                         class: "text-lg text-center",
                         "Your Snapshot"
                     }

                         img {
                             src: "{vir_image_content}",
                             alt: "Image Snapshot",
                             class: "aspect-video object-contain rounded-md",
                             id : "showed_image"
                         }
                 }
             }
             div {
                 class: "mt-10 md:w-1/2 w-[75%] mx-auto bg-gray-800 rounded-md p-5",
                 h2 {
                     class: "text-xl text-center mb-5",
                     "Select upto 5 judges. Be sure everyone agrees to it."
                 }
                 div {
                     class: "flex flex-col gap-3",
                     input {
                         "type": "text",
                         oninput : move |e| judge1.set(e.value()),
                         placeholder: "Judge 1",
                         class: "px-2 h-10 bg-gray-700 rounded-md w-full"
                     }
                     input {
                         "type": "text",
                         oninput : move |e| judge2.set(e.value()),
                         placeholder: "Judge 2",
                         class: "px-2 h-10 bg-gray-700 rounded-md w-full"
                     }
                     input {
                         "type": "text",
                         oninput : move |e| judge3.set(e.value()),
                         placeholder: "Judge 3",
                         class: "px-2 h-10 bg-gray-700 rounded-md w-full"
                     }
                     input {
                         "type": "text",
                         oninput : move |e| judge4.set(e.value()),
                         placeholder: "Judge 4",
                         class: "px-2 h-10 bg-gray-700 rounded-md w-full"
                     }
                     input {
                         "type": "text",
                         oninput : move |e| judge5.set(e.value()),
                         placeholder: "Judge 5",
                         class: "px-2 h-10 bg-gray-700 rounded-md w-full"
                     }
                 }
                 textarea {
                     class: "text-sm mt-5 px-2 h-[64px] pt-[10px] bg-gray-700 rounded-md w-full w-full",
                     value : rules(),
                     oninput : move |e| rules.set(e.value()),
                 }
                 h2 {
                    class: "text-xl text-center mb-5 mt-1",
                    "Please select voting duration"
                }
                input {
                    "type": "number",
                    "max" : "720",
                    "min": "24",
                    oninput : move |e| {
                        if let Ok(val) = e.value().parse::<i32>() {
                            if val <=24{
                                voting_duration.set(24);
                            } else if val >= 720 {
                                voting_duration.set(720)
                            } else {
                                voting_duration.set(val);
                            }
                        }
                    },
                    value : "{voting_duration}",
                    class: "px-2 h-10 bg-gray-700 rounded-md w-full"
                }
                 button {
                     class: "mt-5 rounded-md px-3 w-full h-10 flex items-center justify-center gap-1 bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                     "test-id" : "battle-fire",
                     onclick : move |_| callout(),
                     i {
                         class: "fa-solid fa-angles-right"
                     }
                     "Fire"
                     i {
                         class: "fa-solid fa-angles-left"
                     }
                 }
             }

         }
    }
}

use crate::pages::layout::layout::{NotificationData, NotificationType};
use dioxus::prelude::*;
use js_sys::Array;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::JsCast;
use web_sys::File;
use web_sys::Url;

use super::canvas::Canvas;
use crate::config::SERVER_URL;
use crate::pages::battle::matchs::{SelectedMatch, VotingType};
use crate::pages::layout::layout::SharedData;
use crate::router::router::Route;
use crate::utils::request::send_request;
use crate::utils::util::go_unauthorized;
use crate::utils::ErrResModel;
use crate::{utils::js_binding::captureCanvasImg, utils::js_binding::uploadFile};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSignURLResModel {
    pub img_id: String,
    pub img_sign_url: String,
}

#[component]
pub fn Vote(voting_type: Signal<VotingType>, battle_information: SelectedMatch) -> Element {
    let mut shared_data = use_context::<Signal<SharedData>>();
    let navigation = use_navigator();
    let mut range_value = use_signal(|| 50);
    let mut statement = use_signal(|| String::from("I score it 50 to 50 because..."));
    let mut statement_len = use_signal(|| 30);
    let selected_voting_type = use_signal(|| match voting_type() {
        VotingType::Not => 3,
        VotingType::Judge => 2,
        VotingType::Official => 1,
        VotingType::Unofficial => 0,
    });
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();
    let select_voting_type = use_memo(move || match voting_type() {
        VotingType::Not => 3,
        VotingType::Judge => 2,
        VotingType::Official => 1,
        VotingType::Unofficial => 0,
    });

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

    // Set voting
    let set_voting = move |match_id: String,
                           image_id: String,
                           vote_type: i32,
                           a_vote_count: i32,
                           b_vote_count: i32,
                           statement: String| async move {
        show_loading();
        match send_request(
            "post",
            format!("{}/api/v0/vote/set-vote", SERVER_URL).as_str(),
            json!({
                "match_id": match_id.to_string(),
                "a_camp_votes": a_vote_count,
                "b_camp_votes": b_vote_count,
                "vote_type": vote_type,
                "statement": statement.to_string(),
                "img_id" : image_id.to_string()
            }),
            true,
            "signImage",
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
                    if res.status() == StatusCode::FORBIDDEN {
                        notification_data.set({
                            let mut data = notification_data().clone(); // Clone existing data
                            data.push(NotificationData {
                                title: "".to_string(),
                                content: "Please try once more if you are not robot".to_string(),
                                notification_type: NotificationType::Error,
                            });
                            data // Return the updated data
                        });
                        exit_loading();
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
                exit_loading();
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Voting Success".to_string(),
                        notification_type: NotificationType::Success,
                    });
                    data // Return the updated data
                });
                voting_type.set(VotingType::Not);
            }
            Err(_) => {
                notification_data.set({
                    let mut data = notification_data().clone(); // Clone existing data
                    data.push(NotificationData {
                        title: "".to_string(),
                        content: "Please check internet connection".to_string(),
                        notification_type: NotificationType::Error,
                    });
                    data // Return the updated data
                });
            }
        }
    };

    // Upload sign image
    let sing_img_upload = move |img_url: String,
                                img_id: String,
                                img_content: File,
                                match_id: String,
                                a_vote_count: i32,
                                b_vote_count: i32,
                                statement: String| async move {
        let image_result: bool =
            uploadFile(img_url.to_string(), img_content, "image/jpeg".to_string())
                .await
                .into_serde()
                .unwrap();
        if !image_result {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Please upload correct image".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        set_voting(
            match_id,
            img_id,
            selected_voting_type(),
            a_vote_count,
            b_vote_count,
            statement,
        )
        .await;
    };

    // Submit the vote
    // This function captures the image, validates the statement, and sends the vote request to the server.
    // It handles the response and updates the UI accordingly.
    let voting_submit = move |battle_information: SelectedMatch| async move {
        // Capture image
        let js_blob = captureCanvasImg("drawPlace").await;
        let blob: web_sys::Blob = js_blob.dyn_into().unwrap();
        // Get the blob string url.
        let blob_url = Url::create_object_url_with_blob(&blob).unwrap();
        web_sys::console::log_1(&format!("{}", blob_url.to_string()).to_string().into());
        // Get the File type
        let file_array = Array::new();
        file_array.push(&blob);
        let file = File::new_with_blob_sequence_and_options(
            &file_array,
            "capture.png", // Filename
            web_sys::FilePropertyBag::new() // Options for the file
                .type_("image/png"),
        )
        .unwrap();

        // Check image capture
        if blob_url.is_empty() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Can't capture sign image".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        // Check statement
        if statement().is_empty() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "Please input statement".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
            return;
        }
        // image_content.set(blob_url.to_string());
        let a_vote_count = 100 - range_value();
        let b_vote_count = range_value();
        show_loading();
        // Send request
        match send_request(
            "post",
            format!("{}/api/v0/vote/get-sign-img-url", SERVER_URL).as_str(),
            json!({
                "match_id": battle_information.match_id.to_string(),
                "a_camp_votes": a_vote_count,
                "b_camp_votes": b_vote_count,
                "vote_type": selected_voting_type(),
                "statement": statement().to_string(),
            }),
            true,
            "signImage",
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
                    if res.status() == StatusCode::FORBIDDEN {
                        notification_data.set({
                            let mut data = notification_data().clone(); // Clone existing data
                            data.push(NotificationData {
                                title: "".to_string(),
                                content: "Please try once more if you are not robot".to_string(),
                                notification_type: NotificationType::Error,
                            });
                            data // Return the updated data
                        });
                        exit_loading();
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
                match res.json::<GetSignURLResModel>().await {
                    Ok(results) => {
                        exit_loading();
                        sing_img_upload(
                            results.img_sign_url.to_string(),
                            results.img_id.to_string(),
                            file,
                            battle_information.match_id.to_string(),
                            a_vote_count,
                            b_vote_count,
                            statement().to_string(),
                        )
                        .await;
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

    if voting_type() == VotingType::Not {
        rsx!(
            div {""}
        )
    } else {
        rsx!(
             div {class : "my-16", ""}
             div {
                 class: "w-full flex flex-col items-center gap-10 md:p-2",
                 div {
                     class: "w-full flex item-center justify-center",
                     if select_voting_type() == 1 {
                         div {
                             class: "w-full md:w-1/2 mx-auto", 
                             div {
                                 class: "flex items-center gap-4 mb-5 justify-center text-[2em]",
                                 i {
                                     class: "fas fa-certificate  text-yellow-400 text-2xl",
                                 }
                                 "Official Vote",
                                 i {
                                     class: "fas fa-certificate  text-yellow-400 text-2xl",
                                 }
                             }
                         }
                     } else if select_voting_type() == 2{

                         h2 {
                             class: "text-center text-4xl font-bold flex items-center gap-4 mb-5 justify-center text-[2em]",
                             i {
                                 class : "fas fa-gavel text-yellow-400 text-2xl"
                             }
                             "Judge Vote",
                             i {
                                 class : "fas fa-gavel text-yellow-400 text-2xl"
                             }
                         }
                     } else if select_voting_type() == 0 {
                         h2 {
                             class: "text-center text-4xl font-bold flex items-center gap-4 mb-5 text-gray-500",
                             i {
                                 class : "text-yellow-400 text-2xl"
                             }
                             "Unofficial Vote",
                             i {
                                 class : "text-yellow-400 text-2xl"
                             }
                         }
                     } else {
                         {navigation.push(Route::Login);}
                     }
                 }
                 div {
                     class: "w-full",
                     section {
                         class: "w-full flex flex-col items-center gap-4",
                         input {
                             "type": "range",
                             value: "{range_value}",
                             oninput : move |e| {
                                 if let Ok(val) = e.value().parse::<i32>() {
                                     if val <=0{
                                         range_value.set(0);
                                         statement.set(format!("I score it {:?} to {:?} because...", 100,0));
                                     } else if val >= 100 {
                                         range_value.set(100);
                                         statement.set(format!("I score it {:?} to {:?} because...", 0,100));
                                     } else {
                                         range_value.set(val);
                                         statement.set(format!("I score it {:?} to {:?} because...", (100-val) as i32, val));
                                     }
                                 }
                             },
                             class: "w-full appearance-none bg-transparent",
                         }
                     }
                 }
                 div {
                     class: "w-full flex items-center justify-between gap-5 md:gap-10",
                     div {
                         class: "bg-gray-900  transition-all duration-500 cursor-pointer rounded-xl p-3 w-full md:w-1/2 flex flex-col items-center gap-2",
                         h1 { "{battle_information.a_camp_username}" }
                         h1 {
                             class: "text-6xl font-bold",
                             "{(100 - range_value()).to_string()}"
                         }
                         button {
                             class: "text-2xl w-full rounded-xl  py-2 px-10 bg-gray-800 hover:bg-gray-700 transition-all duration-500",
                             "test-id" : "vote-score-button",
                             onclick : move |_|{
                                 let val = range_value() - 1;
                                 if val <=0{
                                     range_value.set(0);
                                     statement.set(format!("I score it {:?} to {:?} because...", 100,0));
                                 } else if val >= 100 {
                                     range_value.set(100);
                                     statement.set(format!("I score it {:?} to {:?} because...", 0,100));
                                 } else {
                                     range_value.set(val);
                                     statement.set(format!("I score it {:?} to {:?} because...", (100-val) as i32, val));
                                 }
                             },
                             i {
                                 class: "fas fa-arrow-left",
                             }
                         }
                     }
                     div {
                         class: "bg-gray-900  transition-all duration-500 cursor-pointer rounded-xl p-3 w-full md:w-1/2 flex flex-col items-center gap-2",
                         h1 { "{battle_information.b_camp_username}" }
                         h1 {
                             class: "text-6xl font-bold",
                             "{range_value().to_string()}"
                         }
                         button {
                             class: "text-2xl w-full rounded-xl  py-2 px-10 bg-gray-800 hover:bg-gray-700 transition-all duration-500",
                             onclick : move |_|{
                                 let val = range_value() + 1;
                                 if val <=0{
                                     range_value.set(0);
                                     statement.set(format!("I score it {:?} to {:?} because...", 100,0));
                                 } else if val >= 100 {
                                     range_value.set(100);
                                     statement.set(format!("I score it {:?} to {:?} because...", 0,100));
                                 } else {
                                     range_value.set(val);
                                     statement.set(format!("I score it {:?} to {:?} because...", (100-val) as i32, val));
                                 }
                             },
                             i {
                                 class: "fas fa-arrow-right",
                             }
                         }
                     }
                 }
                 div {
                     class: "w-full flex flex-col md:flex-row gap-5 md:gap-10",
                     div {
                         class : if select_voting_type() == 0{
                             "w-full flex flex-col items-center gap-2"
                         } else {
                             "w-full md:w-1/2 flex flex-col items-center gap-2"
                         },
                         textarea {
                             class: "w-full rounded-xl border-4 border-gray-700 bg-transparent p-2 outline-none",
                             rows: "8",
                             value : statement,
                             "maxlength" : "200",
                             oninput : move |e| {
                                 let val : String = e.value().to_string();
                                 if val.len() > 200 {
                                     statement_len.set(200);
                                     return;
                                 }
                                 statement_len.set(val.len());
                                 statement.set(val.to_string())
                             },
                         }
                         small {
                             class: "text-center",
                             "{statement_len}/200"
                         }
                     }
                     if select_voting_type() != 0 {
                         Canvas {}
                     }
                 }
                 if select_voting_type == 0{
                     button {
                         class: "w-full md:w-1/2 bg-gray-800 hover:bg-gray-700 transition-all duration-500 py-4 rounded-full",
                         "test-id" : "vote-trigger-button",
                         onclick : move |_| set_voting(
                             battle_information.match_id.to_string(),
                             "".to_string(),
                             0,
                             100 - range_value(),
                             range_value(),
                             statement()
                         ),
                         "VOTE"
                     }
                 } else {
                     button {
                         class: "w-full md:w-1/2 bg-gray-800 hover:bg-gray-700 transition-all duration-500 py-4 rounded-full",
                         "test-id" : "vote-trigger-button",
                         onclick : move |_| voting_submit(battle_information.clone()),
                         "VOTE"
                     }
                 }
             }
             script {
                src : "/static/canvas.js"
             }
        )
    }
}

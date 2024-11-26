use crate::components::element::user_card::UserCard;
use crate::pages::battle::callout::UserSelect;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use dioxus::{prelude::*, web::WebFileEngineExt};
use web_sys::Url;

#[component]
pub fn Aim(
    page_flag: Signal<i32>,
    selected_user: Signal<Option<UserSelect>>,
    warning: Signal<String>,
    video_content: Signal<String>,
    video_type: Signal<String>,
    video_content_data: Signal<Option<web_sys::File>>,
) -> Element {
    let mut go_to_userlist_page = move || {
        page_flag.set(0);
        selected_user.set(None);
        video_content.set(String::from(""));
        video_type.set(String::from(""));
    };
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();

    let mut next = move || {
        page_flag.set(2);
    };

    rsx! {
        div {
            // ------------------- Start Select video function -------------------- //
            class: "mb-10",
            div {
                class: "flex items-center justify-center",
                form {
                    class: "w-full md:w-1/2 flex flex-col gap-6",
                    div {
                        class: "flex item-center md:flex-row item-center justify-between gap-5 md:gap-10 px-5",
                            // "{selected_user().unwrap().username}"
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
                                    video_content.set(String::from(""));
                                    video_type.set(String::from(""));
                                },
                                i {
                                    class: "fa-solid fa-circle-dot",
                                },
                                "Ready"
                            }
                            button {
                                class: "rounded-md w-10 h-10 flex items-center justify-center bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                                "type" : "button",
                                onclick : move |_| go_to_userlist_page(),
                                i {
                                    class: "fa-solid fa-list",
                                }
                            }
                        }
                    }
                    div {
                        class: "flex items-center justify-center w-full",
                        label {
                            class: "flex flex-col items-center justify-center w-full h-50 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600 transition-all duration-300",
                            div {
                                class: "flex flex-col items-center justify-center pt-5 pb-6",
                                svg {
                                    class: "w-8 h-8 mb-4 text-gray-500 dark:text-gray-400 ",
                                    "aria_hidden": true,
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    "viewBox": "0 0 20 16",
                                    path {
                                        stroke: "currentColor",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"
                                    }
                                }
                                p {
                                    class: "mb-2 text-sm text-gray-500 dark:text-gray-400",
                                    span {
                                        class: "font-semibold",
                                        "Click to upload"
                                    }
                                    " or drag and drop"
                                }
                                    p {
                                        class: "text-xs text-gray-500 dark:text-gray-400",
                                        {if video_content().is_empty(){ "MP4 only"} else {"File selected"}}
                                    }
                            }
                            input {
                                id: "dropzone-file",
                                "type": "file",
                                accept: ".mp4",
                                class: "hidden",
                                onchange: move |evt : FormEvent| async move {
                                    if let Some(file_engine) = evt.files() {
                                        let files: Vec<String> = file_engine.files();
                                        for file_name in &files {
                                            let file_ext = file_name.split('.').last().unwrap_or_default().to_lowercase();
                                            if !["mp4"].contains(&file_ext.as_str()) {
                                                notification_data.set({
                                                    let mut data = notification_data().clone(); // Clone existing data
                                                    data.push(NotificationData {
                                                        title: "".to_string(),
                                                        content: "Select Correct file".to_string(),
                                                        notification_type: NotificationType::Error,
                                                    });
                                                    data // Return the updated data
                                                });
                                                return
                                            }
                                            if let Some(file) = file_engine.get_web_file(&file_name.to_string()).await {
                                                let blob_url = Url::create_object_url_with_blob(&file).unwrap();
                                                video_content_data.set(Some(file.clone()));
                                                video_content.set(blob_url);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // file upload
                    if video_content().is_empty(){
                        button {
                            class: "rounded-md px-3 h-10 flex items-center justify-center gap-1 bg-blue-500 hover:bg-blue-600 transition-all duration-300 opacity-[0.5]",
                            disabled : true,
                            "type" : "button",
                            i {
                                class: "fa-solid fa-crosshairs",
                            }
                            "Aim"
                        }
                    } else {
                        button {
                            class: "rounded-md px-3 h-10 flex items-center justify-center gap-1 bg-blue-500 hover:bg-blue-600 transition-all duration-300",
                            "test-id"  : "battle-aim",
                            "type" : "button",
                            onclick : move |_| next(),
                            i {
                                class: "fa-solid fa-crosshairs",
                            }
                            "Aim"
                        }
                    }

                }
            }
            // --------------------- End -------------------------------//
            // --------------------- User --------------------------//
            div {
                class: "user-grid px-5 mt-[5rem] ",
                //  ------------------ Here ------------------------ //
               {
                match selected_user() {
                    Some(user)=>{
                        rsx!(
                            UserCard { user : user.clone(), onclick : move |_| {}}
                    )
                    }
                    None =>{
                        rsx!(
                            div {"Ne content"}
                        )
                    }
                }
               }
                // -------------------- End ------------------------//
            }

            // --------------------- End --------------------------//
        }
    }
}

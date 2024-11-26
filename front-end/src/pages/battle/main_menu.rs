use crate::utils::request::request_without_recaptcha;
use crate::utils::ErrResModel;
use crate::{config::SERVER_URL, router::Route};

use crate::pages::layout::layout::{NotificationData, NotificationType};
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use web_sys::console;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BattleStatus {
    BattleClosed,
    Voting,
    WaitingResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WinnerStatus {
    NotDetermine,
    WinnerA,
    WinnerB,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectedMatch {
    pub match_id: String,
    pub a_camp_username: String,
    pub b_camp_username: String,
    pub winner: WinnerStatus,
    pub status: BattleStatus,
    pub a_camp_img_src: String,
    pub b_camp_img_src: String,
    pub a_camp_vid_src: String,
    pub b_camp_vid_src: String,
    pub judges: Vec<String>,
    pub rules: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetShowListResModel {
    data: Vec<SelectedMatch>,
    max_page: i32,
}

#[component]
pub fn MainMenu() -> Element {
    let navigation = use_navigator();
    let mut pagination = use_signal(|| 1);
    let mut max_page = use_signal(|| 1);
    let user_show_count = use_signal(|| 5);
    let mut search_text = use_signal(|| String::from(""));
    let mut show_take_back = use_signal(|| false);
    let mut show_incomplete = use_signal(|| false);
    let mut show_close = use_signal(|| true);
    let mut battle_list = use_signal(|| Vec::<SelectedMatch>::new());
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();
    let mut test_voting = 0;

    use_effect(move || {
        let _ = pagination.read();
        to_owned![
            show_take_back,
            show_incomplete,
            show_close,
            pagination,
            search_text
        ];
        spawn(async move {
            match request_without_recaptcha(
                "post",
                format!("{}/api/v0/battle/battle-main/show-battle-list", SERVER_URL).as_str(),
                json!({
                    "search" : search_text(),
                    "count" : user_show_count(),
                     "pagination" : pagination(),
                     "show_take_backs" : show_take_back(),
                     "show_close" : show_close(),
                     "show_incomplete" : show_incomplete()
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
                        match res.json::<GetShowListResModel>().await {
                            Ok(results) => {
                                let a = serde_json::to_string(&results).unwrap();
                                console::log_1(&a.into());
                                battle_list.set(results.data);
                                max_page.set(results.max_page);
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
                }
            };
        });
    });

    rsx! {

        div {
             class: "page-body-min-h pt-10 pb-12 w-4/5 mx-auto relative ",
             // ------------- Start Search Part --------------- //
             div {
                class: "mb-10",
                div {
                    class: "flex items-center justify-center",
                    form {
                        class: "w-full flex flex-col gap-2 items-center",
                        input {
                            "type": "text",
                            oninput : move |e| {
                                search_text.set(e.value());
                                pagination.set(1);
                            },
                            name: "search",
                            id: "search",
                            placeholder: "",
                            class: "px-2 h-10 bg-gray-800 rounded-md w-1/2",
                        }
                        div {
                            class: "flex flex-wrap items-center gap-2 px-2 {show_take_back} mt-2",
                            button {
                                class: "rounded-md w-16 h-10 flex items-center justify-center gap-2 bg-gray-500 hover:bg-green-600 transition-all duration-300",
                                "type" : "button",
                                "test-id" : "main-page-stamp-button",
                                onclick : move |_| {
                                    show_take_back.set(!show_take_back());
                                    pagination.set(1);
                                },
                                if show_take_back() {
                                    div {i { class: "fa-solid fa-check", } }

                                } else {
                                    div {i { class: "fa-solid fa-ban" } }
                                }
                                i { class: "fa-solid fa-stamp" },
                            }
                            button {
                                class: "rounded-md w-16 h-10 flex items-center justify-center gap-2 bg-gray-500 hover:bg-red-600 transition-all duration-300",
                                "type" : "button",
                                "test-id" : "main-page-delete-button",
                                onclick : move |_| {
                                    show_close.set(!show_close());
                                    pagination.set(1);
                                },
                                if show_close() {
                                    div {i { class: "fa-solid fa-check", } }

                                } else {
                                    div {i { class: "fa-solid fa-ban" } }
                                }
                                i { class: "fa-solid fa-trash" }
                            }
                            button {
                                class: "rounded-md w-16 h-10 flex items-center justify-center gap-2 bg-gray-500 hover:bg-gray-600 transition-all duration-300",
                                "type" : "button",
                                "test-id" : "main-page-wait-button",
                                onclick : move |_| {
                                    show_incomplete.set(!show_incomplete());
                                    pagination.set(1);
                                },
                                if show_incomplete() {
                                    div {i { class: "fa-solid fa-check", } }

                                } else {
                                    div {i { class: "fa-solid fa-ban" } }
                                }
                                i { class: "fa-solid fa-question" }
                            }
                        }
                    }
                }
            }
              // ------------- End Search Part --------------- //
              // -------------- Battle card ------------------//
              div {
                class : "w-full md:w-4/5 mx-auto px-2 md:px-0",
                {
                    battle_list().into_iter().map(move |battle| {
                        rsx!(
                            div {
                                class: "flex flex-col md:flex-col items-center justify-between gap-5 mb-5",
                                onclick : move |_| {
                                    navigation.push(Route::Match {match_id : battle.match_id.to_string()});
                                return
                                },
                                // left
                                div {
                                    class: "p-2 border border-4 border-gray-700 bg-transparent w-full md:w-1/2 flex flex-col items-center gap-2",
                                    img {
                                        src: "{battle.a_camp_img_src}",
                                        alt: "",
                                        class: "w-full object-cover",
                                        style: "height: 500px",
                                    },
                                },
                                // bottom
                                if battle.status == BattleStatus::BattleClosed {
                                    div {
                                        class: "w-full md:w-1/2 bg-gray-900 py-2 px-5 flex flex-col items-center justify-center gap-2",
                                        "test-id" : "unofficial-vote",
                                        h2 {
                                            class: "text-lime-400 text-lg text-center",
                                            div {
                                                class: "flex items-center gap-5 text-white flex-col",
                                                if battle.winner == WinnerStatus::WinnerB {
                                                    i {
                                                        class: "fas fa-times fa-fw text-[#992727]",
                                                    }
                                                } else if battle.winner == WinnerStatus::WinnerA {
                                                    i {
                                                        class: "fas fa-trophy fa-fw text-yellow-500",
                                                    }
                                                } else {
                                                    i {
                                                        class: "fa-solid fa-clock  fa-fw text-yellow-500 invisible",
                                                    }
                                                }
                                                h3 { "{battle.a_camp_username}" },
                                            }
                                            i {
                                                class: "fas fa-stamp text-2xl mr-1 my-1 text-gray-500",
                                            },
                                            div {
                                                class: "flex flex-col items-center gap-5 text-white",
                                                h3 { "{battle.b_camp_username}" },
                                                if battle.winner == WinnerStatus::WinnerA {
                                                    i {
                                                        class: "fas fa-times fa-fw text-[#992727]",
                                                    }
                                                } else if battle.winner == WinnerStatus::WinnerB {
                                                    i {
                                                        class: "fas fa-trophy fa-fw text-yellow-500",
                                                    }
                                                } else {
                                                    i {
                                                        class: "fa-solid fa-clock  fa-fw text-yellow-500 invisible",
                                                    }
                                                }
                                            }
                                        },
                                    }
                                } else if battle.status == BattleStatus::WaitingResponse {
                                    div {
                                        class: "w-full md:w-1/2 bg-gray-900 py-2 px-5 flex flex-col items-center justify-center gap-2",
                                        h2 {
                                            class: "text-lime-400 text-lg text-center",
                                            div {
                                                class: "flex flex-col items-center gap-5 text-white",
                                                if battle.winner == WinnerStatus::WinnerB {
                                                    i {
                                                        class: "fas fa-times fa-fw text-[#992727]",
                                                    }
                                                } else if battle.winner == WinnerStatus::WinnerA {
                                                    i {
                                                        class: "fas fa-trophy fa-fw text-yellow-500",
                                                    }
                                                } else {
                                                    i {
                                                        class: "fa-solid fa-clock  fa-fw text-yellow-500 invisible",
                                                    }
                                                }
                                                h3 { class : "text-white", "{battle.a_camp_username}" },
                                            }
                                            i {
                                                class: "fa-solid fa-clock text-2xl mr-1 text-gray-500",
                                            },
                                            "Waiting {battle.b_camp_username}'s response",
                                        },
                                    }
                                } else {
                                    {
                                        test_voting +=  1;
                                    }
                                    div {
                                        class: "w-full md:w-1/2 bg-gray-900 py-2 px-5 flex flex-col items-center justify-center gap-2",
                                        "test-id" : "voting-battle-{test_voting}",
                                        h2 {
                                            class: "text-lime-400 text-lg text-center",
                                            div {
                                                class: "flex flex-col items-center gap-5 text-white",
                                                if battle.winner == WinnerStatus::WinnerB {
                                                    i {
                                                        class: "fas fa-times fa-fw text-[#992727]",
                                                    }
                                                } else if battle.winner == WinnerStatus::WinnerA {
                                                    i {
                                                        class: "fas fa-trophy fa-fw text-yellow-500",
                                                    }
                                                } else {
                                                    i {
                                                        class: "fa-solid fa-clock  fa-fw text-yellow-500 invisible",
                                                    }
                                                }
                                                h3 { "{battle.a_camp_username}" },
                                            }
                                            i {
                                                class: "fa-solid fa-clock text-2xl mr-1 text-gray-500",
                                            },
                                            div {
                                                class: "flex flex-col items-center gap-5 text-white",
                                                h3 { "{battle.b_camp_username}" },
                                                if battle.winner == WinnerStatus::WinnerA {
                                                    i {
                                                        class: "fas fa-times fa-fw text-[#992727]",
                                                    }
                                                } else if battle.winner == WinnerStatus::WinnerB {
                                                    i {
                                                        class: "fas fa-trophy fa-fw text-yellow-500",
                                                    }
                                                } else {
                                                    i {
                                                        class: "fa-solid fa-clock  fa-fw text-yellow-500 invisible",
                                                    }
                                                }
                                            }
                                        },
                                    }
                                }

                                if battle.status != BattleStatus::WaitingResponse{
                                    // right
                                    div {
                                        class: "p-2 border border-4 border-gray-700 bg-transparent w-full md:w-1/2 flex flex-col items-center gap-2",
                                        img {
                                            src: "{battle.b_camp_img_src}",
                                            alt: "",
                                            class: "w-full object-cover",
                                            style: "height: 500px",
                                        },
                                    }
                                } else {
                                }
                            }
                        )
                    })
                }
              }
             // -------------- End Pard ------------------------//
             // ------------- Start Pagination List Part ---------------- //
            div {
                class: "pagination absolute  bottom-1 w-full mt-4",
                div {
                    class: "mb-1",
                    div {
                        class: "flex items-center justify-center gap-3",
                        button {
                            class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                            onclick : move |_|  {
                                if pagination() -1 >= 1 {
                                    pagination.set(pagination() - 1)
                                }
                            },
                            i {
                                class: "fa-solid fa-arrow-left",
                            }
                        }
                        // max page is more than 5
                        if max_page() > 5 {
                        if pagination() <= max_page() -3{
                            if pagination() == 1{
                                for i in 1..3{
                                    button {
                                        class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                                        onclick : move |_|  pagination.set(i),
                                        "{i}"
                                    }
                                }
                            } else {
                                for i in pagination()-1..pagination()+1{
                                    button {
                                        class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                                        onclick : move |_|  pagination.set(i),
                                        "{i}"
                                    }
                                }
                            }
                        } else {
                            for i in max_page()-3..max_page(){
                                button {
                                    class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                                    onclick : move |_|  pagination.set(i),
                                    "{i}"
                                }
                            }
                        }
                        } else {
                        // max page less than 5
                            for i in 0..max_page(){
                                button {
                                    class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                                    onclick : move |_|  pagination.set(i+1),
                                    "{i+1}"
                                }
                            }
                        }

                        button {
                            class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                            onclick : move |_|  {
                                if pagination() +1 <= max_page() {
                                    pagination.set(pagination() + 1)
                                }
                            },
                            i {
                                class: "fa-solid fa-arrow-right",
                            }
                        }

                        // End pagination
                    }

                }
            }
             // ------------- End Pagination List Part ---------------- //
    }
    }
}

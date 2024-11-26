use crate::components::element::user_card::UserCard;
use crate::components::page_element::battle::vote::vote::Vote;
use crate::components::page_element::battle::vote::vote_list::VoteList;
use crate::config::HOST_URL;
use crate::pages::battle::callout::UserSelect;
use crate::pages::battle::matchs::BattleStatus;
use crate::pages::battle::matchs::SelectedMatch;
use crate::pages::battle::matchs::VotingType;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::utils::js_binding::copy_clipboard;
use crate::utils::time::timestamp_to_date;
use dioxus::prelude::*;

#[component]
pub fn ShowBattle(
    warning: Signal<String>,
    page_flag: Signal<i32>,
    battle_information: Signal<Option<SelectedMatch>>,
    a_user_information: Signal<Option<UserSelect>>,
    b_user_information: Signal<Option<UserSelect>>,
    is_fired: bool,
    voting_type: Signal<VotingType>,
) -> Element {
    // notification context
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();
    match battle_information() {
        Some(battle_info) => {
            rsx! {
                // ----------- Search part ---------------//
                div {
                    class: "mb-10 px-3",
                    div {
                        class: "w-full md:w-4/5 mx-auto",
                        div {
                            class : "text-right mb-2 flex",
                            div {
                                class : "w-[50%] text-left",
                                div {
                                    class : "text-left",
                                    "{battle_info.a_camp_username}"
                                }
                                div {
                                    class : "text-left",
                                    "{battle_info.b_camp_username}"
                                }
                                if battle_info.status != BattleStatus::WaitingResponse {
                                    i {
                                        class: "fa-solid fa-clock  fa-fw text-white",
                                    }
                                } else {
                                    i { class: "fa-solid fa-stamp text-white" }
                                }
                            }
                        
                            div {
                                class : "w-[50%] p-4",
                                button {
                                    class : "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 right",
                                    onclick : move |_| {
                                        let url = format!("{}/match/{}", HOST_URL, battle_info.match_id);
                                        let result : bool = copy_clipboard(url.to_string()).into_serde().unwrap();
                                            notification_data.set({
                                                let mut data = notification_data().clone(); // Clone existing data
                                                data.push(NotificationData {
                                                    title: "".to_string(),
                                                    content: "Link Copied".to_string(),
                                                    notification_type: NotificationType::Success,
                                                });
                                                data // Return the updated data
                                            });
                                    },
                                    svg {
                                        "aria_hidden": "true",
                                        "focusable": "false",
                                        "data_prefix": "fas",
                                        "data_icon": "share-alt",
                                        role: "img",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        "viewBox": "0 0 448 512",
                                        class: "svg-inline--fa fa-share-alt fa-w-14",
                                        path {
                                            fill: "currentColor",
                                            d: "M352 320c-22.608 0-43.387 7.819-59.79 20.895l-102.486-64.054a96.551 96.551 0 0 0 0-41.683l102.486-64.054C308.613 184.181 329.392 192 352 192c53.019 0 96-42.981 96-96S405.019 0 352 0s-96 42.981-96 96c0 7.158.79 14.13 2.276 20.841L155.79 180.895C139.387 167.819 118.608 160 96 160c-53.019 0-96 42.981-96 96s42.981 96 96 96c22.608 0 43.387-7.819 59.79-20.895l102.486 64.054A96.301 96.301 0 0 0 256 416c0 53.019 42.981 96 96 96s96-42.981 96-96-42.981-96-96-96z",
                                        }
                                    }
                                }

                            }
                         }
                        div {
                            class: "border border-4 border-gray-800 p-2 flex flex-col gap-5",
                            div {
                                class: "w-full mb-4 flex flex-col item-center justify-center gap-3",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "30",
                                    height: "30",
                                    fill: "#fff",
                                    "viewBox": "0 0 256 256",
                                    path {
                                        d: "M216,32H152a8,8,0,0,0-6.34,3.12l-64,83.21L72,108.69a16,16,0,0,0-22.64,0l-8.69,8.7a16,16,0,0,0,0,22.63l22,22-32,32a16,16,0,0,0,0,22.63l8.69,8.68a16,16,0,0,0,22.62,0l32-32,22,22a16,16,0,0,0,22.64,0l8.69-8.7a16,16,0,0,0,0-22.63l-9.64-9.64,83.21-64A8,8,0,0,0,224,104V40A8,8,0,0,0,216,32Zm-8,68.06-81.74,62.88L115.32,152l50.34-50.34a8,8,0,0,0-11.32-11.31L104,140.68,93.07,129.74,155.94,48H208Z"
                                    }
                                }
                                video {
                                    class: "w-full aspect-video",
                                    controls: true,
                                    source {
                                        src: "{battle_info.a_camp_vid_src}",
                                        "type": "video/mp4",
                                    }
                                    "Your browser does not support the video tag."
                                }
                                p {
                                    class: "text-sm my-2",
                                    "{timestamp_to_date(battle_info.a_camp_timestamp)}"
                                }
                                div {
                                    class: "user-card border border-4 border-gray-800 p-5 transition-all duration-500",
                                    UserCard {user : a_user_information().unwrap(), onclick : move |_| {}, }
                                }
                                h3 {
                                    class: "text-center text-xl mb-10",
                                    "{battle_info.rules}"
                                }
                                br {
                                    class : "bg-gray-500 text-white"
                                }
                            }
                                if battle_info.status != BattleStatus::WaitingResponse {
                                    div {
                                        class: "w-full mb-4 flex flex-col item-center justify-center gap-3",
                                        i {
                                            class: "fa fa-shield  fa-fw text-white text-[1.3em]",
                                        }
                                        video {
                                            class: "w-full aspect-video",
                                            controls: true,
                                            source {
                                                src: "{battle_info.b_camp_vid_src}",
                                                "type": "video/mp4",
                                            }
                                            "Your browser does not support the video tag."
                                        }
                                        p {
                                            class: "text-sm text-center lg:text-start my-2",
                                            "{timestamp_to_date(battle_info.b_camp_timestamp)}"
                                        }
                                        div {
                                            class: "user-card border border-4 border-gray-800 p-5 transition-all duration-500",
                                            UserCard {user : b_user_information().unwrap(), onclick : move |_|{}, }
                                        }
                                        h3 {
                                            class: "text-center text-xl mb-5",
                                            "{battle_info.b_reply}"
                                        }
                                    }

                                }
                            }
                            if battle_info.status == BattleStatus::WaitingResponse {
                                    if is_fired {
                                        div {
                                            class: "w-full bg-gray-900 py-2 px-5 flex flex-col items-center justify-center gap-2 mt-5",
                                            "test-id" : "battle-fire",
                                            onclick : move |_| {
                                                page_flag.set(1);
                                            },
                                                h2 {
                                                    class: "text-lime-400 text-lg text-center",
                                                    i {
                                                        class: "fa-solid fa-clock text-2xl mr-2",
                                                    },
                                                    br {},
                                                    "Fire to {battle_info.b_camp_username}",
                                                }

                                        }
                                    } else {
                                        div {
                                            class: "w-full bg-gray-900 py-2 px-5 flex flex-col items-center justify-center gap-2 mt-5",
                                            h2 {
                                                class: "text-lime-400 text-lg text-center",
                                                i {
                                                    class: "fa-solid fa-clock text-2xl mr-2",
                                                },
                                                br {},
                                                "Waiting {battle_info.b_camp_username}'s response",
                                            },
                                        }
                                    }
                            }
                            // --------- Voting component ---------//
                            if voting_type() != VotingType::Not {
                                Vote {
                                    voting_type : voting_type.clone(),
                                    battle_information : battle_info.clone()
                                }
                            }
                            // --------- Voting liset show component ---------//
                            if battle_info.status == BattleStatus::BattleClosed {
                                VoteList {battle_information : battle_info.clone()}
                            }}
                }
            }
        }
        None => {
            rsx! {
                div {
                    class: "mb-10",
                }
            }
        }
    }
}

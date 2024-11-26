use crate::components::element::user_card::UserCard;
use crate::components::page_element::battle::vote::vote::Vote;
use crate::components::page_element::battle::vote::vote_list::VoteList;
use crate::config::SERVER_URL;
use crate::pages::battle::callout::UserSelect;
use crate::pages::battle::matchs::{
    BattleStatus, SelectedMatch, ShowSelectedBattleResModel, VotingType,
};
use crate::pages::layout::layout::SharedData;
use crate::utils::request::send_request;
use crate::utils::storage::get_local_storage;
use crate::utils::time::timestamp_to_date;
use crate::utils::ErrResModel;
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde_json::json;

#[component]
pub fn LiveBattleShow(code: String) -> Element {
    // Page flag, it will take 3 values, 0,1,2
    // 0 means user_list page, 1 means aim user,  2 means fire
    let mut shared_data = use_context::<Signal<SharedData>>();
    let mut is_fired = use_signal(|| false);
    let mut warning = use_signal(|| String::from(""));
    let mut a_camp_user_information = use_signal(|| None as Option<UserSelect>);
    let mut b_camp_user_information = use_signal(|| None as Option<UserSelect>);
    let mut battle_information = use_signal(|| None as Option<SelectedMatch>);
    let mut voting_type = use_signal(|| VotingType::Not);

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

    // Use Effect will be occur when search and pagination changed
    use_effect(use_reactive((&code,), move |code| {
        let (live_battle_code,) = code;
        spawn(async move {
            show_loading();
            match send_request(
                "post",
                format!("{}/api/v0/battle/live-battle/show-live-battle", SERVER_URL).as_str(),
                json!({
                    "code" : live_battle_code.to_string(),
                    "token" : match get_local_storage("token"){
                        Some(token)=> token,
                        None => "".to_string()
                    }
                }),
                false,
                "liveBattleGet",
            )
            .await
            {
                Ok(res) => {
                    if res.status() != StatusCode::OK {
                        match res.json::<ErrResModel>().await {
                            Ok(results) => {
                                warning.set(results.cause);
                                exit_loading();
                            }
                            Err(_) => {
                                warning.set("Response is not correct".to_string());
                                exit_loading();
                            }
                        }
                    } else {
                        match res.json::<ShowSelectedBattleResModel>().await {
                            Ok(results) => {
                                a_camp_user_information.set(Some(results.a_camp_information));
                                b_camp_user_information.set(Some(results.b_camp_information));
                                battle_information.set(Some(results.battle_information));
                                is_fired.set(results.username_check);
                                voting_type.set(results.voting_type);
                                exit_loading();
                                return;
                            }
                            Err(e) => {
                                warning.set(format!("Response error : {}", e.to_string()));
                                exit_loading();
                            }
                        }
                    }
                }
                Err(_) => {
                    warning.set("Please check your internet connection".to_string());
                    exit_loading();
                }
            };
        });
    }));
    match battle_information() {
        Some(battle_info) => {
            rsx!(
                div {
                    class: "page-body-min-h pt-10 pb-12 mx-auto relative px-10",
                        //  User info
                        div {
                            class: "border border-4 border-gray-800 p-2 flex md:flex-row flex-col gap-5",
                            div {
                                class: "w-full mb-4 flex flex-col item-center justify-center gap-3",

                                p {
                                    class: "my-2 text-[1.2em] gap-5",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class : "inline mx-3 text-[1.5em] w-[36px]",
                                        fill: "#fff",
                                        "viewBox": "0 0 256 256",
                                        path {
                                            d: "M216,32H152a8,8,0,0,0-6.34,3.12l-64,83.21L72,108.69a16,16,0,0,0-22.64,0l-8.69,8.7a16,16,0,0,0,0,22.63l22,22-32,32a16,16,0,0,0,0,22.63l8.69,8.68a16,16,0,0,0,22.62,0l32-32,22,22a16,16,0,0,0,22.64,0l8.69-8.7a16,16,0,0,0,0-22.63l-9.64-9.64,83.21-64A8,8,0,0,0,224,104V40A8,8,0,0,0,216,32Zm-8,68.06-81.74,62.88L115.32,152l50.34-50.34a8,8,0,0,0-11.32-11.31L104,140.68,93.07,129.74,155.94,48H208Z"
                                        }
                                    }
                                    "{timestamp_to_date(battle_info.a_camp_timestamp)}"
                                }
                                div {
                                    class: "user-card border border-4 border-gray-800 p-5 transition-all duration-500",
                                    UserCard {user : a_camp_user_information().unwrap(), onclick : move |_| {}, }
                                }
                            }
                            div {
                                class: "w-full mb-4 flex flex-col item-center justify-center gap-3",

                                p {
                                    class: " my-2 text-[1.2em] gap-5",
                                    i {
                                        class: "fas fa-shield mx-3 text-[1.5em] w-[50px]",
                                    }
                                    "{timestamp_to_date(battle_info.a_camp_timestamp)}"
                                }
                                div {
                                    class: "user-card border border-4 border-gray-800 p-5 transition-all duration-500",
                                    UserCard {user : b_camp_user_information().unwrap(), onclick : move |_| {}, }
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
                        }
                }
            )
        }
        None => {
            rsx! {
                div {
                   class: "page-body-min-h pt-10 pb-12 mx-auto relative ",
                   ""
                }
            }
        }
    }
}

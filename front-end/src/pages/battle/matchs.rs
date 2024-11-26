use crate::components::page_element::battle::callout::aim::Aim;
use crate::components::page_element::battle::reply::show_battle::ShowBattle;
use crate::components::page_element::battle::reply::trigger_reply::TrigerReply;
use crate::config::SERVER_URL;
use crate::pages::battle::callout::UserSelect;
use crate::pages::layout::layout::SharedData;
use crate::utils::request::send_request;
use crate::utils::storage::get_local_storage;
use crate::utils::ErrResModel;
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

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


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum VotingType {
    Not,
    Official,
    Unofficial,
    Judge,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub a_camp_timestamp : i64,
    pub b_camp_timestamp : i64,
    pub b_reply : String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShowSelectedBattleResModel {
    pub a_camp_information: UserSelect,
    pub b_camp_information: UserSelect,
    pub username_check: bool,
    pub battle_information: SelectedMatch,
    pub voting_type: VotingType,
}

#[component]
pub fn Match(match_id: String) -> Element {
    // Page flag, it will take 3 values, 0,1,2
    // 0 means user_list page, 1 means aim user,  2 means fire
    let page_flag = use_signal(|| 0);
    let mut shared_data = use_context::<Signal<SharedData>>();
    let mut is_fired = use_signal(|| false);
    let mut warning = use_signal(|| String::from(""));
    let video_content = use_signal(|| String::from(""));
    let video_content_data = use_signal(|| None as Option<web_sys::File>);
    let video_type = use_signal(|| String::from(""));
    let image_content = use_signal(|| String::from(""));
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
    use_effect(use_reactive((&match_id,), move |match_id| {
        let (battle_match_id,) = match_id;
        spawn(async move {
            show_loading();
            match send_request(
                "post",
                format!("{}/api/v0/battle/battle-main/show-select-battle", SERVER_URL).as_str(),
                json!({
                    "match_id" : battle_match_id.to_string(),
                    "token" : match get_local_storage("token"){
                        Some(token)=> token,
                        None => "".to_string()
                    }
                }),
                false,
                "getBattle",
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

    rsx!(
        div {
            class: "page-body-min-h pt-10 pb-12 mx-auto relative ",
            if page_flag() == 0 {
                ShowBattle {
                    warning : warning.clone(),
                    page_flag : page_flag.clone(),
                    battle_information : battle_information.clone(),
                    a_user_information : a_camp_user_information.clone(),
                    b_user_information : b_camp_user_information.clone(),
                    is_fired : is_fired(),
                    voting_type : voting_type.clone(),
                }
            } else if page_flag() == 1{
                Aim {
                    video_content : video_content.clone(),
                    warning : warning.clone(),
                    page_flag : page_flag.clone(),
                    selected_user : a_camp_user_information.clone(),
                    video_type : video_type.clone(),
                    video_content_data : video_content_data.clone()
                }
            }  else {
                TrigerReply{
                    video_content : video_content.clone(),
                    warning : warning.clone(),
                    page_flag : page_flag.clone(),
                    selected_user : a_camp_user_information.clone(),
                    video_type : video_type.clone(),
                    image_content : image_content.clone(),
                    video_content_data : video_content_data.clone(),
                    battle_information : battle_information.clone(),
                    matchs_id : match_id.to_string()
                }
            }
        }
    )
}

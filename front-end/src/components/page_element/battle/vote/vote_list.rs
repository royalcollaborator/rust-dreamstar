use crate::pages::layout::layout::{NotificationData, NotificationType};
use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::SERVER_URL;
use crate::pages::battle::matchs::BattleStatus;
use crate::pages::battle::matchs::SelectedMatch;
use crate::utils::request::request_without_recaptcha;
use crate::utils::time::timestamp_to_date;
use crate::utils::ErrResModel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub match_id: String,
    pub voter_name: String,
    pub voter_youtube_channel_name: String,
    pub voter_youtube_channel_id: String,
    pub voter_instagram_name: String,
    pub voter_twitter_name: String,
    pub voter_twitter: String,
    pub timestamp: i64,
    pub a_camp_votes: i32,
    pub b_camp_votes: i32,
    pub statement: String,
    pub vote_type: i32,
    pub thumbnail: String,
    pub bitcoin_transaction_id: String,
    pub satoshi_amount: i64,
    pub dollar_amount: f64,
    pub signature_img_file_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BattleScore {
    pub winner_name: String,
    pub loser_name: String,
    pub winner_final_vote: i32,
    pub loser_final_vote: i32,
    pub winner_official_vote: i32,
    pub loser_official_vote: i32,
    pub winner_judge_vote: i32,
    pub loser_judge_vote: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVoteListResModel {
    pub data: Vec<Vote>,
    pub max_page: i32,
    pub battle_info: BattleScore,
}

#[component]
pub fn VoteList(battle_information: SelectedMatch) -> Element {
    let mut vote_list = use_signal(|| Vec::<Vote>::new());
    let mut battle_info = use_signal(|| None as Option<BattleScore>);
    let search_text = use_signal(|| String::from(""));
    let pagination = use_signal(|| 1);
    let max_page = use_signal(|| 1);
    let user_show_count = use_signal(|| 5);
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();
    // Use Effect will be occur when search and pagination changed
    use_effect(use_reactive(
        (&battle_information,),
        move |(battle_information,)| {
            if battle_information.status != BattleStatus::BattleClosed {
                return;
            }
            let _ = pagination.read();
            to_owned![pagination, user_show_count, search_text, max_page];
            spawn(async move {
                match request_without_recaptcha(
                    "post",
                    format!("{}/api/v0/vote/voting-list", SERVER_URL).as_str(),
                    json!({
                        "search" : search_text(),
                        "count" : user_show_count(),
                         "pagination" : pagination(),
                         "match_id" : battle_information.match_id.to_string()
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
                            match res.json::<GetVoteListResModel>().await {
                                Ok(results) => {
                                    vote_list.set(results.data);
                                    max_page.set(results.max_page);
                                    battle_info.set(Some(results.battle_info));
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
        },
    ));

    if battle_information.status == BattleStatus::BattleClosed {
        match battle_info() {
            Some(battle_infos) => {
                rsx! {
                div {
                    class: "my-10 w-full flex items-center justify-between flex-col md:flex-row gap-5 p-4 text-[#facc15]",
                    div {
                        class: "flex flex-col md:flex-row item-center gap-5 ",
                        i { class: "fas fa-trophy text-7xl" }
                        div {
                            h3 { class: "font-bold text-center md:text-start", "Winner" }
                            h3 { class: "text-2xl md:text-3xl text-center font-bold md:text-start", "{battle_infos.winner_name.to_string()}"}
                            h3 { class: "text-2xl md:text-3xl text-center font-bold text-white md:text-start", "{battle_infos.winner_final_vote} | {battle_infos.loser_final_vote}"}
                        }
                    }
                }
                div {
                    class: "w-full flex flex-col items-center gap-5",
                    h1 { class: "text-center md:text-start text-4xl font-bold mb-2 w-full", "All Votes" }
                    div { class : "flex flex-col gap-5 w-full",
                    {
                        vote_list()
                        .into_iter()
                        .map(|vote| {
                            rsx!{
                                div {
                                    class: "bg-slate-900 rounded-xl p-3 flex flex-col gap-2",
                                    div {
                                        class: "flex flex-col md:flex-row gap-2",
                                        div {
                                            class : if vote.vote_type == 0{
                                                "w-full bg-slate-800 hover:bg-slate-700 transition-all duration-500 cursor-pointer rounded-lg p-3 flex flex-col gap-2 text-center"
                                            } else {
                                                "w-full md:w-1/2 bg-slate-800 hover:bg-slate-700 transition-all duration-500 cursor-pointer rounded-lg p-3 flex flex-col gap-2 text-center"
                                            },
                                            h1 {
                                                class: "text-3xl font-bold",
                                                "{vote.voter_name}"
                                            }
                                            h1 {
                                                class: "text-sm",
                                                "{timestamp_to_date(vote.timestamp)}"
                                            }
                                            h1 {
                                                class: "text-2xl font-bold",
                                                if vote.vote_type == 1 {
                                                    i {
                                                        class: "fas fa-certificate mr-2",
                                                    }
                                                } else if vote.vote_type == 2{
                                                    i {
                                                        class: "fas fa-gavel  mr-2",
                                                    }
                                                } else {
                                                    i {
                                                        class: "mr-2",
                                                    }
                                                }
                                                "{vote.a_camp_votes} | {vote.b_camp_votes}"
                                            }
                                        }
                                        if vote.vote_type != 0 {
                                            div {
                                                class: "w-full md:w-1/2 bg-slate-800 rounded-lg p-3 flex flex-col justify-center md:justify-start gap-2",
                                                img {
                                                    src: "{vote.signature_img_file_id}",
                                                    alt: "",
                                                    class: "w-full object-contain",
                                                    style: "height: 120px",
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        class: "w-full bg-slate-800 rounded-lg p-3",
                                        p {
                                            "{vote.statement}"
                                        }
                                    }
                                }
                            }
                        })
                    },
                }
                }
                // ------------- Start Pagination List Part ---------------- //
                // Pagination {pagination : pagination.clone(), max_page : max_page.clone()}
                 // ------------- End Pagination List Part ---------------- //
                }
            }
            None => {
                rsx! {
                    div {""}
                }
            }
        }
    } else {
        rsx!(
            div {""}
        )
    }
}

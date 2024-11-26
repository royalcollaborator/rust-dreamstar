use crate::components::element::user_card::UserCard;
use crate::pages::battle::callout::UserSelect;
use crate::utils::request::request_without_recaptcha;
use crate::utils::util::go_unauthorized;
use crate::utils::ErrResModel;
use crate::{config::SERVER_URL, router::Route};

use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponseResModel {
    pub data: Vec<UserSelect>,
}

#[component]
pub fn Response() -> Element {
    let navigation = use_navigator();
    let user_list = use_signal(|| Vec::<UserSelect>::new());
    let mut warning = use_signal(|| String::from(""));

    // Use Effect will be occur when search and pagination changed.
    use_effect(move || {
        to_owned![user_list];
        spawn(async move {
            match request_without_recaptcha(
                "post",
                format!("{}/api/v0/battle/response/get-response-list", SERVER_URL).as_str(),
                json!({}),
                true,
            )
            .await
            {
                Ok(res) => {
                    if res.status() != StatusCode::OK {
                        if res.status() == StatusCode::UNAUTHORIZED {
                            go_unauthorized(navigation.clone());
                        }
                        match res.json::<ErrResModel>().await {
                            Ok(results) => {
                                warning.set(results.cause);
                                return;
                            }
                            Err(_) => {
                                warning.set("Response is not correct".to_string());
                                return;
                            }
                        }
                    }
                    match res.json::<GetResponseResModel>().await {
                        Ok(results) => {
                            user_list.set(results.data);
                        }
                        Err(e) => {
                            warning.set(format!("Response error : {}", e.to_string()));
                        }
                    }
                }
                Err(_) => {
                    warning.set("Please check your internet connection".to_string());
                }
            };
        });
    });

    // Go to match page
    let selected = move |user: UserSelect| {
        navigation.push(Route::Match {
            match_id: user._id.to_string(),
        });
        return;
    };

    rsx!(
        div { class: "page-body-min-h pt-10 pb-12 w-4/5 mx-auto relative ",
            // ------------- Start User List Part ---------------- //
            div {
                class: "user-grid px-5 ",
                style: "overflow-y: scroll",
                //  ------------------ Here ------------------------ //
                {
                    user_list()
                    .into_iter()
                    .map(|user| {
                        rsx!(
                            UserCard {user : user.clone(), onclick : move |user : UserSelect| selected(user.clone())})
                    })
                },
            }
        }
    )
}

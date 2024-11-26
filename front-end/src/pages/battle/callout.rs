use crate::components::page_element::battle::callout::{
    aim::Aim, judge::Judge, user_list::UserList,
};
use crate::pages::layout::layout::SharedData;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserSelect {
    pub _id: String,
    pub username: String,
    pub battler_score: f64,
    pub instagram_name: String,
    pub instagram_id: String,
    pub matches_won: i32,
    pub matches_lost: i32,
    pub matches_withdrawn: i32,
    pub callout: i32,
    pub response: i32,
    pub one_hundred_badge: i32,
    pub first_tourney_badge: String,
}

#[component]
pub fn CallOut() -> Element {
    let shared_data = use_context::<Signal<SharedData>>();
    let page_flag = use_signal(|| 0);
    let warning = use_signal(|| String::from(""));
    let video_content = use_signal(|| String::from(""));
    let video_content_data = use_signal(|| None as Option<web_sys::File>);
    let video_type = use_signal(|| String::from(""));
    let image_content = use_signal(|| String::from(""));
    let selected_user = use_signal(|| None as Option<UserSelect>);

    // Get login status from use_context
    let auth_flag = use_memo(move || shared_data().auth_flag);

    if page_flag() == 1 {
        rsx!(
            div { class: "page-body-min-h pt-10 pb-12 w-4/5 mx-auto relative ",
            Aim {
                video_content : video_content.clone(),
                warning : warning.clone(),
                page_flag : page_flag.clone(),
                selected_user : selected_user.clone(),
                video_type : video_type.clone(),
                video_content_data : video_content_data.clone()
            }
         }
        )
    } else if page_flag() == 2 {
        rsx!(div { class: "page-body-min-h pt-10 pb-12 w-4/5 mx-auto relative ",
        Judge {
            video_content : video_content.clone(),
            warning : warning.clone(),
            page_flag : page_flag.clone(),
            selected_user : selected_user.clone(),
            video_type : video_type.clone(),
            image_content : image_content.clone(),
            video_content_data : video_content_data.clone()
         } })
    } else {
        rsx!(div { class: "page-body-min-h pt-10 pb-12 w-4/5 mx-auto relative ",
        UserList {
            warning : warning.clone(),
            page_flag : page_flag.clone(),
            selected_user : selected_user.clone(),
            auth_flag : auth_flag(),
        } })
    }
}

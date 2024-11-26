use crate::router::Route;
use crate::utils::storage::set_local_storage;
use dioxus::prelude::*;
pub fn go_to_link(url: &str) {
    web_sys::window()
        .unwrap()
        .location()
        .set_href(url)
        .expect("failed to redirect");
}

pub fn go_unauthorized(nav: Navigator) {
    set_local_storage("token", "");
    nav.push(Route::Login);
}
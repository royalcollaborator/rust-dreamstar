/**
 * @author      Andrii
 * @published   May 30, 2024
 * @description Library to config all variables used in the server
 * @email : fight0903@outlook.com, solomon21century@outlook.com, kunaievandrii@gmail.com
 */

#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;
use crate::utils::js_binding::register_service_worker;
use dioxus::prelude::*;
use router::Route;

mod components;
mod config;
mod pages;
mod router;
mod utils;

// Main function
fn main() {
    config::init();
    // Register the service worker when the app start
    wasm_bindgen_futures::spawn_local(async {
        register_service_worker();
    });
    // Launch App
    launch(|| {
        rsx! { Router::<Route> {}}
    });
}

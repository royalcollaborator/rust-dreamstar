use crate::utils::js_binding::createCanvasElement;
use dioxus::prelude::*;

#[component]
pub fn Canvas() -> Element {
    // Create canvas (it is real dome)
    use_effect(move || {
        web_sys::console::log_1(&format!("I am use").into());
        let _ = createCanvasElement();
    });

    rsx! {
        div {
            class: "w-full md:w-1/2 flex flex-col items-center gap-2",
            "test-id" : "canvas-wrap",
            id : "canvas-wrap",
            small {
                class: "flex gap-2 items-center",
                i {
                    class: "fas fa-signature",
                }
                " signature"
            }
        }
        script {
            src : "/static/canvas.js"
        }
    }
}

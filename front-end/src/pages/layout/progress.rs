use dioxus::prelude::*;

#[component]
pub fn Progress() -> Element {
    rsx! {
        div {
            id : "progress-wrap",
            class: "flex items-center justify-center w-full h-full fixed z-[10] bg-black opacity-[0.8]",
            style : "display : none",
            div {
                class: "flex flex-col items-center justify-center gap-4 h-full p-6",
                div {
                    class: "w-full bg-gray-200 rounded-full dark:bg-gray-700",
                    div {
                        id: "progress-bar-value",
                        class: "bg-blue-600 text-xs font-medium text-blue-100 text-center p-0.5 leading-none rounded-full",
                        style: "width: 0%",
                        "0%"
                    }
                }
            }
        }
    }
}
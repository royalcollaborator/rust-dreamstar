use dioxus::prelude::*;

#[component]
pub fn Badge() -> Element {
    rsx! {
        div {
            class: "page-body-min-h mx-auto py-16 px-5",
            div {
                class: "grid lg:grid-cols-3 md:grid-cols-2 sm:grid-cols-1 gap-5",
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-bell" }
                    }
                    h3 { class: "text-lg",
                        "Winner of the first prestige tournament (8-15-2019 to 10-15-2019) : DanceMachine! (Also awarded to the top eight, and honorable mentions)."
                    }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-stamp" }
                    }
                    h3 { class: "text-lg",
                        "A battle has been finalized. Once the score is final, only unofficial votes are allowed."
                    }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-trophy" }
                    }
                    h3 { class: "text-lg",
                        "A battler is winning a battle (when the voting period is active), or that the battler won."
                    }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-times" }
                    }
                    h3 { class: "text-lg",
                        "A battler is losing a battle (when the voting period is active), or that the battler lost."
                    }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-question" }
                    }
                    h3 { class: "text-lg",
                        "A battle is waiting for a response so that the voting period can start. Battlers should contact each other before starting a battle! If you want to see a response, try commenting or messaging through the battler's social channel."
                    }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-trash" }
                    }
                    h3 { class: "text-lg",
                        "A battler had a video removed for any reason, including copyright violation. This is bad news and it will negatively impact the records."
                    }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-user-cog" }
                    }
                    h3 { class: "text-lg", "The admin badge." }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-badge" }
                    }
                    h3 { class: "text-lg",
                        "A certified/official vote. Requires a voter pass (one-time purchase), and a linked social media account. The vote also has to be cast within the voting period."
                    }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-minus-circle" }
                    }
                    h3 { class: "text-lg", "An unofficial vote." }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-book" }
                    }
                    h3 { class: "text-lg", "The new chapter badge. Coming soon." }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-star" }
                    }
                    h3 { class: "text-lg",
                        "One of the first battlers to register on DanceBattleZ. Awarded to 122 battlers on July 27th 2019."
                    }
                }
                div {
                    class: "flex items-start flex-col gap-4 p-5 bg-gray-900 rounded-md transition-all duration-500 hover:bg-gray-700",
                    div {
                        class: "text-4xl",
                        i { class: "fas fa-sword" }
                    }
                    h3 { class: "text-lg",
                        "The battler badges. The sword means callout. The shield means response. A battler profile may have one or both depending on the number of callouts and responses."
                    }
                }
            }
        }
    }
}

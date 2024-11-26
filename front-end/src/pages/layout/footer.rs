use crate::router::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "pt-6 pb-3 bg-neutral-900 text-white px-2 min-h-[100px]",
            div { class: "w-full md:w-4/5 mx-auto",
                div { class: "flex items-center justify-center",
                    p { class: "text-sm text-center md:text-start",
                        "This site is protected by reCAPTCHA and the Google ",
                        a {
                            class : "text-red-400 text-[1.1em]",
                            target: "_blank",
                            href : "https://policies.google.com/privacy",
                            " Privacy Policy ",
                        }
                        " and ",
                        a {
                            class : "text-red-400 text-[1.1em]",
                            target: "_blank",
                            href : "https://policies.google.com/terms",
                            " Terms of Service"
                        }
                        " apply"

                    }
                }
            }
            div { class: "flex flex-col md:flex-row  items-center justify-between mt-5",
                p { class: "text-sm", "© 2024 DanceBattleZ Inc." }
                ul { class: "flex items-center gap-4",
                    li {
                        Link { class: "text-sm hover:text-red-400 transition-all duration-300 underline",
                        to : Route::Policy,
                            "Privacy Policy"
                        }
                    }
                    li {
                        Link { class: "text-sm hover:text-red-400 transition-all duration-300 underline",
                        to : Route::Policy,
                            "Terms & Conditions"
                        }
                    }
                    li {
                        a { class: "flex items-center gap-2 text-sm hover:text-red-400 transition-all duration-300 underline",
                        target: "_blank",
                        href : "mailto:neutron@dancebattlez.com",
                        "Contact Us"
                        }
                    }
                }
            }
        }
    }
}

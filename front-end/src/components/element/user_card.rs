use crate::pages::battle::callout::UserSelect;
use dioxus::prelude::*;

#[component]
pub fn UserCard(user: UserSelect, onclick: EventHandler<UserSelect>) -> Element {
    rsx!(
        div {
            class: "grid lg:grid-cols-1 md:grid-cols-1 sm:grid-cols-1 gap-5 pb-1 mb-3",
            onclick : move |_| onclick(user.clone()),
            div {
                class: "user-card bg-gray-900 p-5 rounded-md hover:bg-gray-700 transition-all duration-500",
                div {
                    class: "flex flex-col items-center gap-5 md:flex-row md:items-end md:justify-between items-end justify-between",
                    div {
                        div {
                            class: "mb-4 flex items-end flex-col items-center md:flex-row md:items-end gap-3",
                            h1 {
                                class: "text-3xl font-bold",
                                "{user.username}"
                            }
                            if !user.instagram_id.is_empty() && !user.instagram_name.is_empty() {
                                button {
                                    class: "bg-gray-700 text-sm px-2 py-0.5 rounded md",
                                    i {
                                        class: "fa-brands fa-instagram",
                                    }
                                    "Instagram"
                                }
                            }
                        }
                        ul {
                            class: "flex items-center gap-2",
                            li { "Roles:" }
                            if user.one_hundred_badge  == 1{
                                li {
                                    class: "text-yellow-400",
                                    i {
                                        class: "fas fa-star",
                                    }
                                    " 100"
                                }
                            }
                            li {
                                class: "text-yellow-400",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    fill: "#fff",
                                    "viewBox": "0 0 256 256",
                                    path {
                                        d: "M216,32H152a8,8,0,0,0-6.34,3.12l-64,83.21L72,108.69a16,16,0,0,0-22.64,0l-8.69,8.7a16,16,0,0,0,0,22.63l22,22-32,32a16,16,0,0,0,0,22.63l8.69,8.68a16,16,0,0,0,22.62,0l32-32,22,22a16,16,0,0,0,22.64,0l8.69-8.7a16,16,0,0,0,0-22.63l-9.64-9.64,83.21-64A8,8,0,0,0,224,104V40A8,8,0,0,0,216,32Zm-8,68.06-81.74,62.88L115.32,152l50.34-50.34a8,8,0,0,0-11.32-11.31L104,140.68,93.07,129.74,155.94,48H208Z"
                                    }
                                }
                            }
                            li {
                                class: "text-white",
                                i {
                                    class: "fas fa-shield"
                                }
                            }
                        }
                    }
                    ul {
                        class: "flex items-center gap-2",
                        li {
                            class: "text-yellow-400",
                            i {
                                class: "fas fa-trophy",
                            }
                            " {user.matches_won}"
                        }
                        li {
                            class: "text-red-400",
                            i {
                                class: "fas fa-times",
                            }
                            " {user.matches_lost}"
                        }
                        li {
                            class: "flex items-center gap-2",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "20",
                                height: "20",
                                fill: "#fff",
                                "viewBox": "0 0 256 256",
                                path {
                                    d: "M216,32H152a8,8,0,0,0-6.34,3.12l-64,83.21L72,108.69a16,16,0,0,0-22.64,0l-8.69,8.7a16,16,0,0,0,0,22.63l22,22-32,32a16,16,0,0,0,0,22.63l8.69,8.68a16,16,0,0,0,22.62,0l32-32,22,22a16,16,0,0,0,22.64,0l8.69-8.7a16,16,0,0,0,0-22.63l-9.64-9.64,83.21-64A8,8,0,0,0,224,104V40A8,8,0,0,0,216,32Zm-8,68.06-81.74,62.88L115.32,152l50.34-50.34a8,8,0,0,0-11.32-11.31L104,140.68,93.07,129.74,155.94,48H208Z"
                                }
                            }
                            " {user.callout}"
                        }
                        li {
                            class: "text-white",
                            i {
                                class: "fas fa-shield",
                            }
                            " {user.response}"
                        }
                    }
                }
            }
        }

    )
}

use super::layout::SharedData;
use crate::pages::layout::layout::{NotificationData, NotificationType};
use crate::router::router::Route;
use crate::utils::js_binding::{installPWA, isPwaInstalled};
use crate::utils::storage::set_local_storage;
use dioxus::prelude::*;

#[component]
pub fn Header(auth_flag: bool, admin: bool) -> Element {
    let mut mobile_responsive = use_signal(|| false);
    let navigation = use_navigator();
    let mut shared_data = use_context::<Signal<SharedData>>();
    let check_app: bool = isPwaInstalled().into_serde().unwrap();
    let mut notification_data = use_context::<Signal<Vec<NotificationData>>>();

    let mut install_pwa_fuc = move || {
        let str: String = installPWA().into_serde().unwrap();
        if str.to_string() == "safari".to_string() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "This app can only be installed on iOS via Safari.".to_string(),
                    notification_type: NotificationType::Error,
                });
                data // Return the updated data
            });
        }
        if str.to_string() == "ios".to_string() {
            notification_data.set({
                let mut data = notification_data().clone(); // Clone existing data
                data.push(NotificationData {
                    title: "".to_string(),
                    content: "To install this app on your iPhone/iPad, open with Safari, tap the Share button, and then tap 'Add to Home Screen'".to_string(),
                    notification_type: NotificationType::Success,
                });
                data // Return the updated data
            });
        }
    };

    rsx! {
        header { class: "py-5 bg-gray-900 z-50 h-min-[72px]",
            div { class: "w-4/5 mx-auto",
                div { class: "flex items-center justify-between",
                    Link { to: Route::MainMenu,
                        h1 { class: "text-2xl font-bold uppercase", "test-id" : "main-logo" , "DanceBattleZ" }
                    }
                    div { class: "flex items-center justify-end gap-2",
                        // desktop nav
                        nav { class: "hidden lg:flex items-center gap-5 text-xl/8",
                            // -------------------- Admin -------------------- //
                            if admin {
                                div { class: "relative group",
                                    a { class: "user-btn hover:text-blue-400 transition-all duration-300",
                                        i { class: "fa-solid fa-user" }
                                    }
                                    div { class: "pc absolute left-0 mt-0 hidden group-hover:flex bg-gray-500 p-2 rounded flex flex-col items-center gap-5 dropdown-menu z-[5]",
                                        Link { to: Route::AdminUser, class: "flex items-center justify-center",
                                            i { class: "fas fa-user" }
                                        }
                                        Link { to: Route::AdminBattle, class: "flex items-center justify-center",
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                width: "20",
                                                height: "20",
                                                fill: "#fff",
                                                "viewBox": "0 0 256 256",
                                                path { d: "M216,32H152a8,8,0,0,0-6.34,3.12l-64,83.21L72,108.69a16,16,0,0,0-22.64,0l-8.69,8.7a16,16,0,0,0,0,22.63l22,22-32,32a16,16,0,0,0,0,22.63l8.69,8.68a16,16,0,0,0,22.62,0l32-32,22,22a16,16,0,0,0,22.64,0l8.69-8.7a16,16,0,0,0,0-22.63l-9.64-9.64,83.21-64A8,8,0,0,0,224,104V40A8,8,0,0,0,216,32Zm-8,68.06-81.74,62.88L115.32,152l50.34-50.34a8,8,0,0,0-11.32-11.31L104,140.68,93.07,129.74,155.94,48H208Z" }
                                            }
                                        }
                                    }
                                }
                            }
                            if check_app {
                                button {
                                    class: "hover:text-yellow-400 transition-all duration-300",
                                    id: "app-install",
                                    onclick: move |_| install_pwa_fuc(),
                                    i { class: "fa-solid fa-download" }
                                }
                            }
                            Link {
                                to: Route::LiveBattle,
                                class: "hover:text-yellow-400 transition-all duration-300",
                                i { class: "fa-solid fa-tower-cell", "test-id" : "header-livebattle" }
                            }
                            Link {
                                to: Route::CallOut,
                                class: "hover:text-yellow-400 transition-all duration-300",
                                i { class: "fa-solid fa-hand-point-right", "test-id" : "header-callout"}
                            }
                            if auth_flag {
                                Link {
                                    to: Route::Response,
                                    class: "hover:text-yellow-400 transition-all duration-300",
                                    i { class: "fa fa-shield" , "test-id" : "header-reply" }
                                }
                            }
                            button {
                                class: "hover:text-yellow-400 transition-all duration-300",
                                onclick : move |_| {
                                    notification_data.set({
                                        let mut data = notification_data().clone(); // Clone existing data
                                        data.push(NotificationData {
                                            title: "".to_string(),
                                            content: "Coming Soon".to_string(),
                                            notification_type: NotificationType::Success,
                                        });
                                        data // Return the updated data
                                    });
                                },
                                i { class: "fa-solid fa-newspaper" }
                            }
                            Link {
                                to: Route::Badge,
                                class: "hover:text-yellow-400 transition-all duration-300",
                                i { class: "fa-solid fa-certificate" }
                            }
                            button {
                                class: "hover:text-yellow-400 transition-all duration-300",
                                onclick : move |_| {
                                    notification_data.set({
                                        let mut data = notification_data().clone(); // Clone existing data
                                        data.push(NotificationData {
                                            title: "".to_string(),
                                            content: "Coming Soon".to_string(),
                                            notification_type: NotificationType::Success,
                                        });
                                        data // Return the updated data
                                    });
                                },
                                i { class: "fa-solid fa-question" }
                            }
                            if auth_flag {
                                div {
                                    class: "hover:text-yellow-400 transition-all duration-300",
                                    id : "go-login",
                                    onclick: move |_| {
                                        set_local_storage("token", "");
                                        shared_data
                                            .set(SharedData {
                                                auth_flag: false,
                                                loading_flag: false,
                                            });
                                        navigation.push(Route::Login);
                                    },
                                    i { class: "fa-solid fa-arrow-right-from-bracket" }
                                }
                            }

                            if !auth_flag {
                                Link {
                                    to: Route::Login,
                                    id : "go-login",
                                    class: "hover:text-yellow-400 transition-all duration-300",
                                    i { class: "fa-solid fa-arrow-right-to-bracket" }
                                }
                            }
                        }
                        // mobile nav
                        div { class: "inline-block lg:hidden relative z-[10]",
                            button {
                                class: "bg-blue-400 px-2 py-1 rounded",
                                onclick: move |_| mobile_responsive.set(!mobile_responsive()),
                                i { class: "fa-solid fa-bars" }
                            }
                            if mobile_responsive() {
                                nav { class: "absolute top-10 left-0 bg-gray-500 p-2 rounded flex flex-col items-center gap-5 mobile-nav z-[10]",
                                    if admin {
                                        div { class: "relative group",
                                            a { class: "user-btn hover:text-blue-400 transition-all duration-300",
                                                i { class: "fa-solid fa-user" }
                                            }
                                            div { class: "absolute left-0 bg-gray-500 p-2 rounded flex flex-col items-center gap-5 dropdown-menu",
                                                Link {
                                                    to: Route::AdminUser,
                                                    class: "flex items-center justify-center px-2 py-2 hover:bg-gray-700 transition-all duration-300",
                                                    i { class: "fas fa-user" }
                                                }
                                                Link {
                                                    to: Route::AdminBattle,
                                                    class: "flex items-center justify-center px-2 py-2 hover:bg-gray-700 transition-all duration-300",
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        width: "20",
                                                        height: "20",
                                                        fill: "#fff",
                                                        "viewBox": "0 0 256 256",
                                                        path { d: "M216,32H152a8,8,0,0,0-6.34,3.12l-64,83.21L72,108.69a16,16,0,0,0-22.64,0l-8.69,8.7a16,16,0,0,0,0,22.63l22,22-32,32a16,16,0,0,0,0,22.63l8.69,8.68a16,16,0,0,0,22.62,0l32-32,22,22a16,16,0,0,0,22.64,0l8.69-8.7a16,16,0,0,0,0-22.63l-9.64-9.64,83.21-64A8,8,0,0,0,224,104V40A8,8,0,0,0,216,32Zm-8,68.06-81.74,62.88L115.32,152l50.34-50.34a8,8,0,0,0-11.32-11.31L104,140.68,93.07,129.74,155.94,48H208Z" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if check_app {
                                        button {
                                            class: "hover:text-yellow-400 transition-all duration-300",
                                            id: "app-install",
                                            onclick: move |_| install_pwa_fuc(),
                                            i { class: "fa-solid fa-download" }
                                        }
                                    }
                                    Link {
                                        to: Route::LiveBattle,
                                        class: "hover:text-yellow-400 transition-all duration-300",
                                        i { class: "fa-solid fa-tower-cell" }
                                    }
                                    Link {
                                        to: Route::CallOut,
                                        class: "hover:text-yellow-400 transition-all duration-300",
                                        i { class: "fa-solid fa-hand-point-right" }
                                    }
                                    if auth_flag {
                                        Link {
                                            to: Route::Response,
                                            class: "hover:text-yellow-400 transition-all duration-300",
                                            i { class: "fa fa-shield" }
                                        }
                                    }
                                    button {
                                        class: "hover:text-yellow-400 transition-all duration-300",
                                        onclick : move |_| {
                                            notification_data.set({
                                                let mut data = notification_data().clone(); // Clone existing data
                                                data.push(NotificationData {
                                                    title: "".to_string(),
                                                    content: "Coming Soon".to_string(),
                                                    notification_type: NotificationType::Success,
                                                });
                                                data // Return the updated data
                                            });
                                        },
                                        i { class: "fa-solid fa-newspaper" }
                                    }
                                    Link {
                                        to: Route::Badge,
                                        class: "hover:text-yellow-400 transition-all duration-300",
                                        i { class: "fa-solid fa-certificate" }
                                    }
                                    button {
                                        class: "hover:text-yellow-400 transition-all duration-300",
                                        onclick : move |_| {
                                            notification_data.set({
                                                let mut data = notification_data().clone(); // Clone existing data
                                                data.push(NotificationData {
                                                    title: "".to_string(),
                                                    content: "Coming Soon".to_string(),
                                                    notification_type: NotificationType::Success,
                                                });
                                                data // Return the updated data
                                            });
                                        },
                                        i { class: "fa-solid fa-question" }
                                    }
                                    if auth_flag {
                                        div {
                                            class: "hover:text-yellow-400 transition-all duration-300",
                                            id : "go-login",
                                            onclick: move |_| {
                                                set_local_storage("token", "");
                                                shared_data
                                                    .set(SharedData {
                                                        auth_flag: false,
                                                        loading_flag: false,
                                                    });
                                                navigation.push(Route::Login);
                                            },
                                            i { class: "fa-solid fa-arrow-right-from-bracket" }
                                        }
                                    }
                                    if !auth_flag {
                                        Link {
                                            to: Route::Login,
                                            id : "go-login",
                                            class: "hover:text-yellow-400 transition-all duration-300",
                                            i { class: "fa-solid fa-arrow-right-to-bracket" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

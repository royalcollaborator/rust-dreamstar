use dioxus::prelude::*;
// PR... I delete these NotificationData, NotificationType

#[component]
pub fn Pagination(pagination: Signal<i32>, max_page: Signal<i32>) -> Element {
    rsx! {
        div {
            class: "pagination absolute  bottom-1 w-full",
            div {
                class: "mb-1",
                div {
                    class: "flex items-center justify-center gap-3",
                    button {
                        class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                        onclick : move |_|  {
                            if pagination() -1 >= 1 {
                                pagination.set(pagination() - 1)
                            }
                        },
                        i {
                            class: "fa-solid fa-arrow-left",
                        }
                    }
                    // max page is more than 5
                    if max_page() > 5 {
                       if pagination() <= max_page() -3{
                        if pagination() == 1{
                            for i in 1..3{
                                button {
                                    class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                                    onclick : move |_|  pagination.set(i),
                                    "{i}"
                                }
                            }
                        } else {
                            for i in pagination()-1..pagination()+1{
                                button {
                                    class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                                    onclick : move |_|  pagination.set(i),
                                    "{i}"
                                }
                            }
                        }
                       } else {
                        for i in max_page()-3..max_page(){
                            button {
                                class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                                onclick : move |_|  pagination.set(i),
                                "{i}"
                            }
                        }
                       }
                    } else {
                    // max page less than 5
                        for i in 0..max_page(){
                            button {
                                class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                                onclick : move |_|  pagination.set(i+1),
                                "{i+1}"
                            }
                        }
                    }

                    button {
                        class: "flex items-center justify-center w-10 h-10 bg-gray-800 hover:bg-gray-700 transition-all duration-300 rounded-md",
                        onclick : move |_|  {
                            if pagination() +1 <= max_page() {
                                pagination.set(pagination() + 1)
                            }
                        },
                        i {
                            class: "fa-solid fa-arrow-right",
                        }
                    }

                    // End pagination
                }

            }
        }

    }
}

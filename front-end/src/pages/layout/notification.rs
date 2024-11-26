use crate::pages::layout::layout::{NotificationData, NotificationType};
use dioxus::prelude::*;
use gloo_timers::callback::Timeout;

#[component]
pub fn Notification(
    id: i32,
    title: String,
    content: String,
    notification_type: NotificationType,
    notification_instance: Signal<Vec<NotificationData>>,
    notification_data: Vec<NotificationData>,
) -> Element {
    use_effect(use_reactive(
        &notification_data.clone(),
        move |notification_data| {
            let timeout = Timeout::new(5000, move || {
                notification_instance.set({
                    let mut data = notification_data.clone(); // Clone existing data
                    data.remove(id as usize); // Remove the item at the specified index
                    data // Return the updated data
                });
            });
            timeout.forget();
        },
    ));
    rsx! {
        if notification_type == NotificationType::Error{
            div {
                id: "alert-border-2",
                    class: "w-full flex items-center p-4 mb-4 text-red-400 bg-gray-800 border-red-800 mr-[10px]",
                role: "alert",
                i { class: "fas fa-triangle-exclamation" }
                div {
                    class: "ms-3 text-sm font-medium",
                    "{content}"
                }
            }

        }
    if notification_type == NotificationType::Success {
        div {
            id: "alert-border-3",
            class: "w-full flex items-center p-4 mb-4 text-green-400 bg-gray-800 border-green-800 mr-[10px]",
            role: "alert",
            i { class: "fas fa-check-circle" }
            div {
                class: "ms-3 text-sm font-medium",
                "{content}"
            }
         }

    }
    }
}

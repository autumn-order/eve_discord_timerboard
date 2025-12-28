use dioxus::prelude::*;

#[component]
pub fn CreateFleetButton(mut show_create_modal: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "btn btn-primary w-full sm:w-auto",
            onclick: move |_| show_create_modal.set(true),
            "Create Fleet"
        }
    }
}

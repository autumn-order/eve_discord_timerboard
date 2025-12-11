use dioxus::prelude::*;

use crate::client::component::Page;

#[component]
pub fn Login() -> Element {
    rsx! {
        Page {
            class: "flex items-center justify-center w-full h-full",
            p {
                "This is a login page"
            }
        }
    }
}

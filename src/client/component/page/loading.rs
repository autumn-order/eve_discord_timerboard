use dioxus::prelude::*;

use crate::client::component::Page;

#[component]
pub fn LoadingPage() -> Element {
    rsx!(
        Page { class: "flex items-center justify-center",
            span {
                class: "loading loading-spinner loading-xl"
            }
        }
    )
}

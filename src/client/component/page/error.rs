use dioxus::document::{Meta, Title};
use dioxus::prelude::*;

use crate::client::component::Page;

#[component]
pub fn ErrorPage(status: u64, message: String) -> Element {
    let error = match status {
        403 => "Access Denied",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Error",
    };

    rsx!(
        Title { "{status} | {error}" }
        Meta {
            name: "description",
            content: "{message}"
        }
        Page { class: "flex items-center justify-center",
            div { class: "flex flex-col items-center gap-2",
                h1 { class: "text-3xl md:text-4xl", "{status} | {error}" }
                p { class:"text-xl", "{message}" }
            }
        }
    )
}

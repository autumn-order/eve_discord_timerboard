use dioxus::prelude::*;

use crate::client::component::page::ErrorPage;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx!(ErrorPage {
        status: 404,
        message: "The page you are looking for does not exist"
    })
}

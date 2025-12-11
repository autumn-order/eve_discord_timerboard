use dioxus::prelude::*;

use crate::client::router::Route;

/// Shared navbar component.
#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
        }

        Outlet::<Route> {}
    }
}

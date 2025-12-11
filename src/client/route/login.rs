use dioxus::prelude::*;

use crate::client::{component::Page, router::Route, store::user::UserState};

#[component]
pub fn Login() -> Element {
    let user_store = use_context::<Store<UserState>>();
    let nav = navigator();

    let user_logged_in = user_store.read().user.is_some();
    let fetch_completed = user_store.read().fetched;

    // Redirect authenticed user to home after fetch completes
    use_effect(use_reactive!(|(user_logged_in, fetch_completed)| {
        if user_logged_in && fetch_completed {
            nav.push(Route::Home {});
        }
    }));

    rsx! {
        Title { "Login | Black Rose Timerboard" }
        Page {
            class: "flex items-center justify-center w-full h-full",
            p {
                "This is a login page"
            }
        }
    }
}

use dioxus::prelude::*;

use crate::client::{constant::SITE_NAME, model::auth::AuthState, router::Route};

#[cfg(feature = "web")]
use crate::client::api::user::get_user;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LOGO: Asset = asset!(
    "/assets/logo.webp",
    AssetOptions::image().with_size(ImageSize::Manual {
        width: 256,
        height: 256
    })
);

#[component]
pub fn App() -> Element {
    let mut auth_state = use_context_provider(|| Signal::new(AuthState::default()));

    // Fetch user on first load
    #[cfg(feature = "web")]
    {
        let future = use_resource(move || async move {
            if auth_state.peek().is_initializing() {
                Some(get_user().await)
            } else {
                None
            }
        });

        if let Some(Some(result)) = &*future.read_unchecked() {
            let mut state = auth_state.write();
            *state = match result {
                Ok(Some(user)) => AuthState::Authenticated(user.clone()),
                Ok(None) => AuthState::NotLoggedIn,
                Err(e) => AuthState::Error(e.clone()),
            };
        }
    }

    rsx! {
        Title { "{SITE_NAME}" }
        document::Link { rel: "icon", href: FAVICON }
        document::Meta {
            name: "og:image",
            content: LOGO
        }
        document::Meta {
            name: "twitter:image",
            content: LOGO
        }
        document::Meta {
            name: "description",
            content: " Discord-based fleet timerboard for EVE Online "
        }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

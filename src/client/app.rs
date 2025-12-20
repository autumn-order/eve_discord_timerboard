use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::client::{constant::SITE_NAME, router::Route, store::user::UserState};

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
    // Initialize the user store
    let mut user_store = use_store(|| UserState {
        user: None,
        fetched: false,
    });

    // Fetch user on first load
    #[cfg(feature = "web")]
    {
        use crate::client::store::user::get_user;

        let future = use_resource(|| async move { get_user().await });

        match &*future.read_unchecked() {
            Some(Ok(user)) => {
                user_store.write().user = (*user).clone();
                user_store.write().fetched = true;
            }
            Some(Err(err)) => {
                tracing::error!(err);

                user_store.write().fetched = true;
            }
            None => (),
        }
    }

    // Make user store available globally via context
    use_context_provider(|| user_store);

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

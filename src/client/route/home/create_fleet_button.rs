use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{client::model::error::ApiError, model::category::FleetCategoryListItemDto};

#[cfg(feature = "web")]
use crate::client::api::user::get_user_manageable_categories;

#[component]
pub fn CreateFleetButton(guild_id: u64, mut show_create_modal: Signal<bool>) -> Element {
    let mut manageable_categories =
        use_signal(|| None::<Result<Vec<FleetCategoryListItemDto>, ApiError>>);

    #[cfg(feature = "web")]
    {
        let future =
            use_resource(move || async move { get_user_manageable_categories(guild_id).await });

        match &*future.read_unchecked() {
            Some(Ok(categories)) => {
                manageable_categories.set(Some(Ok(categories.clone())));
            }
            Some(Err(err)) => {
                tracing::error!("Failed to fetch categories: {}", err);
                manageable_categories.set(Some(Err(err.clone())));
            }
            None => (),
        }
    }

    let can_create = manageable_categories()
        .and_then(|result| result.ok())
        .map(|categories| !categories.is_empty())
        .unwrap_or(false);

    rsx! {
        if can_create {
            button {
                class: "btn btn-primary w-full",
                onclick: move |_| show_create_modal.set(true),
                "Create Fleet"
            }
        }
    }
}

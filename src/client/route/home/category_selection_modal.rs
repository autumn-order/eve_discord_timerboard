use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    client::{component::Modal, model::error::ApiError},
    model::category::FleetCategoryListItemDto,
};

#[cfg(feature = "web")]
use crate::client::api::user::get_user_manageable_categories;

/// Modal for selecting which fleet category to create a fleet in
#[component]
pub fn CategorySelectionModal(
    guild_id: u64,
    mut show: Signal<bool>,
    on_category_selected: EventHandler<i32>,
) -> Element {
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

    let categories = manageable_categories()
        .and_then(|result| result.ok())
        .unwrap_or_default();

    rsx! {
        Modal {
            show,
            title: "Select Fleet Category",
            prevent_close: false,
            div {
                class: "space-y-4",
                if categories.is_empty() {
                    div {
                        class: "text-center py-8",
                        p {
                            class: "text-base-content/70",
                            "No categories available for fleet creation."
                        }
                    }
                } else {
                    div {
                        class: "grid grid-cols-1 gap-3 max-h-96 overflow-y-auto",
                        for category in categories {
                            {
                                let category_id = category.id;
                                let category_name = category.name.clone();
                                rsx! {
                                    button {
                                        key: "{category_id}",
                                        class: "block w-full text-left p-4 rounded-box border border-base-300 hover:bg-base-200 hover:border-primary transition-all",
                                        onclick: move |_| {
                                            on_category_selected.call(category_id);
                                        },
                                        div {
                                            class: "font-medium text-lg",
                                            "{category_name}"
                                        }
                                        div {
                                            class: "text-sm text-base-content/70 mt-1",
                                            "Create a new fleet in this category"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

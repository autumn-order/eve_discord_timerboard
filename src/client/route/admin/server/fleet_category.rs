use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::client::{
    component::{
        page::{ErrorPage, LoadingPage},
        Page,
    },
    model::error::ApiError,
    router::Route,
};

use crate::model::discord::DiscordGuildDto;

use super::{ActionTabs, GuildInfoHeader, ServerAdminTab};

#[component]
pub fn ServerAdminFleetCategory(guild_id: u64) -> Element {
    let mut guild = use_context::<Signal<Option<DiscordGuildDto>>>();
    let mut error = use_signal(|| None::<ApiError>);

    // Fetch guild data using use_resource if not already cached
    #[cfg(feature = "web")]
    {
        use crate::client::route::admin::get_discord_guild_by_id;

        let future = use_resource(move || async move {
            // Only fetch if we don't have the guild data or if the guild_id doesn't match
            if guild.read().as_ref().map(|g| g.guild_id as u64) != Some(guild_id) {
                get_discord_guild_by_id(guild_id).await
            } else {
                // Return a dummy error to skip updating
                Err(ApiError {
                    status: 0,
                    message: "cached".to_string(),
                })
            }
        });

        match &*future.read_unchecked() {
            Some(Ok(guild_data)) => {
                guild.set(Some(guild_data.clone()));
                error.set(None);
            }
            Some(Err(err)) if err.status != 0 => {
                tracing::error!("Failed to fetch guild: {}", err);
                guild.set(None);
                error.set(Some(err.clone()));
            }
            _ => (),
        }
    }

    rsx! {
        Title { "Fleet Categories | Black Rose Timerboard" }
        if let Some(guild_data) = guild.read().clone() {
            Page {
                class: "flex flex-col items-center w-full h-full",
                div {
                    class: "w-full max-w-6xl",
                    Link {
                        to: Route::Admin {},
                        class: "btn btn-ghost mb-4",
                        "← Back to Servers"
                    }
                    GuildInfoHeader { guild_data: guild_data.clone() }
                    ActionTabs { guild_id, active_tab: ServerAdminTab::FleetCategories }
                    div {
                        class: "space-y-6",
                        FleetCategoriesSection { guild_id }
                    }
                }
            }
        } else if let Some(err) = error() {
            ErrorPage { status: err.status, message: err.message }
        } else {
            LoadingPage { }
        }
    }
}

#[component]
fn FleetCategoriesSection(guild_id: u64) -> Element {
    rsx!(
        div {
            class: "card bg-base-200",
            div {
                class: "card-body",
                div {
                    class: "flex justify-between items-center mb-4",
                    h2 {
                        class: "card-title",
                        "Fleet Categories"
                    }
                    button {
                        class: "btn btn-primary",
                        "Add Category"
                    }
                }
                div {
                    class: "text-center py-8 opacity-50",
                    "No fleet categories configured"
                }
            }
        }
    )
}

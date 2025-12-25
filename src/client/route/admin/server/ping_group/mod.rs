mod component;

use component::{modal::CreatePingGroupModal, table::PingGroupsTable};
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    client::{
        component::{
            page::{ErrorPage, LoadingPage},
            Page, Pagination,
        },
        constant::SITE_NAME,
        model::error::ApiError,
        route::admin::server::{ActionTabs, GuildInfoHeader, PingGroupsCache, ServerAdminTab},
        router::Route,
    },
    model::discord::DiscordGuildDto,
};

#[cfg(feature = "web")]
use crate::client::api::{
    discord_guild::get_discord_guild_by_id, ping_group::get_paginated_ping_groups,
};

#[component]
pub fn ServerAdminPingGroup(guild_id: u64) -> Element {
    let mut guild = use_context::<Signal<Option<DiscordGuildDto>>>();
    let mut error = use_signal(|| None::<ApiError>);

    // Fetch guild data using use_resource if not already cached
    #[cfg(feature = "web")]
    {
        let mut should_fetch = use_signal(|| false);

        // Check cache and initiate fetch if needed
        use_effect(use_reactive!(|guild_id| {
            // Skip if already fetching
            if should_fetch() {
                return;
            }

            // Only run resource if we need to fetch
            let needs_fetch = guild.read().as_ref().map(|g| g.guild_id) != Some(guild_id);

            if needs_fetch {
                should_fetch.set(true);
            }
        }));

        let future = use_resource(move || async move {
            if should_fetch() {
                Some(get_discord_guild_by_id(guild_id).await)
            } else {
                None
            }
        });

        use_effect(move || {
            if let Some(Some(result)) = future.read_unchecked().as_ref() {
                match result {
                    Ok(guild_data) => {
                        guild.set(Some(guild_data.clone()));
                        error.set(None);
                    }
                    Err(err) => {
                        tracing::error!("Failed to fetch guild: {}", err);
                        guild.set(None);
                        error.set(Some(err.clone()));
                    }
                }
            }
        });
    }

    rsx!(
        Title { "Ping Groups | {SITE_NAME}" }
        if let Some(guild_data) = guild.read().clone() {
            Page {
                class: "flex flex-col items-center w-full h-full",
                div {
                    class: "w-full max-w-6xl",
                    Link {
                        to: Route::AdminServers {},
                        class: "btn btn-ghost mb-4",
                        "← Back to Servers"
                    }
                    GuildInfoHeader { guild_data: guild_data.clone() }
                    ActionTabs { guild_id, active_tab: ServerAdminTab::PingGroup }
                    div {
                        class: "space-y-6",
                        PingGroupSection { guild_id }
                    }
                }
            }
        } else if let Some(err) = error() {
            ErrorPage { status: err.status, message: err.message }
        } else {
            LoadingPage { }
        }
    )
}

#[component]
fn PingGroupSection(guild_id: u64) -> Element {
    let mut cache = use_context::<Signal<PingGroupsCache>>();
    let mut error = use_signal(|| None::<ApiError>);
    let mut show_create_modal = use_signal(|| false);

    let page = use_signal(|| cache.read().page);
    let per_page = use_signal(|| cache.read().per_page);
    let refetch_trigger = use_signal(|| 0u32);

    // Fetch ping formats - resource automatically re-runs when page(), per_page(), or refetch_trigger changes
    #[cfg(feature = "web")]
    let future = use_resource(move || async move {
        let _ = refetch_trigger();
        get_paginated_ping_groups(guild_id, page(), per_page()).await
    });

    #[cfg(feature = "web")]
    use_effect(move || {
        if let Some(result) = future.read_unchecked().as_ref() {
            match result {
                Ok(data) => {
                    // Update cache
                    cache.write().guild_id = guild_id;
                    cache.write().data = Some(data.clone());
                    cache.write().page = page();
                    cache.write().per_page = per_page();
                    error.set(None);
                }
                Err(err) => {
                    tracing::error!("Failed to fetch ping groups: {}", err);
                    cache.write().data = None;
                    error.set(Some(err.clone()));
                }
            }
        }
    });

    rsx!(
        div {
            class: "card bg-base-200",
            div {
                class: "card-body",
                div {
                    class: "flex justify-between items-center mb-4",
                    h2 {
                        class: "card-title",
                        "Ping Groups"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| show_create_modal.set(true),
                        "Add Ping Group"
                    }
                }

                if let Some(data) = cache.read().data.clone() {
                    if data.items.is_empty() {
                        div {
                            class: "text-center py-8 opacity-50",
                            "No ping groups configured"
                        }
                    } else {
                        PingGroupsTable {
                            data: data.clone(),
                            guild_id,
                            cache,
                            refetch_trigger
                        }
                        Pagination {
                            page,
                            per_page,
                            data: data.into(),
                            on_page_change: move |new_page| {
                                cache.write().page = new_page;
                            },
                            on_per_page_change: move |new_per_page| {
                                cache.write().per_page = new_per_page;
                                cache.write().page = 0;
                            },
                        }
                    }
                } else if let Some(err) = error() {
                    div {
                        class: "alert alert-error",
                        span { "Error loading ping groups: {err.message}"}
                    }
                } else {
                    div {
                        class: "text-center py-8",
                        span { class: "loading loading-spinner loading-lg" }
                    }
                }

                CreatePingGroupModal {
                    guild_id,
                    show: show_create_modal,
                    refetch_trigger
                }
            }
        }
    )
}

use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    client::{
        component::{
            page::{ErrorPage, LoadingPage},
            Modal, Page,
        },
        model::error::ApiError,
    },
    model::{category::FleetCategoryListItemDto, discord::DiscordGuildDto},
};

#[cfg(feature = "web")]
use crate::client::api::user::{get_user_guilds, get_user_manageable_categories};

#[component]
pub fn Home() -> Element {
    let mut guilds = use_signal(|| None::<Result<Vec<DiscordGuildDto>, ApiError>>);
    let mut selected_guild_id = use_signal(|| None::<u64>);
    let mut show_guild_dropdown = use_signal(|| false);
    let show_create_modal = use_signal(|| false);

    // Fetch user's guilds on first load
    #[cfg(feature = "web")]
    {
        let future = use_resource(|| async move { get_user_guilds().await });

        match &*future.read_unchecked() {
            Some(Ok(guild_list)) => {
                guilds.set(Some(Ok(guild_list.clone())));

                // Auto-select first guild (lowest ID) if nothing selected yet
                if selected_guild_id().is_none() && !guild_list.is_empty() {
                    // Find guild with lowest guild_id
                    let first_guild = guild_list.iter().min_by_key(|g| g.guild_id);

                    if let Some(guild) = first_guild {
                        selected_guild_id.set(Some(guild.guild_id as u64));
                    }
                }
            }
            Some(Err(err)) => {
                tracing::error!("Failed to fetch guilds: {}", err);
                guilds.set(Some(Err(err.clone())));
            }
            None => (),
        }
    }

    // Get selected guild name for display
    let selected_guild = guilds().and_then(|result| {
        result.ok().and_then(|guild_list| {
            selected_guild_id()
                .and_then(|id| guild_list.into_iter().find(|g| g.guild_id as u64 == id))
        })
    });

    rsx! {
        Title { "Black Rose Timerboard" }
        if let Some(Ok(guild_list)) = guilds() {
            if guild_list.is_empty() {
                // No guilds available
                Page {
                    class: "flex items-center justify-center w-full h-full",
                    div {
                        h2 {
                            class: "card-title justify-center text-xl mb-4",
                            "No Timerboards Available"
                        }
                        p {
                            class: "mb-4",
                            "You don't have access to any timerboards."
                        }
                    }
                }
            } else {
                // Has guilds
                Page {
                    class: "flex flex-col items-center w-full h-full",
                    div {
                        class: "w-full max-w-6xl px-4 py-6",

                        // Server selector header
                        div {
                            class: "mb-6",
                            div {
                                class: "flex flex-wrap items-center justify-between gap-4",

                                // Clickable guild header with dropdown
                                div {
                                    class: "relative",
                                    if let Some(guild) = selected_guild.clone() {
                                        button {
                                            class: "flex items-center gap-3 hover:opacity-80 transition-opacity",
                                            onclick: move |_| show_guild_dropdown.set(!show_guild_dropdown()),
                                            if let Some(icon_hash) = &guild.icon_hash {
                                                img {
                                                    src: "https://cdn.discordapp.com/icons/{guild.guild_id}/{icon_hash}.png",
                                                    alt: "{guild.name} icon",
                                                    class: "w-10 h-10 rounded-full",
                                                }
                                            } else {
                                                div {
                                                    class: "w-10 h-10 rounded-full bg-base-300 flex items-center justify-center font-bold",
                                                    "{guild.name.chars().next().unwrap_or('?')}"
                                                }
                                            }
                                            h1 {
                                                class: "text-xl font-bold",
                                                "{guild.name}"
                                            }
                                            // Chevron icon
                                            svg {
                                                class: "w-5 h-5 transition-transform",
                                                class: if show_guild_dropdown() { "rotate-180" },
                                                xmlns: "http://www.w3.org/2000/svg",
                                                fill: "none",
                                                view_box: "0 0 24 24",
                                                stroke: "currentColor",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M19 9l-7 7-7-7"
                                                }
                                            }
                                        }
                                    }

                                    // Guild dropdown menu
                                    if show_guild_dropdown() {
                                        div {
                                            class: "absolute top-full left-0 mt-2 w-80 bg-base-100 rounded-box shadow-lg border border-base-300 z-50",
                                            div {
                                                class: "p-2",
                                                div {
                                                    class: "max-h-96 overflow-y-auto",
                                                    for guild in guild_list.clone() {
                                                        {
                                                            let guild_id = guild.guild_id as u64;
                                                            let is_selected = selected_guild_id() == Some(guild_id);
                                                            rsx! {
                                                                button {
                                                                    key: "{guild_id}",
                                                                    class: "w-full flex items-center gap-3 p-3 rounded-box hover:bg-base-200 transition-colors",
                                                                    class: if is_selected { "bg-base-200" },
                                                                    onclick: move |_| {
                                                                        selected_guild_id.set(Some(guild_id));
                                                                        show_guild_dropdown.set(false);
                                                                    },
                                                                    if let Some(icon) = guild.icon_hash.as_ref() {
                                                                        img {
                                                                            src: "https://cdn.discordapp.com/icons/{guild_id}/{icon}.png",
                                                                            alt: "{guild.name} icon",
                                                                            class: "w-10 h-10 rounded-full",
                                                                        }
                                                                    } else {
                                                                        div {
                                                                            class: "w-10 h-10 rounded-full bg-base-300 flex items-center justify-center font-bold",
                                                                            "{guild.name.chars().next().unwrap_or('?')}"
                                                                        }
                                                                    }
                                                                    div {
                                                                        class: "flex-1 text-left",
                                                                        div {
                                                                            class: "font-medium",
                                                                            "{guild.name}"
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

                                // Create Fleet Button
                                div {
                                    class: "w-full sm:w-auto",
                                    if let Some(guild_id) = selected_guild_id() {
                                        CreateFleetButton {
                                            guild_id,
                                            show_create_modal
                                        }
                                    }
                                }
                            }
                        }

                        // Timerboard content (placeholder)
                        div {
                            class: "flex items-center justify-center min-h-[400px]",
                            if let Some(guild) = selected_guild.clone() {
                                div {
                                    class: "text-center",
                                    h2 {
                                        class: "text-3xl font-bold opacity-50",
                                        "{guild.name}"
                                    }
                                    p {
                                        class: "text-sm opacity-30 mt-2",
                                        "Timerboard content coming soon"
                                    }
                                }
                            } else {
                                p {
                                    class: "opacity-50",
                                    "Select a server to view its timerboard"
                                }
                            }
                        }
                    }
                }

                // Create Fleet Modal
                if let Some(guild_id) = selected_guild_id() {
                    CreateFleetModal {
                        guild_id,
                        show: show_create_modal
                    }
                }
            }
        } else if let Some(Err(error)) = guilds() {
            ErrorPage { status: error.status, message: error.message }
        } else {
            LoadingPage { }
        }

        // Click outside to close dropdown
        if show_guild_dropdown() {
            div {
                class: "fixed inset-0 z-40",
                onclick: move |_| show_guild_dropdown.set(false),
            }
        }
    }
}

#[component]
fn CreateFleetButton(guild_id: u64, mut show_create_modal: Signal<bool>) -> Element {
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

#[component]
fn CreateFleetModal(guild_id: u64, mut show: Signal<bool>) -> Element {
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
            title: "Create Fleet",
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
                                    a {
                                        key: "{category_id}",
                                        href: "/fleets/create?guild_id={guild_id}&category_id={category_id}",
                                        class: "block p-4 rounded-box border border-base-300 hover:bg-base-200 hover:border-primary transition-all",
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

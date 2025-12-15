use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    client::{
        component::{
            page::{ErrorPage, LoadingPage},
            DropdownItem, Page, SearchableDropdown,
        },
        model::error::ApiError,
    },
    model::discord::DiscordGuildDto,
};

#[cfg(feature = "web")]
use crate::client::api::user::get_user_guilds;

#[component]
pub fn Home() -> Element {
    let mut guilds = use_signal(|| None::<Result<Vec<DiscordGuildDto>, ApiError>>);
    let mut selected_guild_id = use_signal(|| None::<u64>);
    let search_query = use_signal(|| String::new());
    let mut show_dropdown = use_signal(|| false);

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

    // Filter guilds based on search query
    let filtered_guilds = guilds().and_then(|result| {
        result.ok().map(|guild_list| {
            let query = search_query().to_lowercase();
            if query.is_empty() {
                guild_list
            } else {
                guild_list
                    .into_iter()
                    .filter(|guild| {
                        guild.name.to_lowercase().contains(&query)
                            || guild.guild_id.to_string().contains(&query)
                    })
                    .collect()
            }
        })
    });

    // Get selected guild name for display
    let selected_guild = guilds().and_then(|result| {
        result.ok().and_then(|guild_list| {
            selected_guild_id()
                .and_then(|id| guild_list.into_iter().find(|g| g.guild_id as u64 == id))
        })
    });

    let has_filtered_guilds = filtered_guilds
        .as_ref()
        .map(|g| !g.is_empty())
        .unwrap_or(false);

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

                        // Server selector
                        div {
                            class: "mb-6",
                            div {
                                class: "flex justify-between items-center gap-4",
                                if let Some(guild) = selected_guild.clone() {
                                    div {
                                        class: "flex items-center gap-3",
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
                                    }
                                }

                                // Guild selector dropdown
                                div {
                                    class: "form-control flex-1 max-w-xs",
                                    SearchableDropdown {
                                        search_query,
                                        placeholder: "Search servers...".to_string(),
                                        display_value: selected_guild.as_ref().map(|g| g.name.clone()),
                                        disabled: false,
                                        required: false,
                                        empty_message: "No servers available".to_string(),
                                        not_found_message: "No servers found matching your search".to_string(),
                                        has_items: has_filtered_guilds,
                                        show_dropdown_signal: show_dropdown,
                                        if let Some(guilds) = filtered_guilds {
                                            for guild in guilds {
                                                {
                                                    let guild_id = guild.guild_id as u64;
                                                    let guild_name = guild.name.clone();
                                                    let icon_hash = guild.icon_hash.clone();
                                                    let is_selected = selected_guild_id() == Some(guild_id);
                                                    rsx! {
                                                        DropdownItem {
                                                            key: "{guild_id}",
                                                            selected: is_selected,
                                                            on_select: move |_| {
                                                                selected_guild_id.set(Some(guild_id));
                                                                show_dropdown.set(false);
                                                            },
                                                            div {
                                                                class: "flex items-center gap-3",
                                                                if let Some(icon) = icon_hash.as_ref() {
                                                                    img {
                                                                        src: "https://cdn.discordapp.com/icons/{guild_id}/{icon}.png",
                                                                        alt: "{guild_name} icon",
                                                                        class: "w-8 h-8 rounded-full",
                                                                    }
                                                                } else {
                                                                    div {
                                                                        class: "w-8 h-8 rounded-full bg-base-300 flex items-center justify-center font-bold text-sm",
                                                                        "{guild_name.chars().next().unwrap_or('?')}"
                                                                    }
                                                                }
                                                                div {
                                                                    class: "flex flex-col",
                                                                    div {
                                                                        class: "font-medium",
                                                                        "{guild_name}"
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
                        }

                        // Timerboard content (placeholder)
                        div {
                            class: "flex items-center justify-center min-h-[400px]",
                            if let Some(guild) = selected_guild {
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
            }
        } else if let Some(Err(error)) = guilds() {
            ErrorPage { status: error.status, message: error.message }
        } else {
            LoadingPage { }
        }
    }
}

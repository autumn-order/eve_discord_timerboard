use chrono::{DateTime, Local, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    client::{
        component::{ConfirmationModal, Pagination, PaginationData},
        model::error::ApiError,
    },
    model::fleet::PaginatedFleetsDto,
};

#[cfg(feature = "web")]
use crate::client::api::fleet::{delete_fleet, get_fleets};

#[derive(Clone, Copy, PartialEq)]
pub enum SortField {
    Category,
    Name,
    Commander,
    Countdown,
    DateTimeUtc,
    DateTimeLocal,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Clone, PartialEq)]
pub struct FleetTableCache {
    pub page: u64,
    pub per_page: u64,
    pub sort_field: SortField,
    pub sort_order: SortOrder,
}

impl Default for FleetTableCache {
    fn default() -> Self {
        Self {
            page: 0,
            per_page: 10,
            sort_field: SortField::Countdown,
            sort_order: SortOrder::Ascending, // Earliest first (upcoming)
        }
    }
}

#[component]
pub fn FleetTable(guild_id: u64, mut refetch_trigger: Signal<u32>) -> Element {
    let mut cache = use_signal(FleetTableCache::default);
    let mut fleets = use_signal(|| None::<Result<PaginatedFleetsDto, ApiError>>);
    let mut show_delete_modal = use_signal(|| false);
    let mut fleet_to_delete = use_signal(|| None::<(i32, String)>);
    let mut is_deleting = use_signal(|| false);

    // Fetch fleets with pagination
    #[cfg(feature = "web")]
    let fetch_future = use_resource(move || async move {
        let _ = refetch_trigger(); // Read trigger to track changes
        let cache_val = cache();
        get_fleets(guild_id, cache_val.page, cache_val.per_page).await
    });

    #[cfg(feature = "web")]
    use_effect(move || match &*fetch_future.read_unchecked() {
        Some(Ok(data)) => {
            fleets.set(Some(Ok(data.clone())));
        }
        Some(Err(err)) => {
            tracing::error!("Failed to fetch fleets: {}", err);
            fleets.set(Some(Err(err.clone())));
        }
        None => (),
    });

    // Handle deletion
    #[cfg(feature = "web")]
    let delete_future = use_resource(move || async move {
        if is_deleting() {
            if let Some((id, _)) = fleet_to_delete() {
                Some(delete_fleet(guild_id, id).await)
            } else {
                None
            }
        } else {
            None
        }
    });

    #[cfg(feature = "web")]
    use_effect(move || {
        if let Some(Some(result)) = delete_future.read_unchecked().as_ref() {
            match result {
                Ok(_) => {
                    refetch_trigger.set(refetch_trigger() + 1);
                    show_delete_modal.set(false);
                    is_deleting.set(false);
                }
                Err(err) => {
                    tracing::error!("Failed to delete fleet: {}", err);
                    is_deleting.set(false);
                }
            }
        }
    });

    rsx! {
        if let Some(Ok(data)) = fleets() {
            if data.fleets.is_empty() {
                div {
                    class: "flex items-center justify-center min-h-[400px]",
                    div {
                        class: "text-center",
                        p {
                            class: "text-lg opacity-50",
                            "No fleets scheduled yet"
                        }
                        p {
                            class: "text-sm opacity-30 mt-2",
                            "Create your first fleet to get started"
                        }
                    }
                }
            } else {
                {
                    // Sort fleets
                    let mut sorted_fleets = data.fleets.clone();
                    let cache_val = cache();
                    match cache_val.sort_field {
                        SortField::Category => {
                            sorted_fleets.sort_by(|a, b| {
                                let cmp = a.category_name.cmp(&b.category_name);
                                if cache_val.sort_order == SortOrder::Descending {
                                    cmp.reverse()
                                } else {
                                    cmp
                                }
                            });
                        }
                        SortField::Name => {
                            sorted_fleets.sort_by(|a, b| {
                                let cmp = a.name.cmp(&b.name);
                                if cache_val.sort_order == SortOrder::Descending {
                                    cmp.reverse()
                                } else {
                                    cmp
                                }
                            });
                        }
                        SortField::Commander => {
                            sorted_fleets.sort_by(|a, b| {
                                let cmp = a.commander_name.cmp(&b.commander_name);
                                if cache_val.sort_order == SortOrder::Descending {
                                    cmp.reverse()
                                } else {
                                    cmp
                                }
                            });
                        }
                        SortField::Countdown => {
                            sorted_fleets.sort_by(|a, b| {
                                let cmp = a.fleet_time.cmp(&b.fleet_time);
                                if cache_val.sort_order == SortOrder::Descending {
                                    cmp.reverse()
                                } else {
                                    cmp
                                }
                            });
                        }
                        SortField::DateTimeUtc => {
                            sorted_fleets.sort_by(|a, b| {
                                let cmp = a.fleet_time.cmp(&b.fleet_time);
                                if cache_val.sort_order == SortOrder::Descending {
                                    cmp.reverse()
                                } else {
                                    cmp
                                }
                            });
                        }
                        SortField::DateTimeLocal => {
                            sorted_fleets.sort_by(|a, b| {
                                let cmp = a.fleet_time.cmp(&b.fleet_time);
                                if cache_val.sort_order == SortOrder::Descending {
                                    cmp.reverse()
                                } else {
                                    cmp
                                }
                            });
                        }
                    }

                    rsx! {
                        div {
                            class: "overflow-x-auto",
                            table {
                                class: "table table-zebra w-full",
                                thead {
                                    tr {
                                        SortableHeader {
                                            label: "Category",
                                            field: SortField::Category,
                                            cache
                                        }
                                        SortableHeader {
                                            label: "Fleet Name",
                                            field: SortField::Name,
                                            cache
                                        }
                                        SortableHeader {
                                            label: "Fleet Commander",
                                            field: SortField::Commander,
                                            cache
                                        }
                                        SortableHeader {
                                            label: "Countdown",
                                            field: SortField::Countdown,
                                            cache
                                        }
                                        SortableHeader {
                                            label: "Date & Time (UTC)",
                                            field: SortField::DateTimeUtc,
                                            cache
                                        }
                                        SortableHeader {
                                            label: "Date & Time (Local)",
                                            field: SortField::DateTimeLocal,
                                            cache
                                        }
                                        th { class: "text-right", "Actions" }
                                    }
                                }
                                tbody {
                                    for fleet in sorted_fleets {
                                        {
                                            let fleet_id = fleet.id;
                                            let fleet_name = fleet.name.clone();
                                            let fleet_time = fleet.fleet_time;
                                            let local_time: DateTime<Local> = fleet_time.with_timezone(&Local);

                                            rsx! {
                                                tr {
                                                    key: "{fleet_id}",
                                                    td { "{fleet.category_name}" }
                                                    td {
                                                        class: "font-semibold",
                                                        "{fleet.name}"
                                                    }
                                                    td { "{fleet.commander_name}" }
                                                    td {
                                                        FleetCountdown { fleet_time }
                                                    }
                                                    td {
                                                        class: "font-mono text-sm",
                                                        {fleet_time.format("%Y-%m-%d %H:%M").to_string()}
                                                    }
                                                    td {
                                                        class: "font-mono text-sm",
                                                        {local_time.format("%Y-%m-%d %H:%M").to_string()}
                                                    }
                                                    td {
                                                        div {
                                                            class: "flex gap-2 justify-end",
                                                            button {
                                                                class: "btn btn-sm btn-ghost",
                                                                title: "View Details",
                                                                "View"
                                                            }
                                                            button {
                                                                class: "btn btn-sm btn-error",
                                                                title: "Delete Fleet",
                                                                onclick: move |_| {
                                                                    fleet_to_delete.set(Some((fleet_id, fleet_name.clone())));
                                                                    show_delete_modal.set(true);
                                                                },
                                                                "Delete"
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

                        // Pagination
                        FleetTablePagination {
                            pagination_data: data.clone(),
                            cache
                        }
                    }
                }
            }
        } else if let Some(Err(error)) = fleets() {
            div {
                class: "alert alert-error",
                span { "Failed to load fleets: {error.message}" }
            }
        } else {
            div {
                class: "flex items-center justify-center min-h-[400px]",
                span { class: "loading loading-spinner loading-lg" }
            }
        }

        // Delete Confirmation Modal
        ConfirmationModal {
            show: show_delete_modal,
            title: "Delete Fleet".to_string(),
            message: rsx!(
                if let Some((_, name)) = fleet_to_delete() {
                    p {
                        class: "py-4",
                        "Are you sure you want to delete the fleet "
                        span { class: "font-bold", "\"{name}\"" }
                        "? This action cannot be undone."
                    }
                }
            ),
            confirm_text: "Delete".to_string(),
            confirm_class: "btn-error".to_string(),
            is_processing: is_deleting(),
            processing_text: "Deleting...".to_string(),
            on_confirm: move |_| {
                is_deleting.set(true);
            },
        }
    }
}

#[component]
fn SortableHeader(label: String, field: SortField, mut cache: Signal<FleetTableCache>) -> Element {
    let cache_val = cache();
    let is_active = cache_val.sort_field == field;
    let is_ascending = cache_val.sort_order == SortOrder::Ascending;

    rsx! {
        th {
            button {
                class: "flex items-center gap-2 hover:opacity-70 transition-opacity w-full justify-start",
                onclick: move |_| {
                    cache.with_mut(|c| {
                        if c.sort_field == field {
                            // Toggle order
                            c.sort_order = if c.sort_order == SortOrder::Ascending {
                                SortOrder::Descending
                            } else {
                                SortOrder::Ascending
                            };
                        } else {
                            // New field, default to ascending
                            c.sort_field = field;
                            c.sort_order = SortOrder::Ascending;
                        }
                        c.page = 0; // Reset to first page
                    });
                },
                span {
                    class: "whitespace-nowrap",
                    "{label}"
                }
                svg {
                    class: "w-4 h-4 flex-shrink-0 transition-opacity",
                    class: if is_active { "opacity-100" } else { "opacity-0" },
                    class: if is_ascending { "" } else { "rotate-180" },
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M5 15l7-7 7 7"
                    }
                }
            }
        }
    }
}

#[component]
fn FleetCountdown(fleet_time: DateTime<Utc>) -> Element {
    let now = Utc::now();
    let duration = fleet_time.signed_duration_since(now);
    let seconds = duration.num_seconds();

    let (text, class) = match seconds {
        // More than 30 minutes after start time
        s if s < -1800 => {
            let time_ago = if duration.num_hours().abs() > 0 {
                let hours = duration.num_hours().abs();
                format!("{} hour{}", hours, if hours == 1 { "" } else { "s" })
            } else {
                let minutes = duration.num_minutes().abs();
                format!("{} minute{}", minutes, if minutes == 1 { "" } else { "s" })
            };
            (format!("Started {} ago", time_ago), "text-neutral")
        }
        // 1-30 minutes after start time
        s if s < -60 => {
            let minutes = duration.num_minutes().abs();
            (
                format!(
                    "Started {} minute{} ago",
                    minutes,
                    if minutes == 1 { "" } else { "s" }
                ),
                "text-error font-bold",
            )
        }
        // Within ±1 minute of start time (the critical window)
        s if s < 60 => ("Starting now".to_string(), "text-error font-bold"),
        // 1-60 minutes before start time (show minutes to avoid "in 0 hours")
        s if s < 3600 => {
            let minutes = duration.num_minutes();
            (
                format!(
                    "In {} minute{}",
                    minutes,
                    if minutes == 1 { "" } else { "s" }
                ),
                "text-warning",
            )
        }
        // 1-3 hours before start time
        s if s < 10800 => {
            let hours = duration.num_hours();
            (
                format!("In {} hour{}", hours, if hours == 1 { "" } else { "s" }),
                "text-warning",
            )
        }
        // More than 1 day before start time
        _ if duration.num_days() > 0 => {
            let days = duration.num_days();
            (
                format!("In {} day{}", days, if days == 1 { "" } else { "s" }),
                "",
            )
        }
        // 3-24 hours before start time
        _ => {
            let hours = duration.num_hours();
            (
                format!("In {} hour{}", hours, if hours == 1 { "" } else { "s" }),
                "",
            )
        }
    };

    rsx! {
        span {
            class: "{class}",
            "{text}"
        }
    }
}

#[component]
fn FleetTablePagination(
    pagination_data: PaginatedFleetsDto,
    mut cache: Signal<FleetTableCache>,
) -> Element {
    let data = PaginationData {
        page: pagination_data.page,
        per_page: pagination_data.per_page,
        total: pagination_data.total,
        total_pages: pagination_data.total_pages,
    };

    let page = use_signal(|| cache().page);
    let per_page = use_signal(|| cache().per_page);

    rsx! {
        Pagination {
            page,
            per_page,
            data,
            on_page_change: move |new_page| {
                cache.write().page = new_page;
            },
            on_per_page_change: move |new_per_page| {
                cache.write().per_page = new_per_page;
                cache.write().page = 0;
            },
        }
    }
}

use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    client::{
        component::modal::ConfirmationModal,
        route::admin::server::{component::duration::format_duration, PingGroupsCache},
    },
    model::ping_group::PaginatedPingGroupsDto,
};

use super::modal::EditPingGroupModal;

#[cfg(feature = "web")]
use crate::client::api::ping_group::delete_ping_group;

#[component]
pub fn PingGroupsTable(
    data: PaginatedPingGroupsDto,
    guild_id: u64,
    mut cache: Signal<PingGroupsCache>,
    mut refetch_trigger: Signal<u32>,
) -> Element {
    let mut sorted_groups = data.items.clone();
    sorted_groups.sort_by_key(|g| g.id);

    let mut show_delete_modal = use_signal(|| false);
    let mut group_to_delete = use_signal(|| None::<(i32, String)>);
    let mut is_deleting = use_signal(|| false);

    let mut show_edit_modal = use_signal(|| false);
    let mut group_to_edit = use_signal(|| None::<crate::model::ping_group::PingGroupDto>);

    // Handle deletion with use_resource
    #[cfg(feature = "web")]
    let delete_future = use_resource(move || async move {
        if is_deleting() {
            if let Some((id, _)) = group_to_delete() {
                Some(delete_ping_group(guild_id, id).await)
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
                    // Trigger refetch
                    refetch_trigger.set(refetch_trigger() + 1);
                    // Close modal (data persists for smooth animation)
                    show_delete_modal.set(false);
                    is_deleting.set(false);
                }
                Err(err) => {
                    tracing::error!("Failed to delete ping group: {}", err);
                    is_deleting.set(false);
                }
            }
        }
    });

    rsx!(
        div {
            class: "overflow-x-auto",
            table {
                class: "table table-zebra w-full",
                thead {
                    tr {
                        th { "Name" }
                        th { "Cooldown" }
                        th {
                            class: "text-right",
                            "Actions"
                        }
                    }
                }
                tbody {
                    for group in &sorted_groups {
                        {
                            let group_id = group.id;
                            let group_name = group.name.clone();
                            let group_clone_for_edit = group.clone();
                            let group_name_for_delete = group_name.clone();
                            let cooldown = group.cooldown;

                            rsx! {
                                tr {
                                    td { "{group.name}" }
                                    td {
                                        if let Some(cd) = cooldown {
                                            span { "{format_duration(&cd)}" }
                                        } else {
                                            span { class: "opacity-50", "No cooldown" }
                                        }
                                    }
                                    td {
                                        div {
                                            class: "flex gap-2 justify-end",
                                            button {
                                                class: "btn btn-sm btn-primary",
                                                onclick: move |_| {
                                                    group_to_edit.set(Some(group_clone_for_edit.clone()));
                                                    show_edit_modal.set(true);
                                                },
                                                "Edit"
                                            }
                                            button {
                                                class: "btn btn-sm btn-error",
                                                onclick: move |_| {
                                                    group_to_delete.set(Some((group_id, group_name_for_delete.clone())));
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

        // Delete Confirmation Modal
        ConfirmationModal {
            show: show_delete_modal,
            title: "Delete Ping Group".to_string(),
            message: rsx!(
                if let Some((_, name)) = group_to_delete() {
                    div {
                        class: "py-4",
                        p {
                            "Are you sure you want to delete the ping group "
                            span { class: "font-bold", "\"{name}\"" }
                            "?"
                        }
                        p {
                            class: "mt-4",
                            "This action cannot be undone."
                        }
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

        // Edit Ping Group Modal
        EditPingGroupModal {
            guild_id,
            show: show_edit_modal,
            group_to_edit,
            refetch_trigger
        }
    )
}

use chrono::Duration;
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::client::component::Modal;

use super::{
    duration::{format_duration, parse_duration, validate_duration_input},
    form_fields::FleetCategoryFormFields,
};

#[cfg(feature = "web")]
use crate::client::api::fleet_category::{create_fleet_category, update_fleet_category};

#[component]
pub fn CreateCategoryModal(
    guild_id: u64,
    mut show: Signal<bool>,
    mut refetch_trigger: Signal<u32>,
) -> Element {
    let mut category_name = use_signal(|| String::new());
    let mut ping_cooldown_str = use_signal(|| String::new());
    let mut ping_reminder_str = use_signal(|| String::new());
    let mut max_pre_ping_str = use_signal(|| String::new());
    let mut submit_name = use_signal(|| String::new());
    let mut submit_ping_cooldown = use_signal(|| None::<Duration>);
    let mut submit_ping_reminder = use_signal(|| None::<Duration>);
    let mut submit_max_pre_ping = use_signal(|| None::<Duration>);
    let mut should_submit = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut ping_cooldown_error = use_signal(|| None::<String>);
    let mut ping_reminder_error = use_signal(|| None::<String>);
    let mut max_pre_ping_error = use_signal(|| None::<String>);

    // Reset form when modal opens (clears data from previous use)
    use_effect(move || {
        if show() {
            category_name.set(String::new());
            ping_cooldown_str.set(String::new());
            ping_reminder_str.set(String::new());
            max_pre_ping_str.set(String::new());
            submit_name.set(String::new());
            submit_ping_cooldown.set(None);
            submit_ping_reminder.set(None);
            submit_max_pre_ping.set(None);
            should_submit.set(false);
            error.set(None);
            ping_cooldown_error.set(None);
            ping_reminder_error.set(None);
            max_pre_ping_error.set(None);
        }
    });

    // Handle form submission with use_resource
    #[cfg(feature = "web")]
    let future = use_resource(move || async move {
        if should_submit() {
            Some(
                create_fleet_category(
                    guild_id,
                    submit_name(),
                    submit_ping_cooldown(),
                    submit_ping_reminder(),
                    submit_max_pre_ping(),
                )
                .await,
            )
        } else {
            None
        }
    });

    #[cfg(feature = "web")]
    use_effect(move || {
        if let Some(Some(result)) = future.read_unchecked().as_ref() {
            match result {
                Ok(_) => {
                    // Trigger refetch
                    refetch_trigger.set(refetch_trigger() + 1);
                    // Close modal (data persists for smooth animation)
                    show.set(false);
                    should_submit.set(false);
                }
                Err(err) => {
                    tracing::error!("Failed to create category: {}", err);
                    error.set(Some(err.message.clone()));
                    should_submit.set(false);
                }
            }
        }
    });

    let on_submit = move |evt: Event<FormData>| {
        evt.prevent_default();

        let name = category_name();
        if name.trim().is_empty() {
            error.set(Some("Category name is required".to_string()));
            return;
        }

        // Validate all duration fields before submitting
        let cooldown_err = validate_duration_input(&ping_cooldown_str());
        let reminder_err = validate_duration_input(&ping_reminder_str());
        let pre_ping_err = validate_duration_input(&max_pre_ping_str());

        ping_cooldown_error.set(cooldown_err.clone());
        ping_reminder_error.set(reminder_err.clone());
        max_pre_ping_error.set(pre_ping_err.clone());

        // Only submit if all validations pass
        if cooldown_err.is_none() && reminder_err.is_none() && pre_ping_err.is_none() {
            error.set(None);
            submit_name.set(name);
            submit_ping_cooldown.set(parse_duration(&ping_cooldown_str()));
            submit_ping_reminder.set(parse_duration(&ping_reminder_str()));
            submit_max_pre_ping.set(parse_duration(&max_pre_ping_str()));
            should_submit.set(true);
        } else {
            error.set(Some("Please fix the validation errors above".to_string()));
        }
    };

    let is_submitting = should_submit();
    let has_validation_errors = ping_cooldown_error().is_some()
        || ping_reminder_error().is_some()
        || max_pre_ping_error().is_some();

    rsx!(
        Modal {
            show,
            title: "Create Fleet Category".to_string(),
            prevent_close: is_submitting,
            form {
                class: "flex flex-col gap-4",
                onsubmit: on_submit,

                FleetCategoryFormFields {
                    category_name,
                    ping_cooldown_str,
                    ping_reminder_str,
                    max_pre_ping_str,
                    is_submitting,
                    ping_cooldown_error,
                    ping_reminder_error,
                    max_pre_ping_error
                }

                // Error Message
                if let Some(err) = error() {
                    div {
                        class: "alert alert-error mt-4",
                        span { "{err}" }
                    }
                }

                // Modal Actions
                div {
                    class: "modal-action",
                    button {
                        r#type: "button",
                        class: "btn",
                        onclick: move |_| show.set(false),
                        disabled: is_submitting,
                        "Cancel"
                    }
                    button {
                        r#type: "submit",
                        class: "btn btn-primary",
                        disabled: is_submitting || has_validation_errors,
                        if is_submitting {
                            span { class: "loading loading-spinner loading-sm mr-2" }
                            "Creating..."
                        } else {
                            "Create"
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn EditCategoryModal(
    guild_id: u64,
    mut show: Signal<bool>,
    category_to_edit: Signal<Option<crate::model::fleet::FleetCategoryDto>>,
    mut refetch_trigger: Signal<u32>,
) -> Element {
    let mut category_name = use_signal(|| String::new());
    let mut ping_cooldown_str = use_signal(|| String::new());
    let mut ping_reminder_str = use_signal(|| String::new());
    let mut max_pre_ping_str = use_signal(|| String::new());
    let mut submit_name = use_signal(|| String::new());
    let mut submit_ping_cooldown = use_signal(|| None::<Duration>);
    let mut submit_ping_reminder = use_signal(|| None::<Duration>);
    let mut submit_max_pre_ping = use_signal(|| None::<Duration>);
    let mut should_submit = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut category_id = use_signal(|| 0i32);
    let mut ping_cooldown_error = use_signal(|| None::<String>);
    let mut ping_reminder_error = use_signal(|| None::<String>);
    let mut max_pre_ping_error = use_signal(|| None::<String>);

    // Initialize form when modal opens with new data
    use_effect(move || {
        if show() {
            if let Some(category) = category_to_edit() {
                category_name.set(category.name.clone());
                category_id.set(category.id);
                ping_cooldown_str.set(
                    category
                        .ping_lead_time
                        .as_ref()
                        .map(|d| format_duration(d))
                        .unwrap_or_default(),
                );
                ping_reminder_str.set(
                    category
                        .ping_reminder
                        .as_ref()
                        .map(|d| format_duration(d))
                        .unwrap_or_default(),
                );
                max_pre_ping_str.set(
                    category
                        .max_pre_ping
                        .as_ref()
                        .map(|d| format_duration(d))
                        .unwrap_or_default(),
                );
                // Reset error and submit state when opening with new data
                error.set(None);
                should_submit.set(false);
                ping_cooldown_error.set(None);
                ping_reminder_error.set(None);
                max_pre_ping_error.set(None);
            }
        }
    });

    // Handle form submission with use_resource
    #[cfg(feature = "web")]
    let future = use_resource(move || async move {
        if should_submit() {
            Some(
                update_fleet_category(
                    guild_id,
                    category_id(),
                    submit_name(),
                    submit_ping_cooldown(),
                    submit_ping_reminder(),
                    submit_max_pre_ping(),
                )
                .await,
            )
        } else {
            None
        }
    });

    #[cfg(feature = "web")]
    use_effect(move || {
        if let Some(Some(result)) = future.read_unchecked().as_ref() {
            match result {
                Ok(_) => {
                    // Trigger refetch
                    refetch_trigger.set(refetch_trigger() + 1);
                    // Close modal (data persists for smooth animation)
                    show.set(false);
                    should_submit.set(false);
                }
                Err(err) => {
                    tracing::error!("Failed to update category: {}", err);
                    error.set(Some(err.message.clone()));
                    should_submit.set(false);
                }
            }
        }
    });

    let on_submit = move |evt: Event<FormData>| {
        evt.prevent_default();

        let name = category_name();
        if name.trim().is_empty() {
            error.set(Some("Category name is required".to_string()));
            return;
        }

        // Validate all duration fields before submitting
        let cooldown_err = validate_duration_input(&ping_cooldown_str());
        let reminder_err = validate_duration_input(&ping_reminder_str());
        let pre_ping_err = validate_duration_input(&max_pre_ping_str());

        ping_cooldown_error.set(cooldown_err.clone());
        ping_reminder_error.set(reminder_err.clone());
        max_pre_ping_error.set(pre_ping_err.clone());

        // Only submit if all validations pass
        if cooldown_err.is_none() && reminder_err.is_none() && pre_ping_err.is_none() {
            error.set(None);
            submit_name.set(name);
            submit_ping_cooldown.set(parse_duration(&ping_cooldown_str()));
            submit_ping_reminder.set(parse_duration(&ping_reminder_str()));
            submit_max_pre_ping.set(parse_duration(&max_pre_ping_str()));
            should_submit.set(true);
        } else {
            error.set(Some("Please fix the validation errors above".to_string()));
        }
    };

    let is_submitting = should_submit();
    let has_validation_errors = ping_cooldown_error().is_some()
        || ping_reminder_error().is_some()
        || max_pre_ping_error().is_some();

    rsx!(
        Modal {
            show,
            title: "Edit Fleet Category".to_string(),
            prevent_close: is_submitting,
            form {
                class: "flex flex-col gap-4",
                onsubmit: on_submit,

                FleetCategoryFormFields {
                    category_name,
                    ping_cooldown_str,
                    ping_reminder_str,
                    max_pre_ping_str,
                    is_submitting,
                    ping_cooldown_error,
                    ping_reminder_error,
                    max_pre_ping_error
                }

                // Error Message
                if let Some(err) = error() {
                    div {
                        class: "alert alert-error mt-4",
                        span { "{err}" }
                    }
                }

                // Modal Actions
                div {
                    class: "modal-action",
                    button {
                        r#type: "button",
                        class: "btn",
                        onclick: move |_| show.set(false),
                        disabled: is_submitting,
                        "Cancel"
                    }
                    button {
                        r#type: "submit",
                        class: "btn btn-primary",
                        disabled: is_submitting || has_validation_errors,
                        if is_submitting {
                            span { class: "loading loading-spinner loading-sm mr-2" }
                            "Updating..."
                        } else {
                            "Update"
                        }
                    }
                }
            }
        }
    )
}

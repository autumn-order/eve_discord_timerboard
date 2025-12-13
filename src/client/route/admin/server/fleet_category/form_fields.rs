use dioxus::prelude::*;

use super::duration::validate_duration_input;

/// Reusable form fields component for fleet category forms
#[component]
pub fn FleetCategoryFormFields(
    category_name: Signal<String>,
    ping_cooldown_str: Signal<String>,
    ping_reminder_str: Signal<String>,
    max_pre_ping_str: Signal<String>,
    is_submitting: bool,
    ping_cooldown_error: Signal<Option<String>>,
    ping_reminder_error: Signal<Option<String>>,
    max_pre_ping_error: Signal<Option<String>>,
) -> Element {
    rsx! {
        // Category Name Input
        div {
            class: "form-control w-full gap-3",
            label {
                class: "label",
                span {
                    class: "label-text",
                    "Category Name"
                }
            }
            input {
                r#type: "text",
                class: "input input-bordered w-full",
                placeholder: "e.g., Structure Timers",
                value: "{category_name()}",
                oninput: move |evt| category_name.set(evt.value()),
                disabled: is_submitting,
                required: true,
            }
        }

        // Ping Cooldown Input
        div {
            class: "form-control w-full gap-3",
            label {
                class: "label",
                span {
                    class: "label-text",
                    "Ping Cooldown (optional)"
                }
            }
            input {
                r#type: "text",
                class: if ping_cooldown_error().is_some() { "input input-bordered input-error w-full" } else { "input input-bordered w-full" },
                placeholder: "e.g., 1h, 30m, 1h30m",
                value: "{ping_cooldown_str()}",
                oninput: move |evt| {
                    let value = evt.value();
                    ping_cooldown_str.set(value.clone());
                    ping_cooldown_error.set(validate_duration_input(&value));
                },
                disabled: is_submitting,
            }
            if let Some(error) = ping_cooldown_error() {
                div {
                    class: "text-error text-sm mt-1",
                    "{error}"
                }
            }
            label {
                class: "label flex-col items-start gap-1",
                span {
                    class: "label-text-alt",
                    "Minimum amount of time between fleets"
                }
                span {
                    class: "label-text-alt text-xs",
                    "Format: 1h = 1 hour, 30m = 30 minutes, 1h30m = 1.5 hours"
                }
            }
        }

        // Ping Reminder Input
        div {
            class: "form-control w-full gap-3",
            label {
                class: "label",
                span {
                    class: "label-text",
                    "Ping Reminder (optional)"
                }
            }
            input {
                r#type: "text",
                class: if ping_reminder_error().is_some() { "input input-bordered input-error w-full" } else { "input input-bordered w-full" },
                placeholder: "e.g., 15m, 30m",
                value: "{ping_reminder_str()}",
                oninput: move |evt| {
                    let value = evt.value();
                    ping_reminder_str.set(value.clone());
                    ping_reminder_error.set(validate_duration_input(&value));
                },
                disabled: is_submitting,
            }
            if let Some(error) = ping_reminder_error() {
                div {
                    class: "text-error text-sm mt-1",
                    "{error}"
                }
            }
            label {
                class: "label flex-col items-start gap-1",
                span {
                    class: "label-text-alt",
                    "Reminder ping before fleet starts"
                }
                span {
                    class: "label-text-alt text-xs",
                    "Format: 1h = 1 hour, 30m = 30 minutes, 1h30m = 1.5 hours"
                }
            }
        }

        // Max Pre-Ping Input
        div {
            class: "form-control w-full gap-3",
            label {
                class: "label",
                span {
                    class: "label-text",
                    "Max Pre-Ping (optional)"
                }
            }
            input {
                r#type: "text",
                class: if max_pre_ping_error().is_some() { "input input-bordered input-error w-full" } else { "input input-bordered w-full" },
                placeholder: "e.g., 2h, 3h",
                value: "{max_pre_ping_str()}",
                oninput: move |evt| {
                    let value = evt.value();
                    max_pre_ping_str.set(value.clone());
                    max_pre_ping_error.set(validate_duration_input(&value));
                },
                disabled: is_submitting,
            }
            if let Some(error) = max_pre_ping_error() {
                div {
                    class: "text-error text-sm mt-1",
                    "{error}"
                }
            }
            label {
                class: "label flex-col items-start gap-1",
                span {
                    class: "label-text-alt",
                    "Maximum advance notice for pings"
                }
                span {
                    class: "label-text-alt text-xs",
                    "Format: 1h = 1 hour, 30m = 30 minutes, 1h30m = 1.5 hours"
                }
            }
        }
    }
}

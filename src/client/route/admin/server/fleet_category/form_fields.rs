use dioxus::prelude::*;

use crate::model::ping_format::PingFormatDto;

use super::duration::validate_duration_input;

/// Form field values
#[derive(Clone, Default, PartialEq)]
pub struct FormFieldsData {
    pub category_name: String,
    pub ping_format_id: Option<i32>,
    pub search_query: String,
    pub ping_cooldown_str: String,
    pub ping_reminder_str: String,
    pub max_pre_ping_str: String,
}

/// Validation errors for duration fields
#[derive(Clone, Default, PartialEq)]
pub struct ValidationErrorsData {
    pub ping_cooldown: Option<String>,
    pub ping_reminder: Option<String>,
    pub max_pre_ping: Option<String>,
}

/// Reusable form fields component for fleet category forms
#[component]
pub fn FleetCategoryFormFields(
    form_fields: Signal<FormFieldsData>,
    validation_errors: Signal<ValidationErrorsData>,
    is_submitting: bool,
    ping_formats: Signal<Vec<PingFormatDto>>,
) -> Element {
    let mut show_dropdown = use_signal(|| false);

    // Filter ping formats based on search query
    let filtered_formats = use_memo(move || {
        let formats = ping_formats();
        let query = form_fields().search_query.to_lowercase();
        if query.is_empty() {
            formats
        } else {
            formats
                .into_iter()
                .filter(|f| f.name.to_lowercase().contains(&query))
                .collect::<Vec<_>>()
        }
    });

    // Get selected format name
    let selected_format_name = use_memo(move || {
        let formats = ping_formats();
        if let Some(id) = form_fields().ping_format_id {
            formats.iter().find(|f| f.id == id).map(|f| f.name.clone())
        } else {
            None
        }
    });

    rsx! {
        // Category Name Input
        div {
            class: "form-control w-full flex flex-col gap-2",
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
                value: "{form_fields().category_name}",
                oninput: move |evt| {
                    form_fields.write().category_name = evt.value();
                },
                disabled: is_submitting,
                required: true,
            }
        }

        // Ping Format Dropdown with Search
        div {
            class: "form-control w-full flex flex-col gap-2",
            label {
                class: "label",
                span {
                    class: "label-text",
                    "Ping Format"
                }
            }
            div {
                class: "relative",
                input {
                    r#type: "text",
                    class: "input input-bordered w-full",
                    placeholder: if selected_format_name().is_some() { "" } else { "Search ping formats..." },
                    value: if show_dropdown() {
                        "{form_fields().search_query}"
                    } else if let Some(name) = &selected_format_name() {
                        "{name}"
                    } else {
                        ""
                    },
                    onfocus: move |_| {
                        show_dropdown.set(true);
                        form_fields.write().search_query = String::new();
                    },
                    onblur: move |_| {
                        show_dropdown.set(false);
                    },
                    oninput: move |evt| {
                        form_fields.write().search_query = evt.value();
                        show_dropdown.set(true);
                    },
                    disabled: is_submitting,
                    required: true,
                }
                {
                    let formats = filtered_formats();
                    if show_dropdown() {
                        if !formats.is_empty() {
                            rsx! {
                                div {
                                    class: "absolute z-10 w-full mt-1 bg-base-100 border border-base-300 rounded-lg shadow-lg max-h-60 overflow-y-auto",
                                    for format in formats {
                                        div {
                                            key: "{format.id}",
                                            class: if Some(format.id) == form_fields().ping_format_id {
                                                "px-4 py-2 cursor-pointer bg-primary text-primary-content hover:bg-primary-focus"
                                            } else {
                                                "px-4 py-2 cursor-pointer hover:bg-base-200"
                                            },
                                            onmousedown: move |evt| {
                                                evt.prevent_default();
                                                form_fields.write().ping_format_id = Some(format.id);
                                                form_fields.write().search_query = String::new();
                                                show_dropdown.set(false);
                                            },
                                            "{format.name}"
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div {
                                    class: "absolute z-10 w-full mt-1 bg-base-100 border border-base-300 rounded-lg shadow-lg",
                                    div {
                                        class: "px-4 py-2 text-center opacity-50",
                                        if !form_fields().search_query.is_empty() {
                                            "No ping formats found"
                                        } else {
                                            "No ping formats available"
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }
            label {
                class: "label",
                span {
                    class: "label-text-alt",
                    "Select the ping format to use for this fleet category"
                }
            }
        }

        // Ping Cooldown Input
        div {
            class: "form-control w-full flex flex-col gap-2",
            label {
                class: "label",
                span {
                    class: "label-text",
                    "Ping Cooldown (optional)"
                }
            }
            input {
                r#type: "text",
                class: if validation_errors().ping_cooldown.is_some() { "input input-bordered input-error w-full" } else { "input input-bordered w-full" },
                placeholder: "e.g., 1h, 30m, 1h30m",
                value: "{form_fields().ping_cooldown_str}",
                oninput: move |evt| {
                    let value = evt.value();
                    form_fields.write().ping_cooldown_str = value.clone();
                    validation_errors.write().ping_cooldown = validate_duration_input(&value);
                },
                disabled: is_submitting,
            }
            if let Some(error) = &validation_errors().ping_cooldown {
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
            class: "form-control w-full flex flex-col gap-2",
            label {
                class: "label",
                span {
                    class: "label-text",
                    "Ping Reminder (optional)"
                }
            }
            input {
                r#type: "text",
                class: if validation_errors().ping_reminder.is_some() { "input input-bordered input-error w-full" } else { "input input-bordered w-full" },
                placeholder: "e.g., 15m, 30m",
                value: "{form_fields().ping_reminder_str}",
                oninput: move |evt| {
                    let value = evt.value();
                    form_fields.write().ping_reminder_str = value.clone();
                    validation_errors.write().ping_reminder = validate_duration_input(&value);
                },
                disabled: is_submitting,
            }
            if let Some(error) = &validation_errors().ping_reminder {
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
            class: "form-control w-full flex flex-col gap-2",
            label {
                class: "label",
                span {
                    class: "label-text",
                    "Max Pre-Ping (optional)"
                }
            }
            input {
                r#type: "text",
                class: if validation_errors().max_pre_ping.is_some() { "input input-bordered input-error w-full" } else { "input input-bordered w-full" },
                placeholder: "e.g., 2h, 3h",
                value: "{form_fields().max_pre_ping_str}",
                oninput: move |evt| {
                    let value = evt.value();
                    form_fields.write().max_pre_ping_str = value.clone();
                    validation_errors.write().max_pre_ping = validate_duration_input(&value);
                },
                disabled: is_submitting,
            }
            if let Some(error) = &validation_errors().max_pre_ping {
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

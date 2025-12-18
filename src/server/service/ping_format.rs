//! Ping format service for business logic.
//!
//! This module provides the `PingFormatService` for managing ping format templates
//! and their fields. It orchestrates creation, updates, deletion, and queries while
//! working with domain models rather than DTOs.

use sea_orm::DatabaseConnection;

use crate::server::{
    data::{
        category::FleetCategoryRepository,
        ping_format::{field::PingFormatFieldRepository, PingFormatRepository},
    },
    error::AppError,
    model::ping_format::{
        CreatePingFormatFieldParam, CreatePingFormatParam, CreatePingFormatWithFieldsParam,
        DeletePingFormatParam, GetPaginatedPingFormatsParam, PaginatedPingFormats,
        PingFormatWithFields, UpdatePingFormatFieldParam, UpdatePingFormatParam,
        UpdatePingFormatWithFieldsParam,
    },
};

/// Service providing business logic for ping format management.
///
/// This struct holds a reference to the database connection and provides methods
/// for creating, updating, deleting, and querying ping format templates with their fields.
pub struct PingFormatService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> PingFormatService<'a> {
    /// Creates a new PingFormatService instance.
    ///
    /// # Arguments
    /// - `db` - Reference to the database connection
    ///
    /// # Returns
    /// - `PingFormatService` - New service instance
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Creates a new ping format with its fields.
    ///
    /// Creates a ping format template and all its associated fields in a coordinated
    /// operation. After creation, fetches the fleet category usage information to
    /// provide complete format metadata.
    ///
    /// # Arguments
    /// - `param` - Parameters containing guild ID, format name, and field definitions
    ///
    /// # Returns
    /// - `Ok(PingFormatWithFields)` - Created ping format with all fields and metadata
    /// - `Err(AppError::Database)` - Database error during creation
    pub async fn create(
        &self,
        param: CreatePingFormatWithFieldsParam,
    ) -> Result<PingFormatWithFields, AppError> {
        let format_repo = PingFormatRepository::new(self.db);
        let field_repo = PingFormatFieldRepository::new(self.db);

        // Create the ping format
        let ping_format = format_repo
            .create(CreatePingFormatParam {
                guild_id: param.guild_id,
                name: param.name,
            })
            .await?;

        // Create all the fields
        let mut result_fields = Vec::new();
        for (field_name, priority, default_value) in param.fields {
            let field = field_repo
                .create(CreatePingFormatFieldParam {
                    ping_format_id: ping_format.id,
                    name: field_name,
                    priority,
                    default_value,
                })
                .await?;
            result_fields.push(field);
        }

        // Get fleet category count
        let fleet_category_count = format_repo.get_fleet_category_count(ping_format.id).await?;

        // Get fleet categories using this ping format
        let category_repo = FleetCategoryRepository::new(self.db);
        let categories = category_repo.get_by_ping_format_id(ping_format.id).await?;
        let fleet_category_names: Vec<String> = categories.into_iter().map(|c| c.name).collect();

        Ok(PingFormatWithFields {
            ping_format,
            fields: result_fields,
            fleet_category_count,
            fleet_category_names,
        })
    }

    /// Gets paginated ping formats for a guild with all their fields.
    ///
    /// Retrieves ping formats for a specific guild with pagination, including all
    /// fields for each format and metadata about fleet category usage. Calculates
    /// total pages based on the per_page parameter and total format count.
    ///
    /// # Arguments
    /// - `param` - Parameters specifying guild ID, page number, and formats per page
    ///
    /// # Returns
    /// - `Ok(PaginatedPingFormats)` - Formats for the requested page with pagination metadata
    /// - `Err(AppError::Database)` - Database error during pagination query
    pub async fn get_paginated(
        &self,
        param: GetPaginatedPingFormatsParam,
    ) -> Result<PaginatedPingFormats, AppError> {
        let format_repo = PingFormatRepository::new(self.db);
        let field_repo = PingFormatFieldRepository::new(self.db);
        let category_repo = FleetCategoryRepository::new(self.db);

        let (ping_formats, total) = format_repo
            .get_all_by_guild_paginated(param.guild_id, param.page, param.per_page)
            .await?;

        let total_pages = if param.per_page > 0 {
            (total as f64 / param.per_page as f64).ceil() as u64
        } else {
            0
        };

        let mut ping_format_with_fields = Vec::new();
        for ping_format in ping_formats {
            let fields = field_repo.get_by_ping_format_id(ping_format.id).await?;

            let fleet_category_count = format_repo.get_fleet_category_count(ping_format.id).await?;

            // Get fleet categories using this ping format
            let categories = category_repo.get_by_ping_format_id(ping_format.id).await?;
            let fleet_category_names: Vec<String> =
                categories.into_iter().map(|c| c.name).collect();

            ping_format_with_fields.push(PingFormatWithFields {
                ping_format,
                fields,
                fleet_category_count,
                fleet_category_names,
            });
        }

        Ok(PaginatedPingFormats {
            ping_formats: ping_format_with_fields,
            total,
            page: param.page,
            per_page: param.per_page,
            total_pages,
        })
    }

    /// Updates a ping format's name and fields.
    ///
    /// Updates the ping format name and synchronizes the fields. Fields with an id
    /// will be updated, fields without an id will be created, and existing fields
    /// not in the update list will be deleted. Verifies the format belongs to the
    /// specified guild before allowing updates.
    ///
    /// # Arguments
    /// - `param` - Parameters containing format ID, guild ID, new name, and field updates
    ///
    /// # Returns
    /// - `Ok(Some(PingFormatWithFields))` - Updated ping format with all fields
    /// - `Ok(None)` - Ping format not found or doesn't belong to the guild
    /// - `Err(AppError::Database)` - Database error during update operations
    pub async fn update(
        &self,
        param: UpdatePingFormatWithFieldsParam,
    ) -> Result<Option<PingFormatWithFields>, AppError> {
        let format_repo = PingFormatRepository::new(self.db);
        let field_repo = PingFormatFieldRepository::new(self.db);

        // Check if ping format exists and belongs to the guild
        if !format_repo
            .exists_in_guild(param.id, param.guild_id)
            .await?
        {
            return Ok(None);
        }

        // Update the ping format
        let ping_format = format_repo
            .update(UpdatePingFormatParam {
                id: param.id,
                name: param.name,
            })
            .await?;

        // Get existing fields
        let existing_fields = field_repo.get_by_ping_format_id(ping_format.id).await?;

        // Determine which fields to keep, update, or create
        let mut updated_fields = Vec::new();
        let mut existing_field_ids: Vec<i32> = Vec::new();

        for (field_id, field_name, priority, default_value) in param.fields {
            if let Some(id) = field_id {
                // Update existing field
                let field = field_repo
                    .update(UpdatePingFormatFieldParam {
                        id,
                        name: field_name,
                        priority,
                        default_value,
                    })
                    .await?;
                existing_field_ids.push(id);
                updated_fields.push(field);
            } else {
                // Create new field
                let field = field_repo
                    .create(CreatePingFormatFieldParam {
                        ping_format_id: ping_format.id,
                        name: field_name,
                        priority,
                        default_value,
                    })
                    .await?;
                updated_fields.push(field);
            }
        }

        // Delete fields that are no longer present
        for existing_field in existing_fields {
            if !existing_field_ids.contains(&existing_field.id) {
                field_repo.delete(existing_field.id).await?;
            }
        }

        // Get fleet category count
        let fleet_category_count = format_repo.get_fleet_category_count(ping_format.id).await?;

        // Get fleet categories using this ping format
        let category_repo = FleetCategoryRepository::new(self.db);
        let categories = category_repo.get_by_ping_format_id(ping_format.id).await?;
        let fleet_category_names: Vec<String> = categories.into_iter().map(|c| c.name).collect();

        Ok(Some(PingFormatWithFields {
            ping_format,
            fields: updated_fields,
            fleet_category_count,
            fleet_category_names,
        }))
    }

    /// Deletes a ping format and all its fields.
    ///
    /// Verifies the format belongs to the specified guild and checks if any fleet
    /// categories are using this format. If categories are using it, returns an error
    /// to prevent orphaned references. Fields are automatically deleted by database
    /// cascade rules.
    ///
    /// # Arguments
    /// - `param` - Parameters containing the format ID and guild ID for verification
    ///
    /// # Returns
    /// - `Ok(true)` - Ping format was successfully deleted
    /// - `Ok(false)` - Ping format not found or doesn't belong to the guild
    /// - `Err(AppError::BadRequest)` - Fleet categories are still using this format
    /// - `Err(AppError::Database)` - Database error during deletion
    pub async fn delete(&self, param: DeletePingFormatParam) -> Result<bool, AppError> {
        let format_repo = PingFormatRepository::new(self.db);

        // Check if ping format exists and belongs to the guild
        if !format_repo
            .exists_in_guild(param.id, param.guild_id)
            .await?
        {
            return Ok(false);
        }

        // Check if there are any fleet categories using this ping format
        let fleet_category_count = format_repo.get_fleet_category_count(param.id).await?;
        if fleet_category_count > 0 {
            return Err(AppError::BadRequest(format!(
                "Cannot delete ping format: {} fleet {} still using this format. Please delete or reassign the {} first.",
                fleet_category_count,
                if fleet_category_count == 1 { "category is" } else { "categories are" },
                if fleet_category_count == 1 { "category" } else { "categories" }
            )));
        }

        // Delete the ping format (fields will be deleted by cascade)
        format_repo.delete(param.id).await?;

        Ok(true)
    }
}

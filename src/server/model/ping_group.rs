//! Domain & parameter models for ping group operations
//!
//! Defines the ping group domain model, the ping group parameter models,
//! and provides methods to convert the ping group domain model from entity
//! and into Dtos

use chrono::Duration;

use crate::{
    model::ping_group::{CreatePingGroupDto, PingGroupDto, UpdatePingGroupDto},
    server::{error::AppError, util::parse::parse_u64_from_string},
};

/// The ping group domain model
///
/// Defines the ping group format with an associated guild, name, and the configured
/// cooldown shared between all fleet categories part of the group if applicable
#[derive(Debug, Clone)]
pub struct PingGroup {
    pub id: i32,
    pub guild_id: u64,
    pub name: String,
    pub cooldown: Option<Duration>,
}

impl PingGroup {
    /// Converts an entity model to the ping group domain model
    ///
    /// # Arguments
    /// - `entity` - The entity model from the database
    ///
    /// # Returns
    /// - `Ok(PingGroup)` - Te converted ping format domain model
    /// - `Err(AppError::InternalError(ParseStringId))` - Failed to parse guild ID to u64
    pub fn from_entity(entity: entity::ping_group::Model) -> Result<Self, AppError> {
        let guild_id = parse_u64_from_string(entity.guild_id)?;

        Ok(Self {
            id: entity.id,
            guild_id,
            name: entity.name,
            cooldown: entity.cooldown.map(|s| Duration::seconds(s as i64)),
        })
    }

    pub fn into_dto(self) -> PingGroupDto {
        PingGroupDto {
            id: self.id,
            guild_id: self.guild_id,
            name: self.name,
            cooldown: self.cooldown,
        }
    }
}

/// Parameters for creating a new ping group
///
/// Creates a new ping group with the provided name and if applicable, a cooldown shared
/// between all fleet categories part of the group.
#[derive(Debug, Clone)]
pub struct CreatePingGroupParam {
    pub name: String,
    pub cooldown: Option<Duration>,
}

impl From<CreatePingGroupDto> for CreatePingGroupParam {
    fn from(dto: CreatePingGroupDto) -> Self {
        Self {
            name: dto.name,
            cooldown: dto.cooldown,
        }
    }
}

/// Parameters for updating an existing ping group
///
/// Updates a ping group with the provided name and if applicable, a cooldown shared
/// between all fleet categories part of the group.
#[derive(Debug, Clone)]
pub struct UpdatePingGroupParam {
    pub name: String,
    pub cooldown: Option<Duration>,
}

impl From<UpdatePingGroupDto> for UpdatePingGroupParam {
    fn from(dto: UpdatePingGroupDto) -> Self {
        Self {
            name: dto.name,
            cooldown: dto.cooldown,
        }
    }
}

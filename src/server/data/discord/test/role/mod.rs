use crate::server::data::discord::role::DiscordGuildRoleRepository;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter};
use serenity::all::{Role, RoleId};
use std::collections::HashMap;
use test_utils::{builder::TestBuilder, factory};

mod delete;
mod get_by_guild_id;
mod upsert;
mod upsert_many;

/// Helper function to create a test Serenity Role
fn create_test_role(role_id: u64, name: &str, color: u32, position: i16) -> Role {
    serde_json::from_value(serde_json::json!({
        "id": role_id.to_string(),
        "name": name,
        "color": color,
        "hoist": false,
        "icon": null,
        "unicode_emoji": null,
        "position": position,
        "permissions": "0",
        "managed": false,
        "mentionable": false,
    }))
    .expect("Failed to create test role")
}

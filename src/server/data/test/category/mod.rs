use crate::server::{
    data::category::FleetCategoryRepository,
    model::category::{AccessRoleData, CreateFleetCategoryParams, UpdateFleetCategoryParams},
};
use chrono::Duration;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter};
use test_utils::{builder::TestBuilder, factory};

mod create;
mod delete;
mod exists_in_guild;
mod get_by_guild_id_paginated;
mod get_by_id;
mod get_by_ping_format_id;
mod get_category_details;
mod update;
mod user_can_create_category;
mod user_can_manage_category;
mod user_can_view_category;

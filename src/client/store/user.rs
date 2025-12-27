use dioxus::prelude::*;

use crate::model::user::UserDto;

#[derive(Store)]
pub struct UserState {
    pub user: Option<UserDto>,
    pub fetched: bool,
}

use dioxus::prelude::*;

use crate::client::component::ProtectedLayout;
use crate::client::route::{timerboard::Home, Login};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/login")]
    Login {},

    #[layout(ProtectedLayout)]
    #[route("/")]
    Home {},
}

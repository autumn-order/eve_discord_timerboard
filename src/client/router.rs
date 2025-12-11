use dioxus::prelude::*;

use crate::client::component::{Layout, ProtectedLayout};
use crate::client::route::{timerboard::Home, Login, NotFound};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/login")]
    Login {},

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },

    #[layout(ProtectedLayout)]
    #[route("/")]
    Home {},
}

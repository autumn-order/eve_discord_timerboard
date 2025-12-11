use dioxus::prelude::*;

use crate::client::components::Header;
use crate::client::routes::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
    #[route("/")]
    Home {},
}

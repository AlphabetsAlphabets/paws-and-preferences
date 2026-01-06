use dioxus::prelude::*;
use dioxus::router::Routable;

use crate::components::{CatView, NavBar};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    CatView,
}

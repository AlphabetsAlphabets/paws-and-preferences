use dioxus::prelude::*;
use dioxus::router::Routable;

use crate::components::{CatView, Favorites, NavBar};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    CatView,

    #[route("/favorites")]
    Favorites,
}

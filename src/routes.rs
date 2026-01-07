use dioxus::prelude::*;
use dioxus::router::Routable;

use crate::components::CatView;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    CatView,
}

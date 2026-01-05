mod backend;
mod components;
mod routes;

use dioxus::prelude::*;

use crate::routes::Route;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}

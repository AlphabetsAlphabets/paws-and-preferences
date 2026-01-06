mod backend;
mod components;
mod routes;

use dioxus::prelude::*;
use routes::Route;

const CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}

mod backend;
mod components;
mod routes;

use dioxus::prelude::*;
use routes::Route;

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

// Next thing to do
// 0. Remove all the code in the `backend` part.
// 1. Add a list or vector to keep track of seen cats.
// 2. Keep track of which cats were liked and disliked.
// 3. Show a summary of which cats were liked and disliked.

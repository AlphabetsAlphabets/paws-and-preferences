use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center text-5xl mb-10",
            id: "title",
            Link {
                to: Route::CatView,
                h1 { "Cold Cats ♥️" }
            }
        }
        Outlet::<Route> {}
    }
}

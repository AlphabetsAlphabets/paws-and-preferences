use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
            id: "title",
            Link {
                to: Route::CatView,
                h1 { "Cold Cats ♥️" }
            }
        }
        Outlet::<Route> {}
    }
}

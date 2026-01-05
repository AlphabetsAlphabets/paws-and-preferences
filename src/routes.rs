use dioxus::prelude::*;
use dioxus::router::Routable;

use crate::backend;
use crate::components::{Favorites, NavBar};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}

#[allow(non_snake_case)]
fn DogView() -> Element {
    let mut image = use_resource(|| async move { backend::get_dog().await.unwrap_or_default() });
    rsx! {
        div {
            id: "dogview",
            img {
                id: "dogimage",
                src: image.cloned().unwrap_or_default()
            }
        }
        div {
            id: "buttons",
            button { onclick: move |_| image.restart(), id: "skip", "skip" },
            button {
                onclick: move |_| async move {
                    let image = image.cloned().unwrap_or_default();
                    if let Ok(_) = backend::save_dog(image).await {
                    }
                },
                id: "save",
                "save"
            }
        }
    }
}

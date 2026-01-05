use dioxus::prelude::*;

use crate::backend;

#[component]
pub fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "HotDog!"}
        }
    }
}

#[component]
pub fn Dogview() -> Element {
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

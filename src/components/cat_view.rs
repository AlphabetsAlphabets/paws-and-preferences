use dioxus::prelude::*;

use crate::backend;

static CATS_SEEN: GlobalSignal<u8> = Signal::global(|| 1);

#[allow(non_snake_case)]
pub fn CatView() -> Element {
    let image = use_resource(|| async move { backend::get_cat().await.unwrap_or_default() });

    if (*CATS_SEEN.resolve())() >= 5 {
        rsx! {
            Summary {}
        }
    } else {
        rsx! {
            CatImage { image: image }
            LikeDislike { image: image }
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn Summary() -> Element {
    rsx! {
        p { "You've seen 10 cats!" }
    }
}

#[allow(non_snake_case)]
#[component]
fn CatImage(image: Resource<String>) -> Element {
    rsx! {
        div {
            id: "catview",
            img {
                id: "catimage",
                src: image.cloned().unwrap_or_default()
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn LikeDislike(image: Resource<String>) -> Element {
    rsx! {
        div {
            id: "buttons",
            button { onclick: move |_| image.restart(), id: "dislike", "dislike" },
            button {
                onclick: move |_| async move {
                    let img_src = image.cloned().unwrap_or_default();

                    if let Ok(_) = backend::save_cat(img_src).await {
                        *CATS_SEEN.write() += 1;
                        image.restart();
                    }
                },
                id: "like",
                "like"
            }
        }

    }
}

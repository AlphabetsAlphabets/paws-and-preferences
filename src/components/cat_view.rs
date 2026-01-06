use std::collections::HashMap;

use dioxus::{logger::tracing, prelude::*};

use crate::backend;

static CATS_SEEN: GlobalSignal<u8> = Signal::global(|| 1);

#[derive(Clone, Debug)]
struct CatHistory(HashMap<String, bool>);

static CAT_HISTORY: GlobalSignal<CatHistory> = Signal::global(|| CatHistory(HashMap::new()));

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
    let history = (*CAT_HISTORY.read()).clone();
    let history = history.0;

    let likes: Vec<_> = history.iter().map(|entry| entry.0.clone()).collect();
    let dislikes: Vec<_> = history.iter().map(|entry| entry.0.clone()).collect();

    rsx! {
        h1 { "Likes "}
        for like in likes {
            img {
                src: like
            }
        },
        h1 { "Dislikes "}
        for dislike in dislikes {
            img {
                src: dislike
            }
        }
    }

    // rsx! {
    //     table {
    //         tr {
    //             th { "Likes" }
    //             th { "Dislikes" }
    //         }
    //         tr {
    //             img {
    //                 src: "hi"
    //             }
    //         }
    //     }
    // }
}

fn track_cats(url: String, like: bool) {
    tracing::info!("Image: {}\nLike: {}", url, like);

    let mut history = CAT_HISTORY.read().clone();
    history.0.entry(url).insert_entry(like);

    *CAT_HISTORY.write() = history;
    *CATS_SEEN.write() += 1;
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
            button {
                onclick: move |_| {
                    track_cats(image.cloned().unwrap_or_default(), false);
                    image.restart()
                },
                id: "dislike", "dislike"
            },
            button {
                onclick: move |_| async move {
                    let img_src = image.cloned().unwrap_or_default();
                    if let Ok(_) = backend::save_cat(img_src.clone()).await {
                        track_cats(img_src, true);
                        image.restart();
                    }
                },
                id: "like",
                "like"
            }
        }

    }
}

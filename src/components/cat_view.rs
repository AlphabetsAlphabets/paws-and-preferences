use std::collections::HashMap;

use dioxus::{fullstack::reqwest, logger::tracing, prelude::*};

static CATS_SEEN: GlobalSignal<u8> = Signal::global(|| 1);

#[derive(Clone, Debug)]
struct CatHistory(HashMap<String, bool>);

static CAT_HISTORY: GlobalSignal<CatHistory> = Signal::global(|| CatHistory(HashMap::new()));

#[derive(serde::Deserialize, serde::Serialize)]
struct CatApi {
    url: String,
}

async fn get_cat() -> Result<String> {
    let response = reqwest::get("https://cataas.com/cat?json=true")
        .await
        .unwrap();
    let result = response.json::<CatApi>().await.unwrap();

    Ok(result.url)
}

#[allow(non_snake_case)]
pub fn CatView() -> Element {
    let image = use_resource(|| async move { get_cat().await.unwrap_or_default() });

    if (*CATS_SEEN.resolve())() >= 5 {
        rsx! {
            Summary {}
        }
    } else {
        rsx! {
            div {
                class: "flex items-center justify-center mb-5",
                CatImage { image: image }
            }
            div {
                class: "flex items-center justify-center",
                LikeDislike { image: image }
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn Summary() -> Element {
    let history = (*CAT_HISTORY.read()).clone();
    let history = history.0;

    let likes: Vec<_> = history
        .iter()
        .filter(|entry| *entry.1)
        .map(|entry| entry.0.clone())
        .collect();

    let dislikes: Vec<_> = history
        .iter()
        .filter(|entry| !*entry.1)
        .map(|entry| entry.0.clone())
        .collect();

    rsx! {
        div {
            class: "grid grid-cols-2",
            div {
                class: "flex flex-col items-center text-center",
                h1 { class: "text-5xl", "Likes"},
                for like in likes {
                    img {
                        class: "w-[500px] h-[500px] object-contain p-5",
                        id: "catimage",
                        src: like
                    }
                },
            },
            div {
                class: "flex flex-col items-center text-center",
                h1 { class: "text-5xl", "Dislikes "}
                for dislike in dislikes {
                    img {
                        class: "w-[500px] h-[500px] object-contain p-5",
                        id: "catimage",
                        src: dislike
                    }
                }

            }
        }
    }
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
    let mut start_x = use_signal(|| 0.0);
    rsx! {
        div {
            img {
                class: "w-[500px] h-[500px] object-contain",
                ontouchstart: move |event| {
                    if let Some(touch) = event.data.touches_changed().first() {
                        start_x.set(touch.screen_coordinates().x);
                    }
                },
                ontouchend: move |event| {
                    if let Some(touch) = event.data.touches_changed().first() {
                        let end_x = touch.screen_coordinates().x;
                        let delta_x = end_x - start_x();
                        let threshold = 50.0;

                        if delta_x > threshold {
                            let img_src = image.cloned().unwrap_or_default();
                            track_cats(img_src, true);
                            image.restart();
                        } else if delta_x < -threshold {
                            track_cats(image.cloned().unwrap_or_default(), false);
                            image.restart()
                        }
                    }
                },
                src: image.cloned().unwrap_or_default()
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn LikeDislike(image: Resource<String>) -> Element {
    rsx! {
        button {
            class: "mr-5 hover:bg-red-500 p-5 text-4xl border border-solid",
            onclick: move |_| {
                track_cats(image.cloned().unwrap_or_default(), false);
                image.restart()
            },
            "dislike 😿"
        },
        button {
            class: "ml-5 hover:bg-violet-300 p-5 text-4xl border border-solid",
            onclick: move |_| async move {
                let img_src = image.cloned().unwrap_or_default();
                track_cats(img_src, true);
                image.restart();
            },
            "like 😻"
        }
    }
}

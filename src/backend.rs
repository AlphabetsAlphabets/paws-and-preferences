use dioxus::{fullstack::reqwest, logger::tracing, prelude::*};

const API: &str = "https://dog.ceo/api/breeds/image/random";

#[post("/api/save_dog")]
pub async fn save_dog(image: String) -> dioxus::Result<()> {
    tracing::info!("Dog {} saved!", image);
    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct DogApi {
    message: String,
}

#[get("/api/get_dog")]
pub async fn get_dog() -> dioxus::Result<String> {
    let response = reqwest::get(API).await.unwrap();
    let result = response.json::<DogApi>().await.unwrap();

    Ok(result.message)
}

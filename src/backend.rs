use dioxus::{fullstack::reqwest, logger::tracing, prelude::*};

#[post("/api/save_cat")]
pub async fn save_cat(image: String) -> dioxus::Result<()> {
    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct CatApi {
    url: String,
}

#[get("/api/get_cat")]
pub async fn get_cat() -> dioxus::Result<String> {
    let response = reqwest::get("https://cataas.com/cat?json=true")
        .await
        .unwrap();
    let result = response.json::<CatApi>().await.unwrap();

    Ok(result.url)
}

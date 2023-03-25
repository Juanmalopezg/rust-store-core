use std::net::SocketAddr;

use axum::{
    Json,
    Router, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Product {
    id: u32,
    title: String,
    price: f64,
    description: String,
    images: Vec<String>,
    #[serde(rename = "creationAt")]
    creation_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    category: Category,
}

#[derive(Debug, Deserialize, Serialize)]
struct Category {
    id: u32,
    name: String,
    image: String,
    #[serde(rename = "creationAt")]
    creation_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

async fn get_products() -> Json<Vec<Product>> {
    let url = "https://api.escuelajs.co/api/v1/products";
    let res = reqwest::get(url).await.expect("failed to get response");
    let products: Vec<Product> = res.json().await.expect("failed to parse response");
    Json(products)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/products", get(get_products));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



use std::net::SocketAddr;

use axum::{
    Json,
    Router, routing::get};
use axum::extract::Query;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

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

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    page: Option<i8>,
    limit: Option<i8>,
}

async fn get_products(Query(params): Query<Params>) -> Json<Vec<Product>> {
    let url = "https://api.escuelajs.co/api/v1/products";
    let res = reqwest::get(url).await.expect("failed to get response");
    let products: Vec<Product> = res.json().await.expect("failed to parse response");

    let start = (params.page.unwrap_or(1) - 1) * params.limit.unwrap_or(10);
    let end = start + params.limit.unwrap_or(100);
    let paginated_products = products.into_iter().skip(start as usize).take((end - start) as usize).collect();

    Json(paginated_products)
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/products", get(get_products).layer(
            CorsLayer::new().allow_origin(Any)
        ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



use std::cmp::Ordering;
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
    page: Option<u8>,
    limit: Option<u8>,
    #[serde(rename = "sortBy")]
    sort_by: Option<String>,
}

async fn get_products(Query(params): Query<Params>) -> Json<Vec<Product>> {
    let products = get_products_from_api().await;
    let products = filter_images(products);
    let products = sort_products(products, &params.sort_by);
    let products = paginate_products(products, &params);

    Json(products)
}

async fn get_products_from_api() -> Vec<Product> {
    let url = "https://api.escuelajs.co/api/v1/products";
    let res = reqwest::get(url).await.expect("failed to get response");
    let products: Vec<Product> = res.json().await.expect("failed to parse response");
    products
}

fn filter_images(mut products: Vec<Product>) -> Vec<Product> {
    for product in &mut products {
        if let Some(image) = product.images.pop() {
            product.images = vec![image];
        }
    }
    products
}

fn sort_products(mut products: Vec<Product>, sort_by: &Option<String>) -> Vec<Product> {
    if let Some(sort_by) = sort_by {
        match sort_by.as_str() {
            "asc" => {
                products.sort_by(|a, b| {
                    a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal)
                });
            }
            "desc" => {
                products.sort_by(|a, b| {
                    b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal)
                });
            }
            _ => {}
        }
    }
    products
}

fn paginate_products(products: Vec<Product>, params: &Params) -> Vec<Product> {
    let start = (params.page.unwrap_or(1) - 1) * params.limit.unwrap_or(10);
    let end = start + params.limit.unwrap_or(100);
    let paginated_products = products
        .into_iter()
        .skip(start as usize)
        .take((end - start) as usize)
        .collect();
    paginated_products
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/products", get(get_products).layer(
            CorsLayer::new().allow_origin(Any)
        ));

    let port = std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
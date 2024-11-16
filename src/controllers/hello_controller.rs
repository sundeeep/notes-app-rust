use axum::extract::{Query, Path};
use axum::response::{Html, IntoResponse};

use crate::models::hello_model::HelloParams;

pub async fn handler_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:?} - handler_query", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("<p>Hello, <strong>{name}</strong></p>"))
}

pub async fn handler_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:?} - handler_path {name:?}", "HANDLER");
    Html(format!("<p>Hello, <strong>{name}</strong></p>"))
}
use axum::Router;
use axum::routing::get;

use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use std::error::Error;
// use tracing::debug;

use dotenv::dotenv;
use std::env;

mod models;
mod routes;
mod controllers;
use controllers::hello_controller::{handler_query, handler_path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "link_shortener=debug".into())
        )
        .with(
            tracing_subscriber::fmt::layer()
        ).init();

    let app = Router::new()
        .route("/hello", get(handler_query))
        .route("/hello/:name", get(handler_path));
    
    let port: String = env::var("PORT").unwrap_or_else(|_| String::from("8002"));

    let tcp_listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Could not initialize TcpListener.");
    
        println!(
        "->> Server is Listening on {:?}",
        tcp_listener.local_addr().expect("Couldn't convert tcp_listener address to local address.")
    );
    
    axum::serve(tcp_listener, app)
        .await
        .expect("Axum Server is not running!");

    Ok(())
}

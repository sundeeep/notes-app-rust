use axum::Router;
use axum::routing::get;

use tokio::net::TcpListener;

use std::error::Error;

mod models;
mod routes;
mod controllers;
use controllers::hello_controller::{handler_query, handler_path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    let routes_hello = Router::new()
        .route("/hello", get(handler_query))
        .route("/hello/:name", get(handler_path));

    let tcp_listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not initialize TcpListener.");

    let local_address = tcp_listener.local_addr().expect("Couldn't convert tcp_listener address to local address.");
    
    println!("->> LISTENING ON {:?}\n", local_address);
    
    axum::serve(tcp_listener, routes_hello)
    .await
    .expect("Axum Server is not running!");

    Ok(())
}

use axum::response::Html;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;



#[tokio::main]
async fn main(){
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING ON {:?}\n", listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service())
    .await
    .unwrap();
}

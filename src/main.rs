use axum::{routing::post,Router};



use restp::receive;

// basic goal of program, be able to receive any json and print it to the 
//terminal
#[tokio::main]
async fn main() {
    let route = Router::new()
    .route("/", post(receive));

    let addr = "0.0.0.0:8080";

    let listender = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server is up and running on port {}", addr);
    axum::serve(listender, route).await.unwrap();

    println!("Hello, world!");
}

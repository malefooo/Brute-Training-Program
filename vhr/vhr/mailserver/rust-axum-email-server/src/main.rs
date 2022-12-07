use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

//make "async" keyword take effect 
#[tokio::main]
async fn main() {
    //crete a application with a route
    let app = Router::new().route("/",get(handler));

    //run this application
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("starting on http://{}", addr);
    //Axum example
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

//here can handld http request and resonse
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
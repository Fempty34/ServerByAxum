use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::env;
use dotenv::dotenv;
async fn hello_world() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello_world));
    dotenv().expect(".env file not found");
    let server_details = env::var("SERVER").unwrap();
    println!("Received server details: {}", server_details);

    let server_addr: SocketAddr = server_details
        .parse()
        .expect("Unable to parse socket address");

    let addr = SocketAddr::from(server_addr);
    let tcp = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp, router).await.unwrap();
}
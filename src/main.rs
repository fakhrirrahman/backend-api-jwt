use std::net::SocketAddr;
use axum::{Extension, Router};
use dotenvy::dotenv;



#[tokio::main]
async fn main(){
    dotenv().ok();

    let app = Router::new()
        .layer(Extension(()));

    let port = std::env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3001);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("server berjalan di {}", addr);

    //jalankan server
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(),
        app).await.unwrap();

}
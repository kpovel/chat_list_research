use axum::{routing::get, Router};
use libsql::Connection;
use std::{error::Error, sync::Arc};
use tokio::net::TcpListener;

mod db;

struct Conf {
    db_client: Connection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_client = db::db_client()?;

    let conf = Arc::new(Conf { db_client });

    let app = Router::new().route("/", get(index));

    let listener = TcpListener::bind("localhost:42069").await?;
    println!("Listenint at localhost:42069");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> &'static str {
    return "Hello world";
}

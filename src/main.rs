use axum::{body::Body, response::Response, routing::get, Router};
use libsql::Connection;
use std::{error::Error, sync::Arc};
use tokio::net::TcpListener;

mod db;
mod templates;

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

async fn index() -> Response {
    Response::builder()
        .header("Content-Type", "text/html")
        .body(Body::from(templates::render_template("index.html")))
        .unwrap()
}

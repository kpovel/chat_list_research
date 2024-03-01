use axum::Router;
use libsql::Connection;
use std::{error::Error, sync::Arc};
use tokio::net::TcpListener;

mod db;
mod templates;

struct AppState {
    db_client: Connection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_client = db::db_client()?;
    let shared_state = Arc::new(AppState { db_client });

    let app = Router::new().nest("/", templates::render_templates(Arc::clone(&shared_state)));

    let listener = TcpListener::bind("localhost:42069").await?;
    println!("Listenint at localhost:42069");
    axum::serve(listener, app).await?;

    Ok(())
}

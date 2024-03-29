use super::render_template;
use crate::{db, AppState};
use axum::{body::Body, extract::State, response::Response};
use std::sync::Arc;

pub async fn index(State(state): State<Arc<AppState>>) -> Response {
    let chat_list = db::chat_preview(Arc::clone(&state)).await;

    let mut context = tera::Context::new();
    context.insert("chat_list", &chat_list);

    Response::builder()
        .header("Content-Type", "text/html")
        .body(Body::from(render_template("index.html", context)))
        .unwrap()
}

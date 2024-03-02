use super::render_template;
use crate::{db, AppState};
use axum::{
    body::Body,
    extract::{Path, State},
    response::Response,
};
use std::sync::Arc;

pub async fn chat(State(state): State<Arc<AppState>>, Path(chat_id): Path<String>) -> Response {
    db::chat::chat_messages(Arc::clone(&state), &chat_id).await;

    let chat_list = db::chat_preview(Arc::clone(&state)).await;

    let mut context = tera::Context::new();
    context.insert("chat_list", &chat_list);

    Response::builder()
        .header("Content-Type", "text/html")
        .body(Body::from(render_template("chat.html", context)))
        .unwrap()
}

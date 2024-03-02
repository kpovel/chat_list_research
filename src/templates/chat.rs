use super::render_template;
use crate::{
    db::{self, chat::ChatMessages},
    AppState,
};
use axum::{
    body::Body,
    extract::{Path, State},
    response::Response,
};
use std::sync::Arc;

pub async fn chat(State(state): State<Arc<AppState>>, Path(chat_id): Path<String>) -> Response {
    let mut context = tera::Context::new();
    let chat_list = db::chat_preview(Arc::clone(&state)).await;
    context.insert("chat_list", &chat_list);

    let chat_messages = db::chat::chat_messages(Arc::clone(&state), &chat_id).await;
    let template_to_render = match chat_messages {
        ChatMessages::ChatNotExists => "chat/not-exists.html",
        ChatMessages::EmptyChat => "chat/empty-chat.html",
        ChatMessages::Messages(m) => {
            context.insert("messages", &m);
            "chat/messages.html"
        }
    };

    Response::builder()
        .header("Content-Type", "text/html")
        .body(Body::from(render_template(template_to_render, context)))
        .unwrap()
}

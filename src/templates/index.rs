use super::render_template;
use crate::AppState;
use axum::{body::Body, extract::State, response::Response};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
struct Chat {
    chat_id: u32,
    chat_name: String,
    sent_at: Option<String>,
    last_message: Option<String>,
}

pub async fn index(State(state): State<Arc<AppState>>) -> Response {
    let chat_list = chat_list(Arc::clone(&state)).await;

    let mut context = tera::Context::new();
    context.insert("chat_list", &chat_list);
    // todo: insert values

    Response::builder()
        .header("Content-Type", "text/html")
        .body(Body::from(render_template("index.html", context)))
        .unwrap()
}

async fn chat_list(state: Arc<AppState>) -> Vec<Chat> {
    let mut rows = state
        .db_client
        .query(
            "\
with last_chat_message as (select chat_id, message, max(sent_at) as sent_at
                           from message
                           group by chat_id)

select chat.id     as chat_id,
       chat.name   as chat_name,
       sent_at,
       lcm.message as last_message
from chat
         left join last_chat_message lcm on chat.id = lcm.chat_id;
",
            (),
        )
        .await
        .unwrap();

    let mut chat_list = vec![];
    while let Ok(Some(row)) = rows.next() {
        chat_list.push(Chat {
            chat_id: row.get(0).unwrap(),
            chat_name: row.get(1).unwrap(),
            sent_at: row.get(2).unwrap(),
            last_message: row.get(3).unwrap(),
        })
    }

    chat_list
}

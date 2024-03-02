use serde::Serialize;

use crate::AppState;
use std::sync::Arc;

pub enum ChatMessages {
    ChatNotExists,
    EmptyChat,
    Messages(Vec<Message>),
}

#[derive(Debug, Serialize)]
pub struct Message {
    id: u32,
    message: String,
    sent_at: String,
}

pub async fn chat_messages(state: Arc<AppState>, chat_id: &str) -> ChatMessages {
    if !is_chat_exists(Arc::clone(&state), chat_id).await {
        return ChatMessages::ChatNotExists;
    }

    let messages = get_messages(Arc::clone(&state), chat_id).await;
    if messages.is_empty() {
        ChatMessages::EmptyChat
    } else {
        ChatMessages::Messages(messages)
    }
}

async fn get_messages(state: Arc<AppState>, chat_id: &str) -> Vec<Message> {
    let mut rows = state
        .db_client
        .query(
            "\
select id as message_id, message, sent_at
from message
where chat_id = ?1;",
            [chat_id],
        )
        .await
        .unwrap();

    let mut messages = vec![];
    while let Ok(Some(message)) = rows.next() {
        messages.push(Message {
            id: message.get(0).unwrap(),
            message: message.get(1).unwrap(),
            sent_at: message.get(2).unwrap(),
        })
    }

    messages
}

async fn is_chat_exists(state: Arc<AppState>, chat_id: &str) -> bool {
    let mut rows = state
        .db_client
        .query("select count(*) from chat where id = ?1;", [chat_id])
        .await
        .unwrap();

    let chat_count: u32 = rows.next().unwrap().unwrap().get(0).unwrap();

    if chat_count > 0 {
        true
    } else {
        false
    }
}

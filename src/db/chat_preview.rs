use std::sync::Arc;
use serde::Serialize;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct ChatPreview {
    chat_id: u32,
    chat_name: String,
    sent_at: Option<String>,
    last_message: Option<String>,
}


pub async fn chat_preview(state: Arc<AppState>) -> Vec<ChatPreview> {
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
        chat_list.push(ChatPreview {
            chat_id: row.get(0).unwrap(),
            chat_name: row.get(1).unwrap(),
            sent_at: row.get(2).unwrap(),
            last_message: row.get(3).unwrap(),
        })
    }

    chat_list
}

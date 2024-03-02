use libsql::{Connection, Database};
use std::fmt::Error;

pub mod chat;
mod chat_preview;

pub use chat_preview::*;

pub fn db_client() -> Result<Connection, Error> {
    let db_url = "file:/tmp/chat-list-research.db";

    let db = Database::open(db_url).unwrap();
    let conn = db.connect().unwrap();

    Ok(conn)
}

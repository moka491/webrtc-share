use serde::{Deserialize, Serialize};

pub use hyper_tungstenite::tungstenite::Message as RawMessage;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: u16,
    pub event: String,
    /* Deserialize later in handler dependent on msg_type */
    pub params: serde_json::Value,
}

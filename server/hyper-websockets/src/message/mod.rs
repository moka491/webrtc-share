use serde::{Deserialize, Serialize};

pub use hyper_tungstenite::tungstenite::Message as RawMessage;

pub mod parse;

#[derive(Serialize, Deserialize)]
pub struct Message<T> {
    pub id: u16,
    pub event: String,
    pub params: T,
}

// Message where the params are left serialized until it's clear what to deserialize them into
pub type PartialMessage = Message<serde_json::Value>;

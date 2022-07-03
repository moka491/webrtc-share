use serde::{Deserialize, Serialize};

// Calling this "RawMessage" in the rest of the crate to differentiate
// from the Message type that this crate consumes
pub use hyper_tungstenite::tungstenite::Message as RawMessage;

pub mod parse;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message<T> {
    pub id: u32,
    pub event: String,
    pub params: T,
}

// Message where the params are left serialized until it's clear what to deserialize them into
pub type PartialMessage = Message<serde_json::Value>;

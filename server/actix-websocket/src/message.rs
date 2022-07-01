use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: u16,
    pub msg_type: String,
    /* Deserialize later in handler dependent on msg_type */
    pub params: Box<RawValue>,
}

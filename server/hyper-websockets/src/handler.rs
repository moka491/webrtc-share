use crate::{context::Context, message::Message};

pub type MessageHandlerFn = fn(Context);

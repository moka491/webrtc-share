use crate::handler::context::Context;
pub mod context;

pub type MessageHandlerFn = fn(Context);

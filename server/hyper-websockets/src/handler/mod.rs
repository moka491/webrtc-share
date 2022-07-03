use std::{collections::HashMap, pin::Pin};

use futures::Future;

use crate::{
    error::{Error, WsResult},
    handler::context::Context,
};
pub mod context;

pub type MessageHandlerFn =
    Box<(dyn Fn(Context) -> Pin<Box<(dyn Future<Output = WsResult<()>> + Send)>> + Send + Sync)>;

pub type HandlerMap = HashMap<String, MessageHandlerFn>;

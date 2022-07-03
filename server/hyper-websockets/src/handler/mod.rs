use std::{collections::HashMap, pin::Pin};

use futures::Future;

use crate::{
    error::{Error, WsResult},
    handler::context::Context,
};
pub mod context;

pub type MessageHandlerFn = fn(Context) -> WsResult<()>;

pub type HandlerMap = HashMap<String, MessageHandlerFn>;

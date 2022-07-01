pub struct ConnHandlerBuilder {
    handlers: HashMap<String, MessageHandlerFn>,
}

impl ConnHandlerBuilder {
    fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler(self, msg_name: &str, handler_fn: MessageHandlerFn) -> Self {
        self
    }

    pub fn build(self) -> ConnHandler {
        ConnHandler {
            handlers: self.handlers,
        }
    }
}

pub struct ConnHandler {
    pub handlers: HashMap<String, MessageHandlerFn>,
}

impl ConnHandler {
    pub fn builder() -> ConnHandlerBuilder {
        ConnHandlerBuilder::new()
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("websocket messaging error")]
    TungsteniteError(#[from] hyper_tungstenite::tungstenite::Error),

    #[error("websocket protocol error")]
    TungsteniteProtocolError(#[from] hyper_tungstenite::tungstenite::error::ProtocolError),

    #[error("http service error")]
    HyperError(#[from] hyper::Error),

    #[error("message de/serialization error")]
    SerdeError(#[from] serde_json::Error),
}

pub type WsResult<T> = std::result::Result<T, Error>;

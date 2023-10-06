use thiserror::Error;
use twilight_validate::message;

#[derive(Error, Debug)]
pub enum DuskError {
    #[error("Invalid component: {0}")]
    InvalidComponentError(String),
    #[error("Interaction error: {0}")]
    InteractionError(#[from] twilight_http::Error),
    #[error("Message validation error: {0}")]
    MessageValidationError(#[from] message::MessageValidationError),
    #[error("Deserialize body error: {0}")]
    DeserializeBodyError(#[from] twilight_http::response::DeserializeBodyError),
    #[error("RecvError: {0}")]
    RecvError(#[from] tokio::sync::oneshot::error::RecvError),
}

pub type Result<T> = std::result::Result<T, DuskError>;

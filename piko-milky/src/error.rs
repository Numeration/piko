use thiserror::Error;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] sonic_rs::Error),

    #[error(transparent)]
    Websocket(#[from] reqwest_websocket::Error),
    
    #[error(transparent)]
    Event(#[from] EventError),
    
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("{0}")]
    Io(anyhow::Error),

    #[error("{0}")]
    Json(anyhow::Error),
}

pub fn throw_event_io_error<E: Into<anyhow::Error>, T: From<EventError>>(err: E) -> T {
    T::from(EventError::Io(err.into()))
}

pub fn throw_event_json_error<E: Into<anyhow::Error>, T: From<EventError>>(err: E) -> T {
    T::from(EventError::Json(err.into()))
}
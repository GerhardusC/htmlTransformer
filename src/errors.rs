use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Unable to parse HTML")]
    ParseError,
    #[error("Failed to manipulate string correctly")]
    StringManipulationError,
    #[error("Server init error")]
    ServerInitError(#[from] std::io::Error),
}

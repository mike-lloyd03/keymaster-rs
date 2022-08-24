use thiserror::Error as ThisError;

#[derive(ThisError, Debug, PartialEq, Clone)]
pub enum Error {
    #[error("invalid user input")]
    BadRequest(String), // 400
    #[error("unauthorized")]
    Unauthorized, // 401
    #[error("not found")]
    NotFound(String), // 404
    #[error("internal server error")]
    InternalServerError(String), // 500
    #[error("error serializing request")]
    SerializationError,
    #[error("error deserializing response")]
    DeserializationError,
    #[error("error sending request")]
    RequestError(String),
}

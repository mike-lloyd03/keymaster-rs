use thiserror::Error as ThisError;

#[derive(ThisError, Debug, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("Invalid user input")]
    BadRequest(String), // 400
    #[error("You are not authorized to access this resource")]
    Unauthorized, // 401
    #[error("Resource not found")]
    NotFound(String), // 404
    #[error("Internal server error")]
    InternalServerError(String), // 500
    #[error("Error serializing request")]
    SerializationError,
    #[error("Error deserializing response")]
    DeserializationError,
    #[error("Error sending request")]
    RequestError(String),
}

use std::fmt;

#[derive(Debug)]
pub(crate) enum CommandError {
    ReqwestError(reqwest::Error),
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    SerdeJsonError(serde_json::Error),
    SqlxError(sqlx::Error)
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::ReqwestError(..) =>
                write!(f, "Request error"),
            CommandError::SerdeJsonError(..) =>
                write!(f, "Error while deserializing Json object"),
            CommandError::SqlxError(..) =>
                write!(f, "Database Error"),
        }
    }
}

impl From<reqwest::Error> for CommandError {
    fn from(err: reqwest::Error) -> CommandError {
        CommandError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for CommandError {
    fn from(err: serde_json::Error) -> CommandError {
        CommandError::SerdeJsonError(err)
    }
}

impl From<sqlx::Error> for CommandError {
    fn from(err: sqlx::Error) -> CommandError {
        CommandError::SqlxError(err)
    }
}
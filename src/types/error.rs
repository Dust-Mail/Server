use std::{error::Error as StdError, fmt};

use dust_mail::types::Error as SdkError;
use rocket::serde::{ser::SerializeStruct, Serialize};

#[derive(Debug)]
pub enum ErrorKind {
    SdkError(SdkError),
    CreateHttpRequest,
    BadConfig,
    Unauthorized,
    BadRequest,
    TooManyRequests,
    NotFound,
    Parse,
    InternalError,
}

#[derive(Debug)]
pub struct Error {
    message: String,
    kind: ErrorKind,
}

impl From<SdkError> for Error {
    fn from(error: SdkError) -> Self {
        Self {
            kind: ErrorKind::SdkError(error),
            message: String::from("Error with upstream mail server"),
        }
    }
}

impl Error {
    pub fn new<S: Into<String>>(kind: ErrorKind, message: S) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.kind() {
            ErrorKind::SdkError(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        let source = self.source().unwrap_or(&self);
        let mut state = serializer.serialize_struct("Error", 2)?;

        state.serialize_field("message", &source.to_string())?;
        state.serialize_field("kind", "MailError")?;
        state.end()
    }
}

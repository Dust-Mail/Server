use std::{error::Error as StdError, fmt};

use reqwest::Error as OutgoingRequestError;

use dust_mail::types::Error as SdkError;
use rocket::serde::{
    json::serde_json::Error as SerializeJsonError, ser::SerializeStruct, Serialize,
};

#[derive(Debug)]
pub enum ErrorKind {
    SdkError(SdkError),
    OutgoingHttpRequest(OutgoingRequestError),
    SerializeJson(SerializeJsonError),
    Oauth2,
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
        Self::new(
            ErrorKind::SdkError(error),
            "Error with upstream mail server",
        )
    }
}

impl From<OutgoingRequestError> for Error {
    fn from(outgoing_request_error: OutgoingRequestError) -> Self {
        Self::new(
            ErrorKind::OutgoingHttpRequest(outgoing_request_error),
            "Outgoing HTTP request failed",
        )
    }
}

impl From<SerializeJsonError> for Error {
    fn from(serialize_json_error: SerializeJsonError) -> Self {
        Self::new(
            ErrorKind::SerializeJson(serialize_json_error),
            "Failed to serialize json data",
        )
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
            ErrorKind::OutgoingHttpRequest(e) => Some(e),
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

/* Copyright © 2025, CosmicMind, Inc. */

use std::error::Error;
use std::fmt::{self, Display};

/// **Errors that can occur when working with entity IDs**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityIdError {
    /// The ID string doesn't have the expected format (prefix + delimiter + identifier)
    InvalidFormat,
    /// The identifier part of the ID string is not a valid identifier (UUID or ULID)
    InvalidIdentifier,
}

impl Display for EntityIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntityIdError::InvalidFormat => f.write_str("The provided ID has an invalid format"),
            EntityIdError::InvalidIdentifier => f.write_str("ID must contain a valid identifier"),
        }
    }
}

impl Error for EntityIdError {}

impl AsRef<str> for EntityIdError {
    fn as_ref(&self) -> &str {
        match self {
            EntityIdError::InvalidFormat => "The provided ID has an invalid format",
            EntityIdError::InvalidIdentifier => "ID must contain a valid identifier",
        }
    }
}

impl From<EntityIdError> for String {
    fn from(err: EntityIdError) -> Self {
        err.as_ref().to_string()
    }
}

impl From<&EntityIdError> for String {
    fn from(err: &EntityIdError) -> Self {
        err.as_ref().to_string()
    }
}

/// **Wrapper for identifier-specific errors**
#[derive(Debug)]
pub enum IdentifierError {
    /// Error from UUID operations
    Uuid(uuid::Error),
    /// Error from ULID operations
    Ulid(ulid::DecodeError),
}

impl Display for IdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IdentifierError::Uuid(err) => write!(f, "Invalid UUID format: {}", err),
            IdentifierError::Ulid(err) => write!(f, "Invalid ULID format: {}", err),
        }
    }
}

impl Error for IdentifierError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            IdentifierError::Uuid(err) => Some(err),
            IdentifierError::Ulid(err) => Some(err),
        }
    }
}

impl From<uuid::Error> for IdentifierError {
    fn from(err: uuid::Error) -> Self {
        IdentifierError::Uuid(err)
    }
}

impl From<ulid::DecodeError> for IdentifierError {
    fn from(err: ulid::DecodeError) -> Self {
        IdentifierError::Ulid(err)
    }
}

impl From<IdentifierError> for String {
    fn from(err: IdentifierError) -> Self {
        match &err {
            IdentifierError::Uuid(uuid_err) => format!("Invalid UUID format: {}", uuid_err),
            IdentifierError::Ulid(ulid_err) => format!("Invalid ULID format: {}", ulid_err),
        }
    }
}

impl From<&IdentifierError> for String {
    fn from(err: &IdentifierError) -> Self {
        match err {
            IdentifierError::Uuid(uuid_err) => format!("Invalid UUID format: {}", uuid_err),
            IdentifierError::Ulid(ulid_err) => format!("Invalid ULID format: {}", ulid_err),
        }
    }
}

impl IdentifierError {
    /// Get the underlying UUID error, if this is a UUID error
    pub fn uuid_error(&self) -> Option<&uuid::Error> {
        match self {
            IdentifierError::Uuid(err) => Some(err),
            _ => None,
        }
    }

    /// Get the underlying ULID error, if this is a ULID error
    pub fn ulid_error(&self) -> Option<&ulid::DecodeError> {
        match self {
            IdentifierError::Ulid(err) => Some(err),
            _ => None,
        }
    }

    /// Get the error message from the underlying error
    pub fn error_message(&self) -> String {
        match self {
            IdentifierError::Uuid(err) => err.to_string(),
            IdentifierError::Ulid(err) => err.to_string(),
        }
    }
}

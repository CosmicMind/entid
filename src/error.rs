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
            IdentifierError::Uuid(err) => write!(f, "UUID error: {}", err),
            IdentifierError::Ulid(err) => write!(f, "ULID error: {}", err),
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

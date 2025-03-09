/* Copyright © 2025, CosmicMind, Inc. */

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::hash::Hash;
use std::mem::transmute;
use std::str::FromStr;
use std::time::{Duration, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use ulid::Ulid;
use uuid::{Uuid, Version};

use crate::error::IdentifierError;

/// **Trait for identifier types**
///
/// This trait defines the common interface for different identifier types (UUID, ULID).
/// It allows the `EntityId` type to be generic over the identifier implementation.
pub trait Identifier:
    Sized + Clone + PartialEq + Eq + Hash + Display + Serialize + for<'de> Deserialize<'de>
{
    /// Parse a string into an identifier
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, IdentifierError>;

    /// Generate a new random identifier
    fn generate() -> Self;

    /// Convert the identifier to a string representation
    fn as_str(&self) -> &str;

    /// Get the timestamp in milliseconds (if applicable)
    fn timestamp_ms(&self) -> Option<u64>;
}

/// **UUID-based identifier implementation**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UuidIdentifier(Uuid);

// Thread-local cache for string representations
thread_local! {
    static UUID_CACHE: RefCell<HashMap<Uuid, String>> =
        RefCell::new(HashMap::new());
}

impl UuidIdentifier {
    /// Create a new UUID v4 (random)
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a new UUID v5 (name-based with SHA-1 hash)
    pub fn new_v5(namespace: &Uuid, name: &str) -> Self {
        Self(Uuid::new_v5(namespace, name.as_bytes()))
    }

    /// Get the underlying UUID
    pub fn uuid(&self) -> Uuid {
        self.0
    }

    /// Get the UUID version
    pub fn version(&self) -> Option<Version> {
        self.0.get_version()
    }
}

impl Identifier for UuidIdentifier {
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, IdentifierError> {
        Ok(Self(
            Uuid::parse_str(s.as_ref()).map_err(IdentifierError::from)?,
        ))
    }

    fn generate() -> Self {
        Self::new_v4()
    }

    fn as_str(&self) -> &str {
        UUID_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            cache.entry(self.0).or_insert_with(|| self.0.to_string());
            // This is safe because we know the string exists in the cache
            // and the cache lives for the duration of the thread
            unsafe { transmute(cache.get(&self.0).unwrap().as_str()) }
        })
    }

    fn timestamp_ms(&self) -> Option<u64> {
        // UUIDs don't have a timestamp component by default
        // UUID v1 has a timestamp, but we'd need to extract it
        None
    }
}

impl Display for UuidIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Uuid> for UuidIdentifier {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl FromStr for UuidIdentifier {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// **ULID-based identifier implementation**
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UlidIdentifier(Ulid);

// Thread-local cache for string representations
thread_local! {
    static ULID_CACHE: RefCell<HashMap<Ulid, String>> =
        RefCell::new(HashMap::new());
}

impl Default for UlidIdentifier {
    fn default() -> Self {
        Self::new()
    }
}

impl UlidIdentifier {
    /// Create a new ULID
    pub fn new() -> Self {
        Self(Ulid::new())
    }

    /// Create a ULID with a specific timestamp
    pub fn with_timestamp(timestamp_ms: u64) -> Self {
        // Use 0 for the random component
        Self(Ulid::from_datetime(
            UNIX_EPOCH + Duration::from_millis(timestamp_ms),
        ))
    }

    /// Get the underlying ULID
    pub fn ulid(&self) -> Ulid {
        self.0
    }

    /// Get the timestamp in milliseconds
    pub fn get_timestamp_ms(&self) -> u64 {
        // Convert the datetime to milliseconds since UNIX epoch
        let datetime = self.0.datetime();
        let since_epoch = datetime
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));
        since_epoch.as_millis() as u64
    }

    /// Create a monotonic ULID based on a previous one
    pub fn monotonic_from(previous: Option<&Self>) -> Self {
        match previous {
            Some(prev) => {
                let new_ulid = Ulid::new();
                let prev_ms = prev.get_timestamp_ms();
                let new_ms = {
                    let datetime = new_ulid.datetime();
                    let since_epoch = datetime
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or(Duration::from_secs(0));
                    since_epoch.as_millis() as u64
                };

                if new_ms <= prev_ms {
                    // Create a new ULID with the previous timestamp + 1ms and random bits
                    Self(Ulid::from_datetime(
                        UNIX_EPOCH + Duration::from_millis(prev_ms + 1),
                    ))
                } else {
                    Self(new_ulid)
                }
            }
            None => Self::new(),
        }
    }
}

impl Identifier for UlidIdentifier {
    fn parse<S: AsRef<str>>(s: S) -> Result<Self, IdentifierError> {
        Ok(Self(
            Ulid::from_string(s.as_ref()).map_err(IdentifierError::from)?,
        ))
    }

    fn generate() -> Self {
        Self::new()
    }

    fn as_str(&self) -> &str {
        ULID_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            cache.entry(self.0).or_insert_with(|| self.0.to_string());
            // This is safe because we know the string exists in the cache
            // and the cache lives for the duration of the thread
            unsafe { transmute(cache.get(&self.0).unwrap().as_str()) }
        })
    }

    fn timestamp_ms(&self) -> Option<u64> {
        Some(self.get_timestamp_ms())
    }
}

impl Display for UlidIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Ulid> for UlidIdentifier {
    fn from(ulid: Ulid) -> Self {
        Self(ulid)
    }
}

impl FromStr for UlidIdentifier {
    type Err = IdentifierError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_identifier() {
        let id = UuidIdentifier::generate();
        let id_str = id.as_str();
        let parsed = UuidIdentifier::parse(id_str).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_ulid_identifier() {
        let id = UlidIdentifier::generate();
        let id_str = id.as_str();
        let parsed = UlidIdentifier::parse(id_str).unwrap();
        assert_eq!(id, parsed);

        // Test timestamp
        assert!(id.timestamp_ms().is_some());
    }
}

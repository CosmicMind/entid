/* Copyright © 2025, CosmicMind, Inc. */

use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::OnceLock;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::EntityIdError;
use crate::identifier::{Identifier, UlidIdentifier, UuidIdentifier};

/// **Trait for entity-specific prefixes**
///
/// - Defines a unique **prefix** for each entity type.
/// - Allows **custom delimiter** (default: `_`).
pub trait Prefix {
    fn prefix() -> &'static str;
    fn delimiter() -> &'static str {
        "_" // Default delimiter
    }
}

/// **Generic ID type supporting both UUID and ULID**
///
/// - Supports **UUID** (all versions) and **ULID**.
/// - Provides **thread-safe caching**.
/// - Allows **custom delimiters**.
/// - Type-safe through generic parameters.
#[derive(Debug)]
pub struct EntityId<T: Prefix, I: Identifier> {
    id: I,
    cached_str: OnceLock<String>, // Thread-safe cache
    _marker: PhantomData<T>,
}

impl<T: Prefix, I: Identifier> EntityId<T, I> {
    /// **Create an `EntityId` from an identifier string**
    pub fn new(s: &str) -> Result<Self, EntityIdError> {
        let expected_prefix = format!("{}{}", T::prefix(), T::delimiter());

        if !s.starts_with(&expected_prefix) {
            return Err(EntityIdError::InvalidFormat);
        }

        let id_part = &s[expected_prefix.len()..];
        let id = I::parse(id_part).map_err(|_| EntityIdError::InvalidIdentifier)?;

        Ok(Self {
            id,
            cached_str: OnceLock::new(),
            _marker: PhantomData,
        })
    }

    /// **Generate a new `EntityId` with a random identifier**
    pub fn generate() -> Self {
        Self {
            id: I::generate(),
            cached_str: OnceLock::new(),
            _marker: PhantomData,
        }
    }

    /// **Create an `EntityId` from an existing identifier**
    pub fn from_identifier(id: I) -> Self {
        Self {
            id,
            cached_str: OnceLock::new(),
            _marker: PhantomData,
        }
    }

    /// **Get the full ID string with prefix (cached)**
    pub fn as_str(&self) -> &str {
        self.cached_str.get_or_init(|| {
            let mut s = String::with_capacity(T::prefix().len() + T::delimiter().len() + 36);
            s.push_str(T::prefix());
            s.push_str(T::delimiter());
            s.push_str(&self.id.as_string());
            s
        })
    }

    /// **Get the timestamp in milliseconds (if available)**
    pub fn timestamp_ms(&self) -> Option<u64> {
        self.id.timestamp_ms()
    }

    /// **Get the underlying identifier**
    pub fn identifier(&self) -> &I {
        &self.id
    }
}

/// **Implement `Hash` based on identifier**
///
/// This ensures `EntityId<T, I>` can be used in `HashSet` or `HashMap`.
impl<T: Prefix, I: Identifier> Hash for EntityId<T, I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// **Implement `PartialEq` and `Eq` (Comparison based on identifier)**
impl<T: Prefix, I: Identifier> PartialEq for EntityId<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Prefix, I: Identifier> Eq for EntityId<T, I> {}

/// **Implement `Clone` (Avoids duplicating cached value)**
impl<T: Prefix, I: Identifier> Clone for EntityId<T, I> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            cached_str: OnceLock::new(), // New cache for clone
            _marker: PhantomData,
        }
    }
}

/// **Implement `PartialOrd` and `Ord` for lexicographical sorting**
impl<T: Prefix, I: Identifier + Ord> PartialOrd for EntityId<T, I> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Prefix, I: Identifier + Ord> Ord for EntityId<T, I> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

/// **Serialization - Store as `<prefix><delimiter><id>`**
impl<T: Prefix, I: Identifier> Serialize for EntityId<T, I> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// **Deserialization - Reconstruct from stored ID string**
impl<'de, T: Prefix, I: Identifier> Deserialize<'de> for EntityId<T, I> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(&s).map_err(serde::de::Error::custom)
    }
}

/// **Implement `Display` for easy printing**
impl<T: Prefix, I: Identifier> Display for EntityId<T, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// **Implement `Deref` to the identifier**
impl<T: Prefix, I: Identifier> Deref for EntityId<T, I> {
    type Target = I;
    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

/// **Implement `Into<String>` to convert to string representation**
impl<T: Prefix, I: Identifier> From<EntityId<T, I>> for String {
    fn from(entity_id: EntityId<T, I>) -> Self {
        entity_id.as_str().to_string()
    }
}

/// **Implement `Into<String>` for references to convert to string representation**
impl<T: Prefix, I: Identifier> From<&EntityId<T, I>> for String {
    fn from(entity_id: &EntityId<T, I>) -> Self {
        entity_id.as_str().to_string()
    }
}

/// **Implement `From<I>` to create an EntityId from an identifier**
impl<T: Prefix, I: Identifier> From<I> for EntityId<T, I> {
    fn from(id: I) -> Self {
        Self::from_identifier(id)
    }
}

/// **Implement `Into<UuidIdentifier>` for UUID-based EntityId**
impl<T: Prefix> From<EntityId<T, UuidIdentifier>> for UuidIdentifier {
    fn from(entity_id: EntityId<T, UuidIdentifier>) -> Self {
        entity_id.id
    }
}

/// **Implement `Into<UuidIdentifier>` for references to UUID-based EntityId**
impl<T: Prefix> From<&EntityId<T, UuidIdentifier>> for UuidIdentifier {
    fn from(entity_id: &EntityId<T, UuidIdentifier>) -> Self {
        entity_id.id
    }
}

/// **Implement `Into<UlidIdentifier>` for ULID-based EntityId**
impl<T: Prefix> From<EntityId<T, UlidIdentifier>> for UlidIdentifier {
    fn from(entity_id: EntityId<T, UlidIdentifier>) -> Self {
        entity_id.id
    }
}

/// **Implement `Into<UlidIdentifier>` for references to ULID-based EntityId**
impl<T: Prefix> From<&EntityId<T, UlidIdentifier>> for UlidIdentifier {
    fn from(entity_id: &EntityId<T, UlidIdentifier>) -> Self {
        entity_id.id
    }
}

// Type aliases for common use cases
/// UUID-based entity ID
pub type UuidEntityId<T> = EntityId<T, UuidIdentifier>;

/// ULID-based entity ID
pub type UlidEntityId<T> = EntityId<T, UlidIdentifier>;

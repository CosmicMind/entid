/* Copyright © 2025, CosmicMind, Inc. */

use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::OnceLock;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ulid::Ulid;
use uuid::Uuid;

use crate::error::{EntityIdError, IdentifierError};
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
    ///
    /// Accepts any type that can be referenced as a string slice.
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, EntityIdError> {
        let s = s.as_ref();
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
    ///
    /// Returns the complete entity ID string in the format `<prefix><delimiter><identifier>`.
    /// For example: "user_123e4567-e89b-12d3-a456-426614174000"
    pub fn as_str(&self) -> &str {
        self.cached_str.get_or_init(|| {
            let mut s = String::with_capacity(T::prefix().len() + T::delimiter().len() + 36);
            s.push_str(T::prefix());
            s.push_str(T::delimiter());
            s.push_str(self.id.as_str());
            s
        })
    }

    /// **Get the raw identifier string without prefix**
    ///
    /// Returns just the identifier part of the ID without the prefix and delimiter.
    /// For example: "123e4567-e89b-12d3-a456-426614174000"
    pub fn id_str(&self) -> &str {
        self.id.as_str()
    }

    /// **Get the timestamp in milliseconds (if available)**
    pub fn timestamp_ms(&self) -> Option<u64> {
        self.id.timestamp_ms()
    }

    /// **Get the underlying identifier object**
    ///
    /// Returns a reference to the underlying identifier object (UuidIdentifier or UlidIdentifier).
    pub fn identifier(&self) -> &I {
        &self.id
    }

    /// **Get the prefix for this entity type**
    pub fn prefix() -> &'static str {
        T::prefix()
    }

    /// **Get the delimiter for this entity type**
    pub fn delimiter() -> &'static str {
        T::delimiter()
    }

    /// **Create a builder for this entity type**
    pub fn builder() -> EntityIdBuilder<T, I> {
        EntityIdBuilder::new()
    }

    /// **Implement `TryFrom<&str>` for the raw identifier string (without prefix)**
    ///
    /// This method parses a raw identifier string (UUID or ULID) without the prefix
    /// and creates an EntityId with the appropriate prefix.
    ///
    /// # Example
    ///
    /// ```
    /// use entid::{Prefix, UuidEntityId};
    ///
    /// #[derive(Prefix)]
    /// #[entid(prefix = "user")]
    /// struct User;
    ///
    /// // Parse a raw UUID string (without the "user_" prefix)
    /// let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
    /// let user_id = UuidEntityId::<User>::from_raw_str(uuid_str).unwrap();
    /// assert_eq!(user_id.as_str(), "user_123e4567-e89b-12d3-a456-426614174000");
    /// ```
    pub fn from_raw_str<S: AsRef<str>>(s: S) -> Result<Self, EntityIdError> {
        let id = I::parse(s).map_err(|_| EntityIdError::InvalidIdentifier)?;
        Ok(Self::from_identifier(id))
    }

    /// **Parse a raw identifier string (without prefix) into an EntityId, with custom error handling**
    ///
    /// This method is similar to `from_raw_str`, but allows mapping the error to a custom type.
    ///
    /// # Example
    ///
    /// ```
    /// use entid::{Prefix, UuidEntityId};
    ///
    /// #[derive(Prefix)]
    /// #[entid(prefix = "user")]
    /// struct User;
    ///
    /// // Parse a raw UUID string with custom error handling
    /// let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
    /// let user_id = UuidEntityId::<User>::parse_raw_str(uuid_str, |e| format!("Invalid UUID: {}", e)).unwrap();
    /// ```
    pub fn parse_raw_str<S, E, F>(s: S, error_mapper: F) -> Result<Self, E>
    where
        S: AsRef<str>,
        F: FnOnce(IdentifierError) -> E,
    {
        let id = I::parse(s).map_err(error_mapper)?;
        Ok(Self::from_identifier(id))
    }

    /// **Convert to the raw identifier string (without prefix)**
    ///
    /// Returns the identifier part of the ID as an owned String, without the prefix and delimiter.
    /// For example: "123e4567-e89b-12d3-a456-426614174000"
    ///
    /// # Example
    ///
    /// ```
    /// use entid::{Prefix, UuidEntityId};
    ///
    /// #[derive(Prefix)]
    /// #[entid(prefix = "user")]
    /// struct User;
    ///
    /// let user_id = UuidEntityId::<User>::generate();
    /// let raw_string = user_id.to_raw_string();
    /// assert_eq!(raw_string, user_id.id_str().to_string());
    /// ```
    pub fn to_raw_string(&self) -> String {
        self.id_str().to_string()
    }

    /// **Convert to the underlying identifier type**
    ///
    /// Returns a clone of the underlying identifier.
    ///
    /// # Example
    ///
    /// ```
    /// use entid::{Prefix, UuidEntityId, UuidIdentifier};
    ///
    /// #[derive(Prefix)]
    /// #[entid(prefix = "user")]
    /// struct User;
    ///
    /// let user_id = UuidEntityId::<User>::generate();
    /// let uuid_identifier: UuidIdentifier = user_id.to_identifier();
    /// assert_eq!(uuid_identifier, *user_id.identifier());
    /// ```
    pub fn to_identifier(&self) -> I {
        self.id.clone()
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

/// **Implement `TryFrom<&str>` for converting from string slices**
impl<T: Prefix, I: Identifier> TryFrom<&str> for EntityId<T, I> {
    type Error = EntityIdError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

/// **Implement `TryFrom<String>` for converting from owned strings**
impl<T: Prefix, I: Identifier> TryFrom<String> for EntityId<T, I> {
    type Error = EntityIdError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

/// **Implement `TryFrom<&String>` for converting from string references**
impl<T: Prefix, I: Identifier> TryFrom<&String> for EntityId<T, I> {
    type Error = EntityIdError;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

/// **Implement `FromStr` for parsing from strings**
impl<T: Prefix, I: Identifier> FromStr for EntityId<T, I> {
    type Err = EntityIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

// Type aliases for common use cases
/// UUID-based entity ID
pub type UuidEntityId<T> = EntityId<T, UuidIdentifier>;

/// ULID-based entity ID
pub type UlidEntityId<T> = EntityId<T, UlidIdentifier>;

impl<T: Prefix> EntityId<T, UuidIdentifier> {
    /// **Create a new UUID-based entity ID with a specific UUID**
    pub fn with_uuid(uuid: Uuid) -> Self {
        Self::from_identifier(UuidIdentifier::from(uuid))
    }

    /// **Create a new UUID v4 (random) entity ID**
    pub fn new_v4() -> Self {
        Self::from_identifier(UuidIdentifier::new_v4())
    }

    /// **Create a new UUID v5 (name-based) entity ID**
    pub fn new_v5(namespace: &Uuid, name: &str) -> Self {
        Self::from_identifier(UuidIdentifier::new_v5(namespace, name))
    }
}

impl<T: Prefix> EntityId<T, UlidIdentifier> {
    /// **Create a new ULID-based entity ID with a specific ULID**
    pub fn with_ulid(ulid: Ulid) -> Self {
        Self::from_identifier(UlidIdentifier::from(ulid))
    }

    /// **Create a new ULID entity ID with a specific timestamp**
    pub fn with_timestamp(timestamp_ms: u64) -> Self {
        Self::from_identifier(UlidIdentifier::with_timestamp(timestamp_ms))
    }

    /// **Create a monotonic ULID entity ID based on a previous one**
    pub fn monotonic_from(previous: Option<&Self>) -> Self {
        let prev_id = previous.map(|p| p.identifier());
        Self::from_identifier(UlidIdentifier::monotonic_from(prev_id))
    }
}

/// **Builder for creating EntityIds**
pub struct EntityIdBuilder<T: Prefix, I: Identifier> {
    id: Option<I>,
    _marker: PhantomData<T>,
}

impl<T: Prefix, I: Identifier> Default for EntityIdBuilder<T, I> {
    fn default() -> Self {
        Self {
            id: None,
            _marker: PhantomData,
        }
    }
}

impl<T: Prefix, I: Identifier> EntityIdBuilder<T, I> {
    /// **Create a new builder**
    pub fn new() -> Self {
        Self::default()
    }

    /// **Set the identifier**
    pub fn with_identifier(mut self, id: I) -> Self {
        self.id = Some(id);
        self
    }

    /// **Build the EntityId**
    pub fn build(self) -> EntityId<T, I> {
        match self.id {
            Some(id) => EntityId::from_identifier(id),
            None => EntityId::generate(),
        }
    }
}

impl<T: Prefix> EntityIdBuilder<T, UuidIdentifier> {
    /// **Set a specific UUID**
    pub fn with_uuid(mut self, uuid: Uuid) -> Self {
        self.id = Some(UuidIdentifier::from(uuid));
        self
    }

    /// **Set a UUID v4 (random)**
    pub fn with_uuid_v4(mut self) -> Self {
        self.id = Some(UuidIdentifier::new_v4());
        self
    }

    /// **Set a UUID v5 (name-based)**
    pub fn with_uuid_v5(mut self, namespace: &Uuid, name: &str) -> Self {
        self.id = Some(UuidIdentifier::new_v5(namespace, name));
        self
    }
}

impl<T: Prefix> EntityIdBuilder<T, UlidIdentifier> {
    /// **Set a specific ULID**
    pub fn with_ulid(mut self, ulid: Ulid) -> Self {
        self.id = Some(UlidIdentifier::from(ulid));
        self
    }

    /// **Set a ULID with a specific timestamp**
    pub fn with_timestamp(mut self, timestamp_ms: u64) -> Self {
        self.id = Some(UlidIdentifier::with_timestamp(timestamp_ms));
        self
    }

    /// **Set a monotonic ULID based on a previous one**
    pub fn with_monotonic_from(mut self, previous: Option<&EntityId<T, UlidIdentifier>>) -> Self {
        let prev_id = previous.map(|p| p.identifier());
        self.id = Some(UlidIdentifier::monotonic_from(prev_id));
        self
    }
}

/// **Implement `Borrow<str>` to allow using EntityId in collections with string keys**
impl<T: Prefix, I: Identifier> Borrow<str> for EntityId<T, I> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

/// **Implement `AsRef<str>` for easy conversion to string slices**
impl<T: Prefix, I: Identifier> AsRef<str> for EntityId<T, I> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

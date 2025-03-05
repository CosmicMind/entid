/* Copyright © 2025, CosmicMind, Inc. */

//! entid provides an entity id generator and validator for Rust models.
//!
//! This library supports both UUIDs and ULIDs:
//!
//! ## UUID (Universally Unique Identifier)
//! - Industry standard for unique identifiers
//! - Multiple versions (v1, v3, v4, v5) for different use cases
//! - Widely supported across databases and systems
//!
//! ## ULID (Universally Unique Lexicographically Sortable Identifier)
//! - Lexicographically sortable (sorts by creation time)
//! - 128-bit compatibility with UUID
//! - No special characters (URL safe)
//! - Monotonicity option for time-ordered IDs
//! - Shorter string representation (26 characters vs 36 for UUID)
//!
//! Choose the identifier type that best suits your application's needs.

mod entity_id;
mod error;
mod identifier;

// Re-export key types for users.
pub use entity_id::{EntityId, Prefix, UlidEntityId, UuidEntityId};
pub use error::{EntityIdError, IdentifierError};
pub use identifier::{Identifier, UlidIdentifier, UuidIdentifier};

// Re-export UUID and ULID types for convenience
pub use ulid::Ulid;
pub use uuid::Uuid;

// Re-export derive macros when the "derive" feature is enabled
#[cfg(feature = "derive")]
pub use entid_derive::Prefix;

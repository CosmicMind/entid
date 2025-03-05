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
//!
//! ## Basic Usage
//!
//! ```rust
//! use entid::{Prefix, UuidEntityId};
//!
//! struct User;
//!
//! impl Prefix for User {
//!     fn prefix() -> &'static str {
//!         "user"
//!     }
//!
//!     fn delimiter() -> &'static str {
//!         "_"
//!     }
//! }
//!
//! // Generate a new ID with the prefix "user_"
//! let user_id = UuidEntityId::<User>::generate();
//! println!("User ID: {}", user_id); // e.g., "user_6ba7b810-9dad-11d1-80b4-00c04fd430c8"
//! ```
//!
//! ## Using the Derive Macro
//!
//! With the `derive` feature enabled, you can use the derive macro to implement the `Prefix` trait:
//!
//! ```rust
//! # #[cfg(feature = "derive")]
//! use entid::{Prefix, UuidEntityId};
//!
//! # #[cfg(feature = "derive")]
//! #[derive(Prefix)]
//! #[entid(prefix = "user", delimiter = "_")]
//! struct User;
//!
//! # #[cfg(feature = "derive")]
//! // The delimiter is optional and defaults to "_"
//! #[derive(Prefix)]
//! #[entid(prefix = "comment")]
//! struct Comment;
//!
//! # #[cfg(feature = "derive")]
//! fn main() {
//!     let user_id = UuidEntityId::<User>::generate();
//!     println!("User ID: {}", user_id); // e.g., "user_6ba7b810-9dad-11d1-80b4-00c04fd430c8"
//!     
//!     let comment_id = UuidEntityId::<Comment>::generate();
//!     println!("Comment ID: {}", comment_id); // e.g., "comment_6ba7b810-9dad-11d1-80b4-00c04fd430c8"
//! }
//! # #[cfg(not(feature = "derive"))]
//! # fn main() {}
//! ```

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

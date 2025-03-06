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
//! use entid::{Identifier, Prefix, UuidEntityId};
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
//!
//! // Get just the identifier part without the prefix
//! let id_str = user_id.id_str(); // e.g., "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
//!
//! // Get the identifier directly and access its string representation
//! let identifier = user_id.identifier();
//! let id_str_alt = identifier.as_str(); // Same as id_str()
//! ```
//!
//! ## Flexible Creation Methods
//!
//! ```rust
//! use entid::{Identifier, Prefix, UuidEntityId, Uuid};
//! use std::convert::TryFrom;
//! use std::str::FromStr;
//!
//! struct User;
//! impl Prefix for User {
//!     fn prefix() -> &'static str { "user" }
//! }
//!
//! // Using the generate method
//! let user_id1 = UuidEntityId::<User>::generate();
//!
//! // Using the new method with flexible string types
//! let id_str = format!("user_{}", Uuid::new_v4());
//! let user_id2 = UuidEntityId::<User>::new(&id_str).unwrap();
//! let user_id3 = UuidEntityId::<User>::new(id_str.clone()).unwrap();
//!
//! // Using TryFrom trait
//! let user_id4 = UuidEntityId::<User>::try_from(&id_str).unwrap();
//! let user_id5 = UuidEntityId::<User>::try_from(id_str.clone()).unwrap();
//!
//! // Using FromStr trait
//! let user_id6 = id_str.parse::<UuidEntityId<User>>().unwrap();
//!
//! // Using convenience methods
//! let uuid = Uuid::new_v4();
//! let user_id7 = UuidEntityId::<User>::with_uuid(uuid);
//! let user_id8 = UuidEntityId::<User>::new_v4();
//! let user_id9 = UuidEntityId::<User>::new_v5(&Uuid::NAMESPACE_DNS, "example.com");
//!
//! // Using the builder pattern
//! let user_id10 = UuidEntityId::<User>::builder().build();
//! let user_id11 = UuidEntityId::<User>::builder().with_uuid(uuid).build();
//! let user_id12 = UuidEntityId::<User>::builder().with_uuid_v4().build();
//! let user_id13 = UuidEntityId::<User>::builder().with_uuid_v5(&Uuid::NAMESPACE_DNS, "example.com").build();
//! ```
//!
//! ## Using the Derive Macro
//!
//! With the `derive` feature enabled, you can use the derive macro to implement the `Prefix` trait:
//!
//! ```rust
//! # #[cfg(feature = "derive")]
//! use entid::{Identifier, Prefix, UuidEntityId};
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
//!     // Get the full ID string with prefix
//!     let full_id = user_id.as_str(); // e.g., "user_6ba7b810-9dad-11d1-80b4-00c04fd430c8"
//!     
//!     // Get just the identifier part without the prefix
//!     let id_str = user_id.id_str(); // e.g., "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
//!     
//!     // Alternative way to get the identifier string
//!     let id_str_alt = user_id.identifier().as_str(); // Same as id_str()
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
pub use entity_id::{EntityId, EntityIdBuilder, Prefix, UlidEntityId, UuidEntityId};
pub use error::{EntityIdError, IdentifierError};
pub use identifier::{Identifier, UlidIdentifier, UuidIdentifier};

// Re-export UUID and ULID types for convenience
pub use ulid::Ulid;
pub use uuid::Uuid;

// Re-export derive macros when the "derive" feature is enabled
#[cfg(feature = "derive")]
pub use entid_derive::Prefix;

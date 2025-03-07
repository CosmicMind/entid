# entid

A Rust library for generating and validating type-safe, prefixed entity identifiers based on UUIDs and ULIDs.

[![Crates.io](https://img.shields.io/crates/v/entid.svg)](https://crates.io/crates/entid)
[![Documentation](https://docs.rs/entid/badge.svg)](https://docs.rs/entid)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Type-safe entity IDs**: Create distinct ID types for different entities
- **Multiple identifier formats**: Support for both UUID and ULID
- **Prefix support**: Automatically add entity-specific prefixes to IDs
- **Performance optimized**: Thread-safe caching of string representations
- **Serde compatible**: Seamless serialization and deserialization
- **Comprehensive error handling**: Clear error types for all operations
- **Zero-cost abstractions**: Minimal runtime overhead
- **Derive macro for implementing the `Prefix` trait**: Optional

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
entid = "0.4.1"
```

To use the derive macro for implementing the `Prefix` trait, enable the `derive` feature:

```toml
[dependencies]
entid = { version = "0.4.1", features = ["derive"] }
```

### API Overview

The `EntityId` type provides several methods for working with entity IDs:

```rust
type UserId = UuidEntityId::<User>;

// Create a new EntityId
let user_id = UserId::generate();

// Get the full ID string with prefix (e.g., "user_123e4567-e89b-12d3-a456-426614174000")
let full_id = user_id.as_str();

// Get just the identifier part without the prefix (e.g., "123e4567-e89b-12d3-a456-426614174000")
let raw_id = user_id.id_str();

// Get a reference to the underlying identifier object
let identifier = user_id.identifier();

// Get the identifier string directly from the identifier
let id_str = user_id.identifier().as_str();

// Get the prefix for this entity type
let prefix = UserId::prefix(); // "user"

// Get the delimiter for this entity type
let delimiter = UserId::delimiter(); // "_"

// For ULID-based IDs, get the timestamp
if let Some(timestamp_ms) = ulid_id.timestamp_ms() {
    println!("ID created at: {} ms since epoch", timestamp_ms);
}
```

### Flexible Creation Methods

The library provides multiple ways to create entity IDs:

```rust
use entid::{EntityId, Identifier, Prefix, UuidEntityId, UlidEntityId, Uuid, Ulid};

type UserId = UuidEntityId::<User>;

// Using the generate method
let user_id1 = UserId::generate();

// Using the new method with flexible string types
let id_str = "user_123e4567-e89b-12d3-a456-426614174000";
let user_id2 = UserId::new(id_str).unwrap();
let user_id3 = UserId::new(id_str.to_string()).unwrap();

// Using TryFrom trait
let user_id4 = UserId::try_from(id_str).unwrap();
let user_id5 = UserId::try_from(id_str.to_string()).unwrap();

// Using FromStr trait
let user_id6 = id_str.parse::<UuidEntityId<User>>().unwrap();

// Using convenience methods
let uuid = Uuid::new_v4();
let user_id7 = UserId::with_uuid(uuid);
let user_id8 = UserId::new_v4();
let user_id9 = UserId::new_v5(&Uuid::NAMESPACE_DNS, "example.com");

// Using the builder pattern
let user_id10 = UserId::builder().build();
let user_id11 = UserId::builder().with_uuid(uuid).build();
let user_id12 = UserId::builder().with_uuid_v4().build();
let user_id13 = UserId::builder().with_uuid_v5(&Uuid::NAMESPACE_DNS, "example.com").build();

// For ULID-based IDs
type PostId = UlidEntityId::<Post>;

let ulid = Ulid::new();
let post_id1 = PostId::with_ulid(ulid);
let post_id2 = PostId::with_timestamp(1625097600000); // July 1, 2021
let post_id3 = PostId::monotonic_from(Some(&post_id2));

// Using the builder pattern for ULID
let post_id4 = PostId::builder().with_ulid(ulid).build();
let post_id5 = PostId::builder().with_timestamp(1625097600000).build();
let post_id6 = PostId::builder().with_monotonic_from(Some(&post_id5)).build();
```

### Using EntityId in Collections

The `EntityId` type implements `Borrow<str>` and `AsRef<str>`, making it easy to use in collections:

```rust
use std::collections::{HashMap, HashSet};

// Use EntityId as a key in a HashMap
let mut user_map = HashMap::new();
user_map.insert(user_id1, "John Doe");

// Look up by string
let user = user_map.get(id_str);

// Use EntityId in a HashSet
let mut user_set = HashSet::new();
user_set.insert(user_id1);

// Check if a string is in the set
let contains = user_set.contains(id_str);
```

## Usage

### Basic Example with UUID

```rust
use entid::{EntityId, Prefix, UuidIdentifier, UuidEntityId};

type UserId = UuidEntityId::<User>;

// Define your entity types with custom prefixes
struct User;
impl Prefix for User {
    fn prefix() -> &'static str {
        "user"
    }

    fn delimiter() -> &'static str {
        "_"
    }
}

type PostId = EntityId::<Post, UuidIdentifier>;

struct Post;
impl Prefix for Post {
    fn prefix() -> &'static str {
        "post"
    }
    
    // Optional: Override the default delimiter
    fn delimiter() -> &'static str {
        "-"
    }
}

fn main() {
    // Generate random IDs with UUID
    let user_id = UserId::generate();
    let post_id = PostId::generate();
    
    // Print the IDs
    println!("User ID: {}", user_id); // e.g., "user_6ba7b810-9dad-11d1-80b4-00c04fd430c8"
    println!("Post ID: {}", post_id); // e.g., "post-123e4567-e89b-12d3-a456-426614174000"
    
    // Parse existing IDs
    let parsed_user_id = UserId::new("user_6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
    
    // Type safety prevents mixing different entity IDs
    // This won't compile:
    // let wrong: UuidEntityId<Post> = user_id;
}
```

### Using the Derive Macro

With the `derive` feature enabled, you can use the derive macro to implement the `Prefix` trait:

```rust
use entid::{Prefix, UuidEntityId, UlidEntityId};

type UserId = UuidEntityId::<User>;

#[derive(Prefix)]
#[entid(prefix = "user", delimiter = "_")]
struct User;

type PostId = UlidEntityId::<Post>;

#[derive(Prefix)]
#[entid(prefix = "post", delimiter = "-")]
struct Post;

type CommentId = UuidEntityId::<Comment>;

// The delimiter is optional and defaults to "_"
#[derive(Prefix)]
#[entid(prefix = "comment")]
struct Comment;

fn main() {
    let user_id = UserId::generate();
    println!("User ID: {}", user_id); // e.g., "user_6ba7b810-9dad-11d1-80b4-00c04fd430c8"
    
    let post_id = PostId::generate();
    println!("Post ID: {}", post_id); // e.g., "post-01H1VECZJYJ1QV2V0D0000JJDX"
    
    let comment_id = CommentId::generate();
    println!("Comment ID: {}", comment_id); // e.g., "comment_6ba7b810-9dad-11d1-80b4-00c04fd430c8"
}
```

### Using ULID Instead of UUID

```rust
use entid::{EntityId, Prefix, UlidIdentifier, UlidEntityId};

type ProductId = UlidEntityId::<Product>;

struct Product;
impl Prefix for Product {
    fn prefix() -> &'static str {
        "prod"
    }
}

fn main() {
    // Generate a ULID-based ID
    let product_id = ProductId::generate();
    
    // ULIDs are lexicographically sortable by creation time
    let product_ids: Vec<UProductId> = (0..10)
        .map(|_| ProductId::generate())
        .collect();
    
    // Sorting will order by creation time
    let mut sorted_ids = product_ids.clone();
    sorted_ids.sort();
    
    // Get the timestamp from a ULID (not available with UUID)
    if let Some(timestamp_ms) = product_id.timestamp_ms() {
        println!("Product ID created at: {} ms since epoch", timestamp_ms);
    }
}
```

### Using Deterministic UUIDs (v5)

```rust
use entid::{EntityId, Prefix, UuidIdentifier, Uuid};

type ApiKeyToken = EntityId::<ApiKey, UuidIdentifier>;

struct ApiKey;
impl Prefix for ApiKey {
    fn prefix() -> &'static str {
        "key"
    }
}

fn main() {
    // Create a namespace for your application
    let namespace = Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap();
    
    // Create a UUID v5 identifier
    let uuid_id = UuidIdentifier::new_v5(&namespace, "user@example.com");
    
    // Create an entity ID from the identifier
    let api_key = ApiKeyToken::from_identifier(uuid_id);
    
    // Same input produces the same ID
    let uuid_id2 = UuidIdentifier::new_v5(&namespace, "user@example.com");
    let api_key2 = ApiKeyToken::from_identifier(uuid_id2);
    
    assert_eq!(api_key, api_key2);
}
```

### Error Handling

```rust
use entid::{EntityId, EntityIdError, IdentifierError, Prefix, UuidIdentifier};

type UserId = <User, UuidIdentifier>;

struct User;
impl Prefix for User {
    fn prefix() -> &'static str {
        "user"
    }
}

fn parse_id(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse an entity ID string
    match EntityId::UserId::new(input) {
        Ok(id) => {
            println!("Successfully parsed ID: {}", id);
            Ok(())
        },
        Err(EntityIdError::InvalidFormat) => {
            // Handle invalid format (missing prefix or delimiter)
            println!("Invalid ID format: {}", input);
            Err(Box::new(EntityIdError::InvalidFormat))
        },
        Err(EntityIdError::InvalidIdentifier) => {
            // Handle invalid identifier (not a valid UUID/ULID)
            println!("Invalid identifier part in ID: {}", input);
            Err(Box::new(EntityIdError::InvalidIdentifier))
        }
    }
}

// Parse a raw identifier string
fn parse_raw_identifier(input: &str) -> Result<UuidIdentifier, IdentifierError> {
    UuidIdentifier::parse(input)
}
```

### Using with Serde

```rust
use entid::{EntityId, Prefix, UlidIdentifier};
use serde::{Serialize, Deserialize};

struct Order;
impl Prefix for Order {
    fn prefix() -> &'static str {
        "order"
    }
}

type OrderRecordId = EntityId<Order, UlidIdentifier>;

#[derive(Serialize, Deserialize)]
struct OrderRecord {
    id: OrderRecordId,
    customer_name: String,
    amount: f64,
}

fn main() {
    let order = OrderRecord {
        id: OrderRecordId::generate(),
        customer_name: "John Doe".to_string(),
        amount: 123.45,
    };
    
    // Serialize to JSON
    let json = serde_json::to_string(&order).unwrap();
    println!("JSON: {}", json);
    
    // Deserialize from JSON
    let deserialized: OrderRecord = serde_json::from_str(&json).unwrap();
    assert_eq!(order.id, deserialized.id);
}
```

### Using with Databases

```rust
use entid::{EntityId, Prefix, UuidIdentifier};

type CustomerId = EntityId<Customer, UuidIdentifier>;

struct Customer;
impl Prefix for Customer {
    fn prefix() -> &'static str {
        "cust"
    }
}

// Example with a hypothetical database library
fn store_in_db(customer_id: &CustomerId, name: &str) {
    // The ID will be stored as a string like "cust_123e4567-e89b-12d3-a456-426614174000"
    let id_str = customer_id.as_str();
    
    // You can also access the raw identifier if needed
    let uuid = customer_id.identifier().uuid();
    
    // Database operations...
}

fn retrieve_from_db(id_str: &str) -> Result<CustomerId, entid::EntityIdError> {
    // Parse the ID string back into an EntityId
    CustomerId::new(id_str)
}
```

## Advanced Usage

### Creating Monotonic ULIDs

```rust
use entid::{EntityId, Prefix, UlidIdentifier};

type TaskId = EntityId::<Task, UlidIdentifier>;

struct Task;
impl Prefix for Task {
    fn prefix() -> &'static str {
        "task"
    }
}

fn main() {
    // Create a ULID-based entity ID
    let task1 = TaskId::generate();
    
    // Create a monotonic ULID (ensures ordering even within the same millisecond)
    let ulid2 = UlidIdentifier::monotonic_from(Some(task1.identifier()));
    let task2 = TaskId::from_identifier(ulid2);
    
    // task2 is guaranteed to sort after task1
    assert!(task2 > task1);
}
```

### Custom Validation

```rust
use entid::{EntityId, Prefix, UuidIdentifier};

type ApiKeyToken = EntityId<ApiKey, UuidIdentifier>;

struct ApiKey;
impl Prefix for ApiKey {
    fn prefix() -> &'static str {
        "key"
    }
}

// Extend EntityId with custom validation logic
impl ApiKeyToken {
    pub fn is_valid_for_environment(&self, env: &str) -> bool {
        // Custom validation logic based on the UUID version
        match env {
            "production" => self.identifier().version() == Some(uuid::Version::Sha1),
            _ => true,
        }
    }
}
```

## Choosing Between UUID and ULID

### UUID Advantages
- Industry standard with wide adoption
- Multiple versions for different use cases (v1, v3, v4, v5)
- Well-supported in databases and other systems

### ULID Advantages
- Lexicographically sortable (sorts by creation time)
- URL-safe (no special characters)
- Shorter string representation (26 characters vs 36 for UUID)
- Built-in timestamp component

## Performance Considerations

- String representations are cached using `OnceLock` for thread-safe lazy initialization
- The `EntityId` type implements `Hash`, `PartialEq`, and `Eq` for efficient use in collections
- Memory usage is optimized by using `PhantomData` for type parameters

## License

This project is licensed under the MIT License - see the LICENSE file for details.

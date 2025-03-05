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

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
entid = "0.2.0"
```

To use the derive macro for implementing the `Prefix` trait, enable the `derive` feature:

```toml
[dependencies]
entid = { version = "0.2.0", features = ["derive"] }
```

## Usage

### Basic Example with UUID

```rust
use entid::{EntityId, Prefix, UuidIdentifier, UuidEntityId};

// Define your entity types with custom prefixes
struct User;
impl Prefix for User {
    fn prefix() -> &'static str {
        "user"
    }
}

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
    let user_id = UuidEntityId::<User>::generate();
    let post_id = EntityId::<Post, UuidIdentifier>::generate();
    
    // Print the IDs
    println!("User ID: {}", user_id); // e.g., "user_123e4567-e89b-12d3-a456-426614174000"
    println!("Post ID: {}", post_id); // e.g., "post-123e4567-e89b-12d3-a456-426614174000"
    
    // Parse existing IDs
    let parsed_user_id = UuidEntityId::<User>::new("user_123e4567-e89b-12d3-a456-426614174000").unwrap();
    
    // Type safety prevents mixing different entity IDs
    // This won't compile:
    // let wrong: UuidEntityId<Post> = user_id;
}
```

### Using the Derive Macro

With the `derive` feature enabled, you can use the `#[derive(Prefix)]` attribute to implement the `Prefix` trait:

```rust
use entid::{Prefix, UuidEntityId};

// Use the derive macro to implement the Prefix trait
#[derive(Debug, Prefix)]
#[prefix = "user"]
struct User;

// Use the derive macro with a custom delimiter
#[derive(Debug, Prefix)]
#[prefix = "post"]
#[delimiter = "-"]
struct Post;

fn main() {
    // Generate random IDs with UUID
    let user_id = UuidEntityId::<User>::generate();
    let post_id = UuidEntityId::<Post>::generate();
    
    // Print the IDs
    println!("User ID: {}", user_id); // e.g., "user_123e4567-e89b-12d3-a456-426614174000"
    println!("Post ID: {}", post_id); // e.g., "post-123e4567-e89b-12d3-a456-426614174000"
}
```

### Using ULID Instead of UUID

```rust
use entid::{EntityId, Prefix, UlidIdentifier, UlidEntityId};

struct Product;
impl Prefix for Product {
    fn prefix() -> &'static str {
        "prod"
    }
}

fn main() {
    // Generate a ULID-based ID
    let product_id = UlidEntityId::<Product>::generate();
    
    // ULIDs are lexicographically sortable by creation time
    let product_ids: Vec<UlidEntityId<Product>> = (0..10)
        .map(|_| UlidEntityId::<Product>::generate())
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
    let api_key = EntityId::<ApiKey, UuidIdentifier>::from_identifier(uuid_id);
    
    // Same input produces the same ID
    let uuid_id2 = UuidIdentifier::new_v5(&namespace, "user@example.com");
    let api_key2 = EntityId::<ApiKey, UuidIdentifier>::from_identifier(uuid_id2);
    
    assert_eq!(api_key, api_key2);
}
```

### Error Handling

```rust
use entid::{EntityId, EntityIdError, IdentifierError, Prefix, UuidIdentifier};

struct User;
impl Prefix for User {
    fn prefix() -> &'static str {
        "user"
    }
}

fn parse_id(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse an entity ID string
    match EntityId::<User, UuidIdentifier>::new(input) {
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

#[derive(Serialize, Deserialize)]
struct OrderRecord {
    id: EntityId<Order, UlidIdentifier>,
    customer_name: String,
    amount: f64,
}

fn main() {
    let order = OrderRecord {
        id: EntityId::<Order, UlidIdentifier>::generate(),
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

struct Customer;
impl Prefix for Customer {
    fn prefix() -> &'static str {
        "cust"
    }
}

// Example with a hypothetical database library
fn store_in_db(customer_id: &EntityId<Customer, UuidIdentifier>, name: &str) {
    // The ID will be stored as a string like "cust_123e4567-e89b-12d3-a456-426614174000"
    let id_str = customer_id.as_str();
    
    // You can also access the raw identifier if needed
    let uuid = customer_id.identifier().uuid();
    
    // Database operations...
}

fn retrieve_from_db(id_str: &str) -> Result<EntityId<Customer, UuidIdentifier>, entid::EntityIdError> {
    // Parse the ID string back into an EntityId
    EntityId::<Customer, UuidIdentifier>::new(id_str)
}
```

## Advanced Usage

### Creating Monotonic ULIDs

```rust
use entid::{EntityId, Prefix, UlidIdentifier};

struct Task;
impl Prefix for Task {
    fn prefix() -> &'static str {
        "task"
    }
}

fn main() {
    // Create a ULID-based entity ID
    let task1 = EntityId::<Task, UlidIdentifier>::generate();
    
    // Create a monotonic ULID (ensures ordering even within the same millisecond)
    let ulid2 = UlidIdentifier::monotonic_from(Some(task1.identifier()));
    let task2 = EntityId::<Task, UlidIdentifier>::from_identifier(ulid2);
    
    // task2 is guaranteed to sort after task1
    assert!(task2 > task1);
}
```

### Custom Validation

```rust
use entid::{EntityId, Prefix, UuidIdentifier};

struct ApiKey;
impl Prefix for ApiKey {
    fn prefix() -> &'static str {
        "key"
    }
}

// Extend EntityId with custom validation logic
impl EntityId<ApiKey, UuidIdentifier> {
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
/* Copyright © 2025, CosmicMind, Inc. */

//! Advanced usage example for the entid crate
//!
//! Run with: cargo run --example advanced_usage

use entid::{EntityId, Prefix, UlidIdentifier, Uuid, UuidIdentifier};
use serde::{Deserialize, Serialize};

// Define entity types
struct ApiKey;
impl Prefix for ApiKey {
    fn prefix() -> &'static str {
        "key"
    }
}

#[derive(Debug)]
struct Order;
impl Prefix for Order {
    fn prefix() -> &'static str {
        "order"
    }
}

// A serializable struct that uses EntityId
#[derive(Serialize, Deserialize, Debug)]
struct OrderRecord {
    id: EntityId<Order, UlidIdentifier>,
    customer_name: String,
    amount: f64,
}

fn main() {
    // Example 1: Deterministic UUIDs (v5)
    let namespace = Uuid::NAMESPACE_DNS;

    // Create deterministic UUIDs for the same input
    let uuid1 = UuidIdentifier::new_v5(&namespace, "user@example.com");
    let uuid2 = UuidIdentifier::new_v5(&namespace, "user@example.com");

    // Same input produces the same ID
    let api_key1 = EntityId::<ApiKey, UuidIdentifier>::from_identifier(uuid1);
    let api_key2 = EntityId::<ApiKey, UuidIdentifier>::from_identifier(uuid2);

    println!("API Key 1: {}", api_key1);
    println!("API Key 2: {}", api_key2);
    println!("Keys are equal: {}", api_key1 == api_key2);

    // Example 2: Monotonic ULIDs
    let order1 = EntityId::<Order, UlidIdentifier>::generate();

    // Create a monotonic ULID based on the previous one
    let ulid2 = UlidIdentifier::monotonic_from(Some(order1.identifier()));
    let order2 = EntityId::<Order, UlidIdentifier>::from_identifier(ulid2);

    println!("\nOrder 1: {}", order1);
    println!("Order 2: {}", order2);

    // Compare the raw ULIDs
    println!(
        "Order 2 > Order 1: {}",
        order2.identifier().ulid() > order1.identifier().ulid()
    );

    // Example 3: Serialization with serde
    let order = OrderRecord {
        id: EntityId::<Order, UlidIdentifier>::generate(),
        customer_name: "John Doe".to_string(),
        amount: 123.45,
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&order).unwrap();
    println!("\nSerialized Order:\n{}", json);

    // Deserialize from JSON
    let deserialized: OrderRecord = serde_json::from_str(&json).unwrap();
    println!("\nDeserialized Order: {:?}", deserialized);
    println!("IDs match: {}", order.id == deserialized.id);
}

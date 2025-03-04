/* Copyright © 2025, CosmicMind, Inc. */

use entid::{EntityId, Prefix, UlidEntityId, UlidIdentifier, Uuid, UuidEntityId, UuidIdentifier};

// Define test entity types
#[derive(Debug)]
struct User;
impl Prefix for User {
    fn prefix() -> &'static str {
        "user"
    }
}

#[derive(Debug)]
struct Post;
impl Prefix for Post {
    fn prefix() -> &'static str {
        "post"
    }

    fn delimiter() -> &'static str {
        "-"
    }
}

#[test]
fn test_uuid_entity_id() {
    // Generate a UUID-based entity ID
    let user_id = UuidEntityId::<User>::generate();

    // Test string representation
    let id_str = user_id.as_str();
    assert!(id_str.starts_with("user_"));

    // Test parsing
    let parsed_id = UuidEntityId::<User>::new(id_str).unwrap();
    assert_eq!(user_id, parsed_id);

    // Test error handling
    let result = UuidEntityId::<User>::new("invalid");
    assert!(result.is_err());

    let result = UuidEntityId::<User>::new("post_123e4567-e89b-12d3-a456-426614174000");
    assert!(result.is_err());
}

#[test]
fn test_ulid_entity_id() {
    // Generate a ULID-based entity ID
    let post_id = UlidEntityId::<Post>::generate();

    // Test string representation
    let id_str = post_id.as_str();
    assert!(id_str.starts_with("post-"));

    // Test parsing
    let parsed_id = UlidEntityId::<Post>::new(id_str).unwrap();
    assert_eq!(post_id, parsed_id);

    // Test timestamp
    assert!(post_id.timestamp_ms().is_some());
}

#[test]
fn test_uuid_v5_deterministic() {
    // Create a namespace
    let namespace = Uuid::NAMESPACE_DNS;

    // Create deterministic UUIDs
    let uuid1 = UuidIdentifier::new_v5(&namespace, "example.com");
    let uuid2 = UuidIdentifier::new_v5(&namespace, "example.com");

    // Same input should produce same UUID
    assert_eq!(uuid1, uuid2);

    // Create entity IDs
    let id1 = EntityId::<User, UuidIdentifier>::from_identifier(uuid1);
    let id2 = EntityId::<User, UuidIdentifier>::from_identifier(uuid2);

    assert_eq!(id1, id2);
}

#[test]
fn test_ulid_monotonic() {
    // Create a sequence of monotonic ULIDs
    let id1 = UlidEntityId::<User>::generate();

    // Create a monotonic ULID based on the previous one
    let ulid2 = UlidIdentifier::monotonic_from(Some(id1.identifier()));
    let id2 = EntityId::<User, UlidIdentifier>::from_identifier(ulid2);

    // Second ID should be greater than first (compare timestamps)
    let ts1 = id1.timestamp_ms().unwrap();
    let ts2 = id2.timestamp_ms().unwrap();
    assert!(ts2 >= ts1);

    // Compare the raw ULIDs
    assert!(id2.identifier().ulid() > id1.identifier().ulid());
}

#[test]
fn test_type_safety() {
    let _user_id = UuidEntityId::<User>::generate();
    let _post_id = UlidEntityId::<Post>::generate();

    // Different entity types cannot be compared
    // This would not compile:
    // assert_ne!(user_id, post_id);

    // Different identifier types cannot be compared
    // This would not compile:
    // let user_ulid_id = UlidEntityId::<User>::generate();
    // assert_ne!(user_id, user_ulid_id);
}

#[test]
fn test_serde() {
    // Test UUID serialization/deserialization
    let user_id = UuidEntityId::<User>::generate();
    let serialized = serde_json::to_string(&user_id).unwrap();
    let deserialized: UuidEntityId<User> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(user_id, deserialized);

    // Test ULID serialization/deserialization
    let post_id = UlidEntityId::<Post>::generate();
    let serialized = serde_json::to_string(&post_id).unwrap();
    let deserialized: UlidEntityId<Post> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(post_id, deserialized);
}

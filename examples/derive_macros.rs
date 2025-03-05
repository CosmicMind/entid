/* Copyright © 2025, CosmicMind, Inc. */

// Enable the derive feature to use the Prefix derive macro
#[cfg(feature = "derive")]
use entid::{Prefix, UlidEntityId, UuidEntityId};

// Define entity types using the derive macro
#[cfg(feature = "derive")]
#[derive(Prefix)]
#[entid(prefix = "user", delimiter = "_")]
struct User;

#[cfg(feature = "derive")]
#[derive(Prefix)]
#[entid(prefix = "post", delimiter = "-")]
struct Post;

#[cfg(feature = "derive")]
#[derive(Debug, Prefix)]
#[entid(prefix = "comment")]
struct Comment;

#[cfg(feature = "derive")]
fn main() {
    // Generate UUID-based entity IDs
    let user_id = UuidEntityId::<User>::generate();
    println!("User ID: {}", user_id); // e.g., "user_123e4567-e89b-12d3-a456-426614174000"
    assert!(user_id.to_string().starts_with("user_"));

    // Generate ULID-based entity IDs
    let post_id = UlidEntityId::<Post>::generate();
    println!("Post ID: {}", post_id); // e.g., "post-01H1VECZJYJ1QV2V0D0000JJDX"
    assert!(post_id.to_string().starts_with("post-"));

    // Parse existing IDs
    let comment_id_str = format!("comment_{}", uuid::Uuid::new_v4());
    let comment_id = UuidEntityId::<Comment>::new(&comment_id_str).unwrap();
    println!("Comment ID: {}", comment_id);

    // Demonstrate that the prefix and delimiter are correctly applied
    assert!(user_id.to_string().starts_with("user_"));
    assert!(post_id.to_string().starts_with("post-"));
    assert!(comment_id.as_str().starts_with("comment_"));
}

#[cfg(not(feature = "derive"))]
fn main() {
    println!("This example requires the 'derive' feature to be enabled.");
    println!("Run with: cargo run --example derive_macros --features derive");
}

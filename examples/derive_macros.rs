/* Copyright © 2025, CosmicMind, Inc. */

// Enable the derive feature to use the Prefix derive macro
use entid::{Prefix, UlidEntityId, UuidEntityId};

// Define entity types using the derive macro
#[derive(Debug, Prefix)]
#[prefix = "user"]
struct User;

#[derive(Debug, Prefix)]
#[prefix = "post"]
#[delimiter = "-"]
struct Post;

#[derive(Debug, Prefix)]
#[prefix = "comment"]
struct Comment;

fn main() {
    // Generate UUID-based entity IDs
    let user_id = UuidEntityId::<User>::generate();
    println!("User ID: {}", user_id); // e.g., "user_123e4567-e89b-12d3-a456-426614174000"

    // Generate ULID-based entity IDs
    let post_id = UlidEntityId::<Post>::generate();
    println!("Post ID: {}", post_id); // e.g., "post-01H1VECZJYJ1QV2V0D0000JJDX"

    // Parse existing IDs
    let comment_id_str = format!("comment_{}", uuid::Uuid::new_v4());
    let comment_id = UuidEntityId::<Comment>::new(&comment_id_str).unwrap();
    println!("Comment ID: {}", comment_id);

    // Demonstrate that the prefix and delimiter are correctly applied
    assert!(user_id.as_str().starts_with("user_"));
    assert!(post_id.as_str().starts_with("post-"));
    assert!(comment_id.as_str().starts_with("comment_"));
}

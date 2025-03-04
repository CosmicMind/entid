/* Copyright © 2025, CosmicMind, Inc. */

//! Basic usage example for the entid crate
//!
//! Run with: cargo run --example basic_usage

use entid::{Prefix, UlidEntityId, UuidEntityId};

// Define entity types with custom prefixes
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

    // Override the default delimiter
    fn delimiter() -> &'static str {
        "-"
    }
}

fn main() {
    // Generate UUID-based IDs
    let user_id = UuidEntityId::<User>::generate();
    println!("UUID-based User ID: {}", user_id);

    // Generate ULID-based IDs
    let post_id = UlidEntityId::<Post>::generate();
    println!("ULID-based Post ID: {}", post_id);

    // Parse existing IDs
    let user_id_str = user_id.as_str();
    let parsed_user_id = UuidEntityId::<User>::new(user_id_str).unwrap();
    println!("Parsed User ID: {}", parsed_user_id);

    // Demonstrate type safety
    // This would not compile:
    // let wrong: UuidEntityId<Post> = user_id;

    // Get timestamp from ULID (not available with UUID)
    if let Some(timestamp_ms) = post_id.timestamp_ms() {
        println!("Post ID created at: {} ms since epoch", timestamp_ms);
    }
}

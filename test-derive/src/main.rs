/* Copyright © 2025, CosmicMind, Inc. */

use entid::{Prefix, UuidEntityId};

// Define the User struct
#[derive(Debug)]
pub struct User;

// Manually implement the Prefix trait
impl Prefix for User {
    fn prefix() -> &'static str {
        "user"
    }
}

fn main() {
    // Now we can use the User pub struct with EntityId
    let user_id = UuidEntityId::<User>::generate();
    println!("User ID: {}", user_id);
    assert!(user_id.as_str().starts_with("user_"));
}

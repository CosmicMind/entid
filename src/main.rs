/* Copyright © 2025, CosmicMind, Inc. */

use entid::{Prefix, UuidEntityId};

#[derive(Prefix)]
#[entid(prefix = "user", delimiter = "_")]
pub struct User;

fn main() {
    let user_id = UuidEntityId::<User>::generate();
    println!("User ID: {}", user_id);
    assert!(user_id.to_string().starts_with("user_"));
}

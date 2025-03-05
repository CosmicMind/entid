use entid::{Prefix, UuidEntityId};

// Use the derive macro to implement the Prefix trait
#[derive(Debug, Prefix)]
#[prefix = "user"]
struct User;

fn main() {
    // Now we can use the User struct with EntityId
    let user_id = UuidEntityId::<User>::generate();
    println!("User ID: {}", user_id);
    assert!(user_id.as_str().starts_with("user_"));
}

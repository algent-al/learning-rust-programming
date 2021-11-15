
// Add _ to sign_in_count to prevent warnings because it is not being used
struct User {
    username: String,
    email: String,
    _sign_in_count: u64,
    active: bool,
}

fn main() {
    // make user1 mutable because we are going to change it later
    let mut user1 = User {
        email: String::from("someone1@example.com"),
        username: String::from("someone1"),
        active: true,
        _sign_in_count: 1,
    };

    println!("User's email is {}", user1.email);

    // Change username of user1
    user1.username = String::from("username1");
    println!("User's name is {}", user1.username);

    let email2 = String::from("someone3@example.com");
    let username2 = String::from("someone3");

    let user2 = build_user(email2, username2);
    println!("User email: {}", user2.email);

    let user3 = User {
        email: String::from("anotheruser@example.com"),
        username: String::from("anotheruser"),
        ..user1 // Remaining fields are same with user1
    };

    println!("user3: username={}, email={}", user3.username, user3.email);

    let inactive_user2 = User {
        active: false,
        ..user2
    };

    println!("Inactive user2 info - active: {}", inactive_user2.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        _sign_in_count: 1,
    }
}
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: false,
        sign_in_count: 42
    }
}

fn struct_update(email: String, user1: User) -> User {
    // returns a new instance of the struct with a different email, using struct update syntax
    User {
        email,
        ..user1
    }
}

// tuple structs are structs that are named but do not name any of their fields. Like this:

struct Point(i32, i32, i32); // Notice the parenthesis

fn use_point() {
    let origin = Point(0, 0, 0);
    let my_point = Point(4, 5, 6);

}
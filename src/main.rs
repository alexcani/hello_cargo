struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Tuple structs
struct Color(u32, u32, u32);  // uses ( ) rather than { }
struct Point(u32, u32);

// Unit-like struct
struct AlwaysEqual;  // no () nor {}

fn main() {
    let user = User {
        active: true,
        username: String::from("alexx109"),
        sign_in_count: 1,
        email: String::from("test@a.b.c"),
    };

    print_user(&user);

    let uname = String::from("sheep2000");
    let user = create_user(uname, String::from("a@b.c"));  // uname is moved
    print_user(&user);

    let user = User {
        active: false,
        username: String::from("haha"),
        ..user  // struct update syntax, take everything else from here (ownership rules apply)
    };

    print_user(&user);

    // Tuple struct instance
    let black = Color(127, 127, 127);  // () not {}
    let origin = Point(0, 0);
    println!("{} {} {} {} {}", black.0, black.1, black.2, origin.0, origin.1);

    let Point(x, y) = origin;  // destructuring
    println!("{} {}", x, y);

    // Unit-like
    let _x = AlwaysEqual;  // no () or {}
}

fn print_user(user: &User) {
    println!("Username: {}", user.username);
    println!("Active? {}", user.active);
    println!("Email: {}", user.email);
    println!("Sign In Count: {}", user.sign_in_count);
    println!();
}

fn create_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

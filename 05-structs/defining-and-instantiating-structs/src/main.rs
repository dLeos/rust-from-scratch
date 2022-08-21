struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


struct AnotherUser {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// Tuple struct examples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct example
struct AlwaysEqual;


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("user name: {}", user.username);
    println!("user email: {}", user.email);
    println!("user active: {}", user.active);
    println!("user sign_in_count: {}", user.sign_in_count);
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    user1.email = String::from("anotheremail@example.com");

    print_user(&user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // user1 moved to user2

    print_user(&user2);

    let user3 = User {
        username: String::from("anotherusername"),
        email: String::from("another@example.com"),
        ..user2
    };
    // user2 not meved to user3 because only Copy fields assigned from user2 to user3

    print_user(&user2);
    print_user(&user3);


    // this does not work, because user3 and another_user should be of the same type
    // let another_user = AnotherUser {
    //     ..user3
    // };


    // Tuple struct examples
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color: {} {} {}", black.0, black.1, black.2);
    println!("Type of black: {}", std::any::type_name::<Color>());
    println!("Type of origin: {}", std::any::type_name::<Point>());
}

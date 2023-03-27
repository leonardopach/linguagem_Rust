#[derive(Debug)]
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    activate: bool,
}
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(unused_variables)]
pub fn structs_print() {
    let mut user1 = User {
        username: "Leonardo".to_string(),
        email: "leo@gmail.com".to_string(),
        sign_in_count: 12,
        activate: false,
    };
    let name = &user1.username;

    println!("{}", user1.username);
    println!("{}", name);
    println!("{:?}", user1);

    user1.username = String::from("pedro");
    println!("{:?}", user1);

    let user2 = build_user(String::from("teste@gmail.com"), "lucas".to_string());
    println!("{:?}", user2);

    let user3 = User {
        username: "leonardo".to_string(),
        email: "teste@gmail.com".to_string(),
        ..user1
    };

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rectangle)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        activate: true,
        sign_in_count: 1,
    }
}

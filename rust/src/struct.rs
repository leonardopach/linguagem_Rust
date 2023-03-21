#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?} {:?}", black, origin);
    let mut user1 = User {
        email: String::from("alguem@exemplo.com"),
        username: String::from("algumnome123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("outroemail@exemplo.com");
    println!("{:?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("\n{:?}\n", user2);
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn print_user() {
    structs();
    println!(
        "{:?}",
        build_user("hola@gmail.com".to_string(), "pedro".to_string())
    );
    let (num, string) = (10, "hello");

    println!("{}, {1}", num, string);
}

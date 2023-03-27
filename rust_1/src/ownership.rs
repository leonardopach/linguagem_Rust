pub fn owner_ship() {
    let x: i32 = 5;
    let y = x; //copy

    println!("{} {}", x, y);

    let s1 = String::from("hello");
    let s2 = &s1;

    println!("{} {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(&s);
    println!("{}", s);

    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("");
    println!("{}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn takes_ownership(some_string: &String) {
    println!("{}", some_string);
}

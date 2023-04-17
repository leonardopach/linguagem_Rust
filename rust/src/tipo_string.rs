fn print_string() {
    let mut s = String::from("olá");
    s.push_str(" mundo");
    println!("{s}");
    let x = 5;
    let y = x;

    println!("{x} {y}");
    let s1 = String::from("texto");
    let s2 = s1.clone();
    println!("{s1} {s2}");
}

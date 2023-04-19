pub fn print_input() {
    println!("what is your name? ");

    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("failed to read");
    println!("Hello {}", name);
}

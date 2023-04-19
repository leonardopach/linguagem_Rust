pub fn print_match() {
    let age: i32 = 8;

    match age {
        3..=17 => println!("Adolencente"),
        18..=50 => println!("Adulto"),
        51..=i32::MAX => println!("velho"),
        _ => println!("Se tiver vivo e um vampiro"),
    }

    println!("Digite um número entre 1 e 10:");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    let number: i32 = match input.trim().parse() {
        Ok(n) if n > 1 && n <= 10 => n,
        Ok(_) => {
            println!("O numero precisa estar entre 1 e 10");
            return;
        }
        Err(e) => {
            println!("Digite um numero");
            return;
        }
    };

    println!("o numero digitado é {number}");
}

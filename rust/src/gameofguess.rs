use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn gameofguess() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=101);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(e) => panic!("Failed to read line {e}"),
        };

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too smail!".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You guesse is right... You Win!".green());
                break;
            }
        }
    }
}

pub fn print_numbers_to(x: i32) {
    for i in 1..x {
        if is_even(i) {
            println!("is even: {}", i)
        }
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

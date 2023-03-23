pub fn while_loop() {
    let mut n = 1;

    while n <= 50 {
        if n % 5 == 0 {
            println!("the value of n is : {}", n);
        }
        n += 1;
    }
}

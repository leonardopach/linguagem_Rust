pub fn loop_print() {
    let mut n = 0;
    loop {
        n += 1;

        if n % 2 == 0 {
            continue;
        }

        if n > 100 {
            break;
        }

        println!("The value of n Is : {}", n);
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {}", result);
}

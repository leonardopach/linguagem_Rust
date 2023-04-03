pub fn closures() {
    let sum = |x: i32, y: i32| x + y;

    println!("the sum of 10 + 2 is: {}", sum(10, 2));
    let is_even = |num: i32| -> bool { num % 2 == 0 };

    let print_numbers_to = |x: i32| {
        for i in 1..x {
            if is_even(i) {
                println!("is even: {}", i)
            }
        }
    };
    print_numbers_to(20);

    let x = 4;

    let equal_to_x = |z: i32| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

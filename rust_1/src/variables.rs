pub fn variables_print() {
    let mut x: i32 = 45;
    let f: f32 = 6.7;
    let b: bool = false;

    println!("The value of x is : {}", x);
    println!("The value of f is : {}", f);
    println!("The value of b is : {}", b);

    x = 60;
    println!("The value of x is : {}", x);
}

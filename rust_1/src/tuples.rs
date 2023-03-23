pub fn tuples() {
    let tup1 = (20, "Rust", 30, 3.4, false, (1, 4, 7));

    println!("{}", tup1.2);
    println!("{}", tup1.1);
    println!("{}", tup1.3);
    println!("{}", tup1.4);
    println!("{:?}", tup1.5);
    println!("{}", (tup1.5).0);
}

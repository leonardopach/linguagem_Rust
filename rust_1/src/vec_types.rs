pub fn vec_print() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    println!("Array: {:?}, Vec:{:?}", a, v);
    let v2 = vec![1, 2, 3, 4, 5, 6];
    println!("{:?} {}", v2, v2[2]);

    let slices = &v2[0..3];
    println!("{:?}", slices);
    match v.get(3) {
        Some(third) => println!("The third elements is {}", third),
        None => println!("There is no third element"),
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}

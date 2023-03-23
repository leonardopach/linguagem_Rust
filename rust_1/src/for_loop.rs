pub fn for_loop() {
    for i in 1..=11 {
        println!("The number is {}", i);
    }

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for (index, name) in animals.iter().enumerate() {
        println!("The {} animals are: {}", index, name);
    }
}

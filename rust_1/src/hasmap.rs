use std::collections::HashMap;

pub fn hash_map_print() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(&blue, 10);
    scores.insert(&yellow, 50);

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }

    let text = "Hello world worderfull world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

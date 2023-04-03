pub fn iterator_print() {
    let v1 = vec![1, 2, 3, 4];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }

    let shoes = vec![
        Shoe {
            size: 12,
            style: "all start".to_string(),
        },
        Shoe {
            size: 10,
            style: String::from("Nike"),
        },
    ];
    println!("{:?}", map_iter());
    println!("{:?}", shoes_in_my_size(shoes, 12));
    iterator_sum();

    let v1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    let x = v1
        .iter()
        .map(|x| x + 1)
        .filter(|x| *x > 2 && *x < 7)
        .collect::<Vec<i32>>();

    let y = x;

    println!("{:?}", y);
}

fn map_iter() -> Vec<i32> {
    let v1 = vec![1, 2, 3, 4];
    let v2: Vec<_> = v1.iter().map(|x: &i32| x + 1).collect();
    return v2;
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size <= shoe_size).collect()
}

// #[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3, 4];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 10);
}

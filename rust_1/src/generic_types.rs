#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}
pub fn generic_types() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest_number(&number_list));

    let number_list2 = vec![102, 34, 6000, 89, 50, 25, 100, 65];
    println!("The largest number is {}", largest_number(&number_list2));
    let caracter_list = vec!["m", "y", "t", "a", "z"];
    println!("The largest number is {}", largest_number(&caracter_list));

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "r", y: "a" };

    println!("{} {}", p1.x, p2.y);
    println!("{:?} {:?}", p1, p2);
    let p1 = Point { x: 5, y: 10 };
    println!("{}", p1.x());
    let p1 = Point { x: 5.0, y: 10.2 };
    println!("{}", p1.y());
}

fn largest_number<T: PartialOrd>(number_list: &Vec<T>) -> &T {
    let mut largest = &number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

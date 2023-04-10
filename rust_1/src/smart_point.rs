use std::rc::Rc;

#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn smart_point() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let b = Box::new(5);

    // println!("b: {}", b);
    // println!("list: {:?}", list);

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!(
        "Count after creating b = {} , {:?}",
        Rc::strong_count(&a),
        b
    );
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}, {:?}", Rc::strong_count(&a), c);
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}

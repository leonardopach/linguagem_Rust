use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub fn deref_trait() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rusty"));
    hello(&m);
    // &MyBox<String> -> &String -> &str
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello: {}", name);
}
#[test]
fn test_trait() {
    deref_trait()
}

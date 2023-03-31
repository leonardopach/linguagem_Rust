pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub mod cli_app;
mod lifetimes;

pub fn greeting(name: &str) -> String {
    format!("hello, {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("carol");
        assert!(
            result.contains("carol"),
            "Greeting did not contain name, value was '{}'",
            result
        )
    }
    #[test]
    fn is_add_two() {
        assert_eq!(4, add_two(2))
    }
    #[test]
    fn it_compare_longest() {
        let string1 = String::from("abcd");

        let string2: String = String::from("xyz");
        let doze = String::from("abcd");

        assert_eq!(&doze, lifetimes::longest(&string1, &string2));
    }
}

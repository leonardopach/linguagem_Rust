#[cfg(test)]
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        client::connect();
    }
}

mod outermost {
    pub fn middle_secret_function() {
        println!("middle_secret_function");
    }
    pub fn middle_function() {}

    pub mod inside {
        pub fn inner_function() {}
        pub fn secret_function() {}
    }
}

pub fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

pub fn print_use_mod() {
    of::nested_modules();

    let _red = Red;
    let _yellow = Yellow;
    let _green = TrafficLight::Green;
}

#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn enum_print() {
    let player_direction: Direction = Direction::Down;

    match player_direction {
        Direction::Up => println!("We are heading up"),
        Direction::Down => println!("We are heading Down"),
        Direction::Left => println!("We are heading Left"),
        Direction::Right => println!("We are heading Right"),
    }
}

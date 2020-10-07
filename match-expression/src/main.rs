enum Action {
    Drive,
    Pickup,
    Turn(Direction),
    Stop
}

enum Direction {
    Left,
    Right,
}

fn print_action(a: &Action) {
    match a {
        Action::Drive => println!("Drive forward!"),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("Turn left!"),
            Direction::Right => println!("Turn right!"),
        }
        Action::Stop => println!("Stop!"),
        Action::Pickup => println!("Pick up!"),
    }
}

fn main() {
    //use self::Action::*;
    let program = vec![
        Action::Drive,
        Action::Turn(Direction::Left),
        Action::Drive, 
        Action::Stop,
        Action::Pickup,
        Action::Turn(Direction::Left),
        Action::Turn(Direction::Left),
        Action::Turn(Direction::Left),
        Action::Turn(Direction::Left),
        Action::Drive, 
        Action::Turn(Direction::Right),
        Action::Drive,
        Action::Stop,
    ];

    for action in program {
        print_action(&action);
    }
}

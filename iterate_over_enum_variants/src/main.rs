use strum::IntoEnumIterator;
use strum::EnumIter;

#[derive(Debug, EnumIter)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn main() {
    for direction in Direction::iter() {
        println!("{:?}", direction);
    }
}

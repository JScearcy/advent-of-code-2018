use std::collections::BTreeMap;

pub struct Rails {
    carts: BTreeMap<Point, Cart>,
    rails: Vec<Vec<char>>,
}

struct Cart {
    location: Point,
    direction: Direction,

}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Point = (usize, usize);
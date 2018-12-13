use std::collections::BTreeMap;

pub struct Rails {
    carts: BTreeMap<Point, Cart>,
    rails: Vec<Vec<Rail>>,
}
impl Rails {
    pub fn new(input: &str) -> Rails {
        let input_lines = input.split("\n");
        let mut rails = vec![];
        let mut carts: BTreeMap<Point, Cart> = BTreeMap::new();
        for input_line in input_lines {
            let mut rail_line = vec![];
            let rail_chars = input_line.split("");
            for rail_char in rail_chars {
                match rail_char {
                    "-" => rail_line.push(Rail::Horizontal),
                    "|" => rail_line.push(Rail::Vertical),
                    "\\" => rail_line.push(Rail::Left),
                    "/" => rail_line.push(Rail::Right),
                    "+" => rail_line.push(Rail::Intersection),
                    " " => rail_line.push(Rail::Empty),
                    "<" | ">" | "^" | "v" => {
                        let x = rail_line.len();
                        let y = rails.len();
                        let new_cart = Cart::new((y, x), rail_char);
                        match new_cart.current_direction {
                            Direction::Up | Direction::Down => rail_line.push(Rail::Vertical),
                            Direction::Left | Direction::Right => rail_line.push(Rail::Horizontal),
                        }
                        carts.insert(new_cart.location, new_cart);
                    }
                    "" => {}
                    _ => panic!(
                        "Unknown character, missed one in the matching: {}",
                        rail_char
                    ),
                };
            }
            rails.push(rail_line);
        }

        Rails { carts, rails }
    }

    pub fn ride_the_rails(&mut self, remove_crashes: bool) {
        let mut tick = 1;
        'outer: loop {
            let mut new_carts = BTreeMap::new();
            for (_point, cart) in &self.carts {
                let mut new_cart = cart.clone();
                if !new_carts.contains_key(&new_cart.location) {
                    let new_location = new_cart.tick(&self.rails);
                    let current_carts_contains_crash = self.carts.contains_key(&new_location);
                    let new_carts_contains_crash = new_carts.contains_key(&new_location);
                    if current_carts_contains_crash || new_carts_contains_crash {
                        if !remove_crashes {
                            println!(
                                "Tick: {}, Crash: {},{}",
                                tick, new_location.1, new_location.0
                            );
                            break 'outer;
                        } else {
                            println!(
                                "Tick: {}, Crash: {},{}",
                                tick, new_location.1, new_location.0
                            );
                            new_cart.crashed = true;
                            new_carts.insert(new_location, new_cart);
                        }
                    } else {
                        new_carts.insert(new_location, new_cart);
                    }
                }
            }
            if !remove_crashes {
                self.carts = new_carts;
            } else {
                self.carts = new_carts.iter().filter(|(_loc, cart)| !cart.crashed).fold(
                    BTreeMap::new(),
                    |mut tree, (location, cart)| {
                        tree.insert(*location, cart.to_owned());
                        tree
                    },
                );

                if self.carts.len() == 1 {
                    let last_cart_standing: Vec<(&Point, &Cart)> =
                        self.carts.iter().take(1).collect();
                    let last_cart_point = last_cart_standing[0].0;
                    println!(
                        "Last cart standing: {}, {}",
                        last_cart_point.1, last_cart_point.0
                    );
                    break 'outer;
                }
            }
            tick += 1;
        }
    }
}

#[derive(Clone, Debug)]
struct Cart {
    location: Point,
    current_direction: Direction,
    last_turn: Turn,
    crashed: bool,
}
impl Cart {
    pub fn new(location: Point, char_pos: &str) -> Cart {
        let direction = match char_pos {
            "^" => Direction::Up,
            "v" => Direction::Down,
            "<" => Direction::Left,
            ">" => Direction::Right,
            _ => panic!("Unexpected character for cart"),
        };

        Cart {
            location,
            current_direction: direction,
            last_turn: Turn::Right,
            crashed: false,
        }
    }

    pub fn tick(&mut self, rails: &Vec<Vec<Rail>>) -> Point {
        let next_location = self.next_location();
        let next_rail = &rails[next_location.0][next_location.1];
        let (next_direction, turn_opt) = self.next_direction(next_rail);

        if let Some(turn) = turn_opt {
            self.last_turn = turn;
        }

        self.current_direction = next_direction;
        self.location = next_location;

        self.location
    }

    fn next_direction(&self, turn_char: &Rail) -> (Direction, Option<Turn>) {
        match turn_char {
            // matches \
            Rail::Left => match self.current_direction {
                Direction::Up => (Direction::Left, None),
                Direction::Down => (Direction::Right, None),
                Direction::Left => (Direction::Up, None),
                Direction::Right => (Direction::Down, None),
            },
            // matches /
            Rail::Right => match self.current_direction {
                Direction::Up => (Direction::Right, None),
                Direction::Down => (Direction::Left, None),
                Direction::Left => (Direction::Down, None),
                Direction::Right => (Direction::Up, None),
            },
            Rail::Vertical | Rail::Horizontal => (self.current_direction.clone(), None),
            Rail::Intersection => {
                let next_turn = Cart::next_turn(&self.last_turn);
                match next_turn {
                    Turn::Left => match self.current_direction {
                        Direction::Up => (Direction::Left, Some(next_turn)),
                        Direction::Down => (Direction::Right, Some(next_turn)),
                        Direction::Left => (Direction::Down, Some(next_turn)),
                        Direction::Right => (Direction::Up, Some(next_turn)),
                    },
                    Turn::Right => match self.current_direction {
                        Direction::Up => (Direction::Right, Some(next_turn)),
                        Direction::Down => (Direction::Left, Some(next_turn)),
                        Direction::Left => (Direction::Up, Some(next_turn)),
                        Direction::Right => (Direction::Down, Some(next_turn)),
                    },
                    Turn::Straight => (self.current_direction.clone(), Some(next_turn)),
                }
            }
            Rail::Empty => panic!("Rail is empty - cart off the rails!"),
        }
    }

    fn next_location(&self) -> Point {
        match self.current_direction {
            Direction::Up => (self.location.0 - 1, self.location.1),
            Direction::Down => (self.location.0 + 1, self.location.1),
            Direction::Left => (self.location.0, self.location.1 - 1),
            Direction::Right => (self.location.0, self.location.1 + 1),
        }
    }

    fn next_turn(last_turn: &Turn) -> Turn {
        match last_turn {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
enum Turn {
    Straight,
    Left,
    Right,
}

enum Rail {
    Left,
    Right,
    Vertical,
    Horizontal,
    Intersection,
    Empty,
}

type Point = (usize, usize);

mod power_grid;

use crate::power_grid::PowerGrid;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");
    let mut input_contents = String::new();
    input_file
        .read_to_string(&mut input_contents)
        .expect("couldn't read the input");
    let mut file = File::create("results.txt").expect("couldn't create results file");
    
    let grid_id= isize::from_str_radix(&input_contents, 10).expect("Input was not a number");
    let power_grid = PowerGrid::init(grid_id, 90000, true);
    println!("{:?}", power_grid.get_most_powerful(Some(&mut file)));
}

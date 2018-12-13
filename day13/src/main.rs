mod rails;

use crate::rails::Rails;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");
    let mut input_contents = String::new();
    input_file
        .read_to_string(&mut input_contents)
        .expect("couldn't read the input");

    let mut rails = Rails::new(&input_contents);
    rails.ride_the_rails(true);
}

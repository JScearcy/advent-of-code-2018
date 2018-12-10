extern crate regex;
mod sky;

use crate::sky::Sky;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");
    let mut input_contents = String::new();
    input_file
        .read_to_string(&mut input_contents)
        .expect("couldn't read the input");

    let mut file = File::create("results.txt").expect("couldn't create results file");

    let mut sky = Sky::new();
    sky.create_the_stars_in_the_sky(&input_contents);
    sky.only_want_message();
    sky.write_to_file(&mut file);
}

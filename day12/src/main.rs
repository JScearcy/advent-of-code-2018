extern crate regex;
mod plants;

use crate::plants::Planter;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");
    let mut input_contents = String::new();
    input_file
        .read_to_string(&mut input_contents)
        .expect("couldn't read the input");
    let mut file = File::create("results.json").expect("couldn't create results file");
    
    let mut planter = Planter::new(&input_contents, Some(&mut file));
    planter.run_steps(50000000000);
}

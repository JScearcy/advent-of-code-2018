extern crate regex;

mod sorter;

use sorter::SleighSorter;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");
    let mut input_contents = String::new();
    input_file
        .read_to_string(&mut input_contents)
        .expect("couldn't read the input");

    let sleigh_sorter = SleighSorter::new(&input_contents);

    let mut file = File::create("results.json").expect("couldn't create results file");
    let write_string = format!("{{ \"order\": \"{}\", \"time\": {} }}", sleigh_sorter.order, sleigh_sorter.time);
    file.write_all(write_string.as_bytes()).expect("couldn't write to results file");
}

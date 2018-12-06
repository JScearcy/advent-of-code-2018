mod polymer;

use polymer::Polymer;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");
    let mut input_contents = String::new();
    input_file
        .read_to_string(&mut input_contents)
        .expect("couldn't read the input");

    let mut polymer = Polymer::new();
    polymer.add_chain(&input_contents);
    let best_chain_len = polymer.best_chain_len();

    let mut file = File::create("results.json").expect("couldn't create results file");
    let write_string = format!(
        "{{ \n\t\"destroyedPolymer\": {},\n\t\"bestDestroyedPolymer\": {}\n}}",
        polymer.len(),
        best_chain_len
    );
    file.write_all(write_string.as_bytes())
        .expect("couldn't write to results file");
}

mod fabric;

use fabric::{Fabric, Rectangle};
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut fabric = Fabric::new();
    let mut input_file = File::open("input.txt").expect("file not found");
    let mut input_contents = String::new();
    input_file
        .read_to_string(&mut input_contents)
        .expect("couldn't read the input");

    let rectangle_strings = input_contents.split("\n");
    for rectangle_string in rectangle_strings {
        let rectangle = Rectangle::from_str(rectangle_string);
        fabric.add_claim(rectangle);
    }

    let rectangle_strings = input_contents.split("\n");
    let mut no_overlap_rectangle = "";
    for rectangle_string in rectangle_strings {
        let rectangle = Rectangle::from_str(rectangle_string);
        let has_overlap = fabric.does_claim_overlap(rectangle);

        if !has_overlap {
            no_overlap_rectangle = rectangle_string;
            break;
        }
    }

    let mut file = File::create("results.json").expect("couldn't create results file");
    let write_string = format!(
        "{{ \n\t\"overlapClaimInches\": {},\n\t\"noOverlapRectangle\": \"{}\"\n}}",
        fabric.overlap_claim_inches(),
        no_overlap_rectangle
    );
    file.write_all(write_string.as_bytes()).expect("couldn't write to results file");
}

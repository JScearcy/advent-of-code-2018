mod map;

use map::Map;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut input_file = File::open("input.txt").expect("file not found");
    let mut input_contents = String::new();
    input_file
        .read_to_string(&mut input_contents)
        .expect("couldn't read the input");
    
    let coordinates = input_contents.split("\n");
    let mut parsed_coordinates = vec![];
    for coordinate in coordinates {
        let point: (isize, isize) = coordinate.split(", ").fold((999, 999), |tuple, coord_part| {
            let parsed_part = isize::from_str_radix(coord_part, 10).expect("coordinate was not a valid integer");
            if tuple.0 == 999 {
                (parsed_part, tuple.1)
            } else {
                (tuple.0, parsed_part)
            }
        });

        parsed_coordinates.push(point);
    }

    let mut map = Map::new(&parsed_coordinates);
    let chars = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut count = 0;
    for coordinate in parsed_coordinates {
        // let letter = chars[count];
        map.add_parent(coordinate.0, coordinate.1, 'A');
        count += 1;
    }
    // let mut unchanged_parents = vec![];
    // map.calculate_areas(1, &mut unchanged_parents);
    map.calculate_nearest_points(10000);
    // map.print();

    let mut file = File::create("results.json").expect("couldn't create results file");
    let write_string = format!("");
    file.write_all(write_string.as_bytes())
        .expect("couldn't write to results file");
}

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Write;

pub struct PowerGrid {
    get_best_square: bool,
    grid: Vec<isize>,
}

impl PowerGrid {
    pub fn init(grid_id: isize, grid_size: usize, get_best_square: bool) -> PowerGrid {
        let side_size = (grid_size as f32).sqrt() as usize;
        let mut grid = vec![];
        for power_cell in 0..grid_size {
            let (x, y) = PowerGrid::grid_location_from_index(power_cell, side_size);
            let power_level = PowerGrid::get_power_level(grid_id, x, y);
            grid.push(power_level);
        }
        let power_grid = PowerGrid {
            get_best_square,
            grid,
        };

        power_grid
    }

    pub fn get_most_powerful(&self, file: Option<&mut File>) -> ((isize, isize), usize) {
        let grid_size = self.grid.len();
        let side_size = (grid_size as f32).sqrt() as usize;
        let mut largest_power = 0;
        let mut largest_power_corner = (0, 0);
        let mut square_size = 0;
        if self.get_best_square {
            let mut square_builder: HashMap<(isize, isize), isize> = HashMap::new();
            for current_square_size in 0..side_size as isize {
                for index in 0..grid_size {
                    let mut internal_power_sum = 0;
                    let mut ring_count = 0;
                    let (grid_location_x, grid_location_y) =
                        PowerGrid::grid_location_from_index(index, side_size);
                    // if index == 0 || index == 299 || index == 89700 || index == 89999 {
                    //     println!("({}, {})", grid_location_x, grid_location_y);
                    // }
                    if grid_location_x + current_square_size < side_size as isize
                        && grid_location_y + current_square_size < side_size as isize
                    {
                        if square_builder.contains_key(&(grid_location_x, grid_location_y)) {
                            let current_square_power = square_builder
                                .get(&(grid_location_x, grid_location_y))
                                .unwrap();
                            let (ring_power, count) = self.get_ring_amount(
                                grid_size,
                                side_size,
                                current_square_size as usize,
                                grid_location_x as usize,
                                grid_location_y as usize,
                            );
                            let combined_power = current_square_power + ring_power;
                            ring_count = count;
                            internal_power_sum = combined_power;
                            square_builder
                                .insert((grid_location_x, grid_location_y), combined_power);
                        } else {
                            internal_power_sum += self.get_grid_level(
                                grid_size,
                                side_size,
                                current_square_size as usize,
                                grid_location_x as usize,
                                grid_location_y as usize,
                            );
                            square_builder
                                .insert((grid_location_x, grid_location_y), internal_power_sum);
                        }

                        if internal_power_sum > largest_power {
                            largest_power = internal_power_sum;
                            largest_power_corner = (grid_location_x, grid_location_y);
                            square_size = current_square_size + 1;
                            println!(
                                "largest_power_corner: {:?}, square_size: {}, power: {}, count: {}",
                                largest_power_corner, square_size, largest_power, ring_count
                            );
                        }
                    }
                }
            }
        } else {
            for index in 0..grid_size {
                let (grid_location_x, grid_location_y) =
                    PowerGrid::grid_location_from_index(index, side_size);
                let square_side_len = 3;
                let internal_power_sum = self.get_grid_level(
                    grid_size,
                    side_size,
                    square_side_len,
                    grid_location_x as usize,
                    grid_location_y as usize,
                );

                if internal_power_sum > largest_power {
                    largest_power = internal_power_sum;
                    largest_power_corner = (grid_location_x, grid_location_y);
                }
            }
        }

        if let Some(file) = file {
            let output = format!(
                "{{ \"largestPowerCorner\": \"({},{})\", \"squareSize\": {} }}",
                largest_power_corner.0, largest_power_corner.1, square_size
            );
            file.write_all(output.as_bytes())
                .expect("couldn't write to results file");
        }

        (largest_power_corner, square_size as usize)
    }

    fn get_ring_amount(
        &self,
        grid_size: usize,
        side_size: usize,
        square_size: usize,
        start_x: usize,
        start_y: usize,
    ) -> (isize, usize) {
        let mut count = 0;
        let mut internal_grid_values = vec![];
        for x in start_x..=start_x + square_size {
            count += 1;
            let y = start_y + square_size;
            let next_index = PowerGrid::index_from_grid_location(x as usize, y as usize, side_size);
            if next_index == usize::max_value() {
                continue;
            } else if next_index >= grid_size {
                return (0, count);
            }
            internal_grid_values.push(self.grid[next_index]);
        }

        for y in start_y..start_y + square_size {
            count += 1;
            let x = start_x + square_size;
            let next_index = PowerGrid::index_from_grid_location(x as usize, y as usize, side_size);
            if next_index == usize::max_value() {
                continue;
            } else if next_index >= grid_size {
                return (0, count);
            }
            internal_grid_values.push(self.grid[next_index]);
        }
        if count == 11 {}

        (
            internal_grid_values
                .iter()
                .fold(0, |total, curr| total + curr),
            count,
        )
    }

    fn get_grid_level(
        &self,
        grid_size: usize,
        side_size: usize,
        square_size: usize,
        start_x: usize,
        start_y: usize,
    ) -> isize {
        let mut internal_grid_values = vec![];
        for y in start_y..start_y + square_size {
            for x in start_x..start_x + square_size {
                let next_index =
                    PowerGrid::index_from_grid_location(x as usize, y as usize, side_size);
                if next_index == usize::max_value() {
                    continue;
                } else if next_index >= grid_size {
                    return 0;
                }
                internal_grid_values.push(self.grid[next_index]);
            }
        }
        internal_grid_values
            .iter()
            .fold(0isize, |total, curr| total + curr)
    }

    fn get_power_level(grid_id: isize, x: isize, y: isize) -> isize {
        let rack_id = x + 10;
        let initial_power_level = (rack_id * y + grid_id) * rack_id;
        PowerGrid::get_hundreds(initial_power_level) - 5
    }

    fn grid_location_from_index(index: usize, side_size: usize) -> (isize, isize) {
        let initial_x = index % side_size;
        let initial_y = (index - initial_x) / side_size;

        ((initial_x + 1) as isize, (initial_y + 1) as isize)
    }

    fn index_from_grid_location(x: usize, y: usize, side_size: usize) -> usize {
        if x != 0 && y != 0 && x < side_size && y < side_size {
            let y_index = (y - 1) * side_size;
            let index = y_index + (x - 1);
            return index;
        } else {
            return usize::max_value();
        }
    }

    fn get_hundreds(input: isize) -> isize {
        let no_less_hundreds = input / 100;
        no_less_hundreds % 10
    }
}

impl Display for PowerGrid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let grid_size = self.grid.len();
        let side_size = (grid_size as f32).sqrt() as usize;
        let mut count = 1;
        for idx in 0..grid_size {
            let value = self.grid[idx];
            let formatted_value = format!("{:+2}", value);
            if count < side_size {
                write!(f, "{}", formatted_value)?;
                count += 1;
            } else {
                writeln!(f, "{}", formatted_value)?;
                count = 1;
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::power_grid::PowerGrid;
    #[test]
    fn generates_power_levels() {
        assert_eq!(PowerGrid::get_power_level(57, 122, 79), -5);
        assert_eq!(PowerGrid::get_power_level(39, 217, 196), 0);
        assert_eq!(PowerGrid::get_power_level(71, 101, 153), 4);
    }

    #[test]
    fn gets_correct_location1() {
        let power_grid = PowerGrid::init(18, 90000, false);
        let largest_grid = power_grid.get_most_powerful(None);
        assert_eq!(largest_grid, ((33, 45), 0));
    }

    #[test]
    fn gets_correct_location2() {
        let power_grid = PowerGrid::init(42, 90000, false);
        let largest_grid = power_grid.get_most_powerful(None);
        assert_eq!(largest_grid, ((21, 61), 0));
    }

    #[test]
    fn gets_correct_location_and_square_size() {
        let power_grid = PowerGrid::init(1723, 90000, true);
        let largest_grid = power_grid.get_most_powerful(None);
        assert_eq!(largest_grid, ((280, 218), 11));
    }
}

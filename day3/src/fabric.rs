pub struct Fabric {
    sheet: Vec<FabricCell>,
    row_width: usize,
}
impl Fabric {
    pub fn new() -> Fabric {
        Fabric {
            sheet: vec![FabricCell::new(); 1000000],
            row_width: 1000,
        }
    }

    pub fn add_claim(&mut self, rectangle: Rectangle) {
        for count in 0..rectangle.height {
            let (start, end) = self.get_row_indexes(rectangle.top + count, rectangle.side, rectangle.width);
            for sheet_index in start..end {
                self.sheet[sheet_index].add_claim();
            }
        }   
    }

    pub fn does_claim_overlap(&self, rectangle: Rectangle) -> bool {
        for count in 0..rectangle.height {
            let (start, end) = self.get_row_indexes(rectangle.top + count, rectangle.side, rectangle.width);
            let row_has_overlap = self.overlap_in_slice(start, end);

            if row_has_overlap {
                return true;
            }
        }

        false
    }

    pub fn overlap_claim_inches(&self) -> usize {
        let mut used_inches = 0;
        for cell in self.sheet.iter() {
            if cell.used_times > 1 {
                used_inches += 1;
            }
        }
        used_inches
    }

    pub fn get_row_indexes(&self, top: usize, side: usize, width: usize) -> (usize, usize) {
        let start_index = top * self.row_width + side;
        let end_index = start_index + width;
        (start_index, end_index)
    }

    fn overlap_in_slice(&self, start: usize, end: usize) -> bool {
        for index in start..end {
            if self.sheet[index].used_times > 1 {
                return true;
            }
        }

        return false;
    }
}

#[derive(Clone, Copy)]
struct FabricCell {
    used_times: usize,
}
impl FabricCell {
    pub fn new() -> FabricCell {
        FabricCell {
            used_times: 0,
        }
    }

    pub fn add_claim(&mut self) {
        self.used_times += 1;
    }
}

#[derive(Clone)]
pub struct Rectangle {
    pub top: usize,
    pub side: usize,
    pub width: usize,
    pub height: usize
}
impl Rectangle {
    pub fn new(top: usize, side: usize, width: usize, height: usize) -> Rectangle {
        Rectangle {
            top,
            side,
            width,
            height,
        }
    }

    pub fn from_str(rectangle_str: &str) -> Rectangle {
        // example from input: #1 @ 100,366: 24x27
        let data_points: Vec<&str> = rectangle_str.split("@ ").skip(1).take(1).collect();
        let split_data_points: Vec<&str> = data_points[0].split(": ").collect(); 
        let coordinates: Vec<&str> = split_data_points[0].split(",").collect();
        let size_strings: Vec<&str> = split_data_points[1].split("x").collect();
        let mut point_ints: Vec<usize> = vec![];
        for coordinate in coordinates.iter() {
            let parsed = usize::from_str_radix(coordinate, 10).expect("coordinate was not a valid integer");
            point_ints.push(parsed);
        }

        for size in size_strings {
            let parsed = usize::from_str_radix(size, 10).expect("coordinate was not a valid integer");
            point_ints.push(parsed);
        }

        Rectangle::new(point_ints[1], point_ints[0], point_ints[2], point_ints[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fabric_is_created() {
        let fabric = Fabric::new();

        assert!(fabric.row_width == 1000);
    }

    #[test]
    fn rectangle_is_created() {
        let rectangle = Rectangle::from_str("#1 @ 100,366: 24x27");

        assert_eq!(rectangle.top, 366);
        assert_eq!(rectangle.side, 100);
        assert_eq!(rectangle.width, 24);
        assert_eq!(rectangle.height, 27);
    }

    #[test]
    fn get_row_indexes_returns() {
        let fabric = Fabric::new();
        let rectangle = Rectangle::new(0, 0, 10, 1);
    
        assert_eq!(fabric.get_row_indexes(rectangle.top, rectangle.side, rectangle.width), (0, 10));
    }

    #[test]
    fn adds_used_cells() {
        let mut fabric = Fabric::new();
        let rectangle = Rectangle::new(0, 0, 10, 1);

        let (start, end) = fabric.get_row_indexes(rectangle.top, rectangle.side, rectangle.width);
        fabric.add_claim(rectangle);
    
        for index in start..end {
            assert_eq!(fabric.sheet[index].used_times, 1);
        } 
    }

    #[test]
    fn calculates_used_cells() {
        let mut fabric = Fabric::new();
        let rectangle = Rectangle::new(0, 0, 10, 1);

        fabric.add_claim(rectangle);
        let used_inches = fabric.overlap_claim_inches();
    
        assert_eq!(used_inches, 0);
    }

    #[test]
    fn calculates_overlap_cells() {
        let mut fabric = Fabric::new();
        let rectangle = Rectangle::new(0, 0, 10, 1);
        let overlap_rectangle = Rectangle::new(0, 5, 10, 2);

        fabric.add_claim(rectangle);
        fabric.add_claim(overlap_rectangle);
        let used_inches = fabric.overlap_claim_inches();
    
        assert_eq!(used_inches, 5);
    }


    #[test]
    fn calculates_overlap_from_rect() {
        let mut fabric = Fabric::new();
        let rectangle_one_str = "#129 @ 613,496: 14x29";
        let rectangle_two_str = "#128 @ 678,172: 24x15";
        let rectangle_three_str = "#127 @ 677,172: 10x10";
        let rectangle_one = Rectangle::from_str(rectangle_one_str);
        let rectangle_two = Rectangle::from_str(rectangle_two_str);
        let rectangle_three = Rectangle::from_str(rectangle_three_str);

        fabric.add_claim(rectangle_one.clone());
        fabric.add_claim(rectangle_two.clone());
        fabric.add_claim(rectangle_three.clone());
        let has_overlap_for_one = fabric.does_claim_overlap(rectangle_one);
        let has_overlap_for_two = fabric.does_claim_overlap(rectangle_two);
        let has_overlap_for_three = fabric.does_claim_overlap(rectangle_three);
    
        assert_eq!(has_overlap_for_one, false);
        assert_eq!(has_overlap_for_two, true);
        assert_eq!(has_overlap_for_three, true);
    }
}
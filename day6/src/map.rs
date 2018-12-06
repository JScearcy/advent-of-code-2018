use std::collections::HashMap;

pub struct Map {
    pub points: Vec<Coordinate>,
    parents: Vec<Coordinate>,
    size_x: isize,
    size_y: isize,
}
impl Map {
    pub fn new(parents: &Vec<(isize, isize)>) -> Map {
        let (size_x, size_y) = parents.iter().fold((-1, -1), |(curr_x, curr_y), parent| {
            let mut updated_x = curr_x;
            let mut updated_y = curr_y;
            if parent.0 > curr_x {
                updated_x = parent.0;
            }

            if parent.1 > curr_y {
                updated_y = parent.1;
            }

            (updated_x, updated_y)
        });

        println!("{}, {}", size_x, size_y);
        let mut points = vec![];
        let constant_size = 400;
        for y in 0..constant_size + 1 {
            for x in 0..constant_size + 1 {
                points.push(Coordinate::new(x, y, false, true, None, None, None));
            }
        }
        Map {
            points,
            size_x: constant_size,
            size_y: constant_size,
            parents: vec![],
        }
    }

    pub fn add_parent(&mut self, x: isize, y: isize, letter: char) {
        let new_parent = Coordinate::new(x, y, true, false, None, None, Some(letter));
        let index = Map::get_index_from_point((x, y), self.size_x, self.size_y);

        self.parents.push(new_parent.clone());
        self.points[index] = new_parent;
    }

    pub fn calculate_nearest_points(&self, max_distance: isize) {
        let mut nearest_points: Vec<(isize, isize)> = vec![];
        for point in &self.points {
            let mut distance_count = 0;
            for parent in &self.parents {
                if distance_count >= max_distance {
                    continue;
                }
                let distance = Map::get_distance_from_points(point.location, parent.location);
                distance_count += distance;
            }
            if distance_count >= max_distance {
                continue;
            } else {
                nearest_points.push(point.location);
            }
        }

        println!("{}", nearest_points.len());

        // println!("{:?}", nearest_points);
        // let mut count = 0;
        // for point in &self.points {
        //     count += 1;
        //     if nearest_points.contains(&point.location) {
        //         print!("#");
        //     } else {
        //         print!(".");
        //     }
        //     if count > self.size_x {
        //         count = 0;
        //         println!("");
        //     }
        // }
    }

    pub fn calculate_areas(
        &mut self,
        ring: isize,
        unchanged_parent_locations: &mut Vec<(isize, isize)>,
    ) {
        let negative_ring = -ring;
        let mut has_made_changes = false;
        for parent_coordinate in &mut self.parents {
            let mut parent_changed = false;
            let (parent_x, parent_y) = parent_coordinate.location;
            if !unchanged_parent_locations.contains(&(parent_x, parent_y)) {
                for y in negative_ring..ring + 1 {
                    for x in [negative_ring, ring].iter() {
                        let check_x = parent_x + x;
                        let check_y = parent_y + y;
                        let index = Map::get_index_from_point((check_x, check_y), self.size_x, self.size_y);
                        let parent_index =
                            Map::get_index_from_point((parent_x, parent_y), self.size_x, self.size_y);
                        if index != usize::max_value() {
                            let mut coord = &mut self.points[index];
                            let distance = Map::get_distance_from_points(
                                (parent_x, parent_y),
                                (check_x, check_y),
                            );
                            if !coord.is_taken {
                                coord.distance = Some(distance);
                                coord.is_taken = true;
                                coord.is_area = true;
                                coord.parent_index = Some(parent_index);
                                has_made_changes = true;
                                parent_changed = true;
                            } else {
                                if let Some(curr_distance) = coord.distance {
                                    if curr_distance > distance {
                                        coord.distance = Some(distance);
                                        coord.parent_index = Some(parent_index);
                                        has_made_changes = true;
                                        parent_changed = true;
                                    } else if curr_distance == distance {
                                        coord.parent_index = None;
                                        has_made_changes = true;
                                        parent_changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
                for y in [negative_ring, ring].iter() {
                    for x in (negative_ring + 1)..ring {
                        let check_x = parent_x + x;
                        let check_y = parent_y + y;
                        let index = Map::get_index_from_point((check_x, check_y), self.size_x, self.size_y);
                        let parent_index =
                            Map::get_index_from_point((parent_x, parent_y), self.size_x, self.size_y);
                        if index != usize::max_value() {
                            let mut coord = &mut self.points[index];
                            let distance = Map::get_distance_from_points(
                                (parent_x, parent_y),
                                (check_x, check_y),
                            );
                            if !coord.is_taken && coord.is_area {
                                coord.distance = Some(distance);
                                coord.is_taken = true;
                                coord.parent_index = Some(parent_index);
                                has_made_changes = true;
                                parent_changed = true;
                            } else if coord.is_area {
                                if let Some(curr_distance) = coord.distance {
                                    if curr_distance > distance {
                                        coord.distance = Some(distance);
                                        coord.parent_index = Some(parent_index);
                                        has_made_changes = true;
                                        parent_changed = true;
                                    } else if curr_distance == distance {
                                        coord.parent_index = None;
                                        has_made_changes = true;
                                        parent_changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
                if !parent_changed {
                    unchanged_parent_locations.push((parent_x, parent_y));
                }
            }
        }

        if has_made_changes {
            // println!(
            //     "Ring: {}, unchanged_parents: {}",
            //     ring,
            //     unchanged_parent_locations.len()
            // );
            self.calculate_areas(ring + 1, unchanged_parent_locations);
        } else {
            self.get_largest_area()
        }
    }

    fn get_largest_area(&self) {
        let infinite_parent_indexes: Vec<usize> = self.points.iter()
            .filter(|point| Map::is_point_on_edge(point.location.0, point.location.1, self.size_x, self.size_y))
            .map(|point| {
                if let Some(parent_index) = point.parent_index {
                    parent_index
                } else {
                    usize::max_value()
                }
            })
            .fold(vec![], |mut parent_indexes: Vec<usize>, index| {
                if !parent_indexes.contains(&index) && index != usize::max_value() {
                    parent_indexes.push(index);
                } 
                parent_indexes
            });
        
        let finite_parent_index_counts: HashMap<usize, usize> = self.points.iter()
            // points that are area points and area points only
            .filter(|point| point.is_area)
            .fold(HashMap::new(), |mut parent_count_map, area_point| {
                if let Some(parent_index) = area_point.parent_index {
                    if !infinite_parent_indexes.contains(&parent_index) {
                        let mut parent_index_count = 1;
                        if let Some(count) = parent_count_map.get(&parent_index) {
                            parent_index_count = *count + 1;    
                        }

                        parent_count_map.insert(parent_index, parent_index_count);
                    }
                }
                parent_count_map
            });
        
        let mut index_counts: Vec<_> = finite_parent_index_counts.iter().collect();
        index_counts.sort_by(|a, b| b.1.cmp(a.1));
        println!("{:?}", index_counts);
        
        // let highest_area = non_infinite_parents.iter()
        //     .fold(0, |curr_area: usize, parent| {
        //         let area = parent.area.len() + 1;
        //         if curr_area < area {
        //             area
        //         } else {
        //             curr_area
        //         }
        //     });
        // println!("{}", highest_area);
    }

    fn get_distance_from_points(point_one: (isize, isize), point_two: (isize, isize)) -> isize {
        let x_distance = (point_one.0 - point_two.0).abs();
        let y_distance = (point_one.1 - point_two.1).abs();

        x_distance + y_distance
    }

    fn get_index_from_point(point: (isize, isize), size_x: isize, size_y: isize) -> usize {
        let (x, y) = point;
        if x >= 0 && y >= 0 && x <= size_x && y <= size_y {
            let index = y * (size_x + 1) + x;
            if index >= 0 {
                usize::from_str_radix(&index.to_string(), 10).expect("!!!!")
            } else {
                usize::max_value()
            }
        } else {
            usize::max_value()
        }
    }

    fn is_point_on_edge(x: isize, y: isize, size_x: isize, size_y: isize) -> bool {
        x == 0 || y == 0 || x >= size_x || y >= size_y
    }

    fn get_point_from_index(&self, index: isize) -> (isize, isize) {
        let x = index % self.size_x;
        let y = (index - x) / self.size_y;

        (x, y)
    }

    pub fn print(&self) {
        let mut count = 0;
        for point in &self.points {
            let point_index = Map::get_index_from_point(point.location, self.size_x, self.size_y);
            let point_char = '.';
            if point.is_area && point.is_taken {
                if let Some(_) = point.parent_index {

                    print!("{}", '-');
                } else {
                    print!("{}", point_char);
                }
            } else if point.is_taken {
                print!("{}", point.letter) // Map::get_index_from_point(point.location, self.size_x, self.size_y));
            } else {
                print!("{}", point_char);
            }
            count += 1;
            if count > self.size_x {
                count = 0;
                println!("");
            }
            
        }
    } 
}

#[derive(Debug, Clone)]
pub struct Coordinate {
    location: (isize, isize),
    area: Vec<Coordinate>,
    is_taken: bool,
    is_area: bool,
    distance: Option<isize>,
    parent_index: Option<usize>,
    letter: char,
}
impl Coordinate {
    pub fn new(
        x: isize,
        y: isize,
        is_taken: bool,
        is_area: bool,
        distance: Option<isize>,
        parent_index: Option<usize>,
        letter: Option<char>,
    ) -> Coordinate {
        let mut coord_letter = '.';
        if let Some(coord_char) = letter {
            coord_letter = coord_char;
        }

        Coordinate {
            location: (x, y),
            area: vec![],
            is_taken,
            is_area,
            distance,
            parent_index,
            letter: coord_letter,
        }
    }

    pub fn add_child(&mut self, child: Coordinate) {
        if child.is_area {
            self.area.push(child);
        }
    }
}

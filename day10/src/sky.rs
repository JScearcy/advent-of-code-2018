use std::fs::File;
use std::io::Write;
use regex::Regex;
use std::fmt::{self, Display, Formatter};
use std::thread;
use std::time::Duration;

pub struct Sky {
    stars: Stars,
    x_min: isize,
    y_min: isize,
    x_max: isize,
    y_max: isize,
    message_area: usize,
    min_secs: usize,
}
impl Sky {
    pub fn new() -> Sky {
        Sky {
            stars: vec![],
            x_min: 0,
            y_min: 0,
            x_max: 0,
            y_max: 0,
            message_area: 0,
            min_secs: 0,
        }
    }

    pub fn create_the_stars_in_the_sky(&mut self, star_data: &str) {
        let star_data_regex: Regex = Regex::new("position=<\\s*(?P<x>-?\\d+),\\s+(?P<y>-?\\d+)> velocity=<\\s*(?P<xv>-?\\d+),\\s+(?P<yv>-?\\d+)>").unwrap();
        let star_data_lines = star_data.split("\n");
        for star_data_line in star_data_lines {
            let matches = star_data_regex.captures(star_data_line).unwrap();
            let x = isize::from_str_radix(&matches["x"], 10).expect("x wasn't a number");
            let y = isize::from_str_radix(&matches["y"], 10).expect("y wasn't a number");
            let x_v = isize::from_str_radix(&matches["xv"], 10).expect("x_v wasn't a number");
            let y_v = isize::from_str_radix(&matches["yv"], 10).expect("y_v wasn't a number");
            self.stars.push(Star::new(x, y, x_v, y_v));
            if x > self.x_max {
                self.x_max = x;
            }
            if x < self.x_min {
                self.x_min = x;
            }
            if y > self.y_max {
                self.y_max = y;
            }
            if y < self.y_min {
                self.y_min = y;
            }
        }
    }

    pub fn only_want_message(&mut self) {
        let mut area = ((self.x_min.abs() + self.x_max.abs()) * (self.y_min.abs() + self.y_max.abs())) as usize;
        let mut min_sec = 0;
        let mut tightest_star_area_map = vec![];
        for seconds in 1..10005 {
            self.move_the_stars_one();
            let (new_min_x, new_min_y, new_max_x, new_max_y, new_area) = self.get_stars_area();
            if new_area <= area {
                min_sec = seconds; 
                area = new_area;
                tightest_star_area_map = self.stars.clone();
                self.x_min = new_min_x;
                self.y_min = new_min_y;
                self.x_max = new_max_x;
                self.y_max = new_max_y;
            }
        }

        self.stars = tightest_star_area_map;
        self.message_area = area;
        self.min_secs = min_sec;
    }

    pub fn watch_the_stars_slow_for(&mut self, secs: usize) {
        for sec in 0..secs {
            println!("Sec: {}", sec + 1);
            self.move_the_stars_one();
            println!("{}", self);
            thread::sleep(Duration::from_millis(1000));
        }
    }

    pub fn write_to_file(&self, file: &mut File) {
        let write_string = format!("{}\nMin Secs: {}", self, self.min_secs);
        file.write_all(write_string.as_bytes()).expect("couldn't write to results file");
    }

    fn get_stars_area(&self) -> (isize, isize, isize, isize, usize) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for star in &self.stars {
            if star.x > max_x {
                max_x = star.x;
            }
            if star.x < min_x {
                min_x = star.x;
            }
            if star.y > max_y {
                max_y = star.y;
            }
            if star.y < min_y {
                min_y = star.y;
            }

        }
        let mut _width = 0;
        let mut _height = 0;
        if min_x.is_negative() && max_x.is_negative() {
            _width = max_x + min_x.abs();
        } else {
            _width = min_x.abs() + max_x.abs();
        }
        if min_y.is_negative() && max_y.is_negative() {
            _height = max_y + min_y.abs();
        } else {
            _height = min_y.abs() + max_y.abs();
        }

        (min_x, min_y, max_x, max_y, (_width * _height) as usize)
    }

    fn move_the_stars_one(&mut self) {
        for star in self.stars.iter_mut() {
            star.x = star.x + star.x_v;
            star.y = star.y + star.y_v;
        }
    }
}

#[derive(Debug, Clone)]
struct Star {
    x: isize,
    y: isize,
    x_v: isize,
    y_v: isize,
}
impl Star {
    pub fn new(x: isize, y: isize, x_v: isize, y_v: isize) -> Star {
        Star { x, y, x_v, y_v }
    }
}

type Stars = Vec<Star>;

impl Display for Sky {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // writeln!(f, "x_min: {}, y_min: {}, x_max: {}, y_max: {}", self.x_min, self.y_min, self.x_max, self.y_max)?;
        let width = self.x_min.abs() + self.x_max.abs() + 1;
        let height = self.y_min.abs() + self.y_max.abs() + 1;
        let area = (width * height) as usize;
        let mut full_sky: Vec<Option<usize>> = vec![None; area];
        for star in &self.stars {
            let index = get_index_for_star(star.x, star.y, self.x_min, self.y_min, width);
            // println!("Index: {}, Star: {:?}, y_min: {}, x_min: {}, width: {}, height: {}", index, star, self.y_min, self.x_min, width, height);
            if index < area - 1 {
                full_sky[index] = Some(index);
            }
        }

        let mut count = 1;
        for opt in full_sky.iter() {
            let mut char_to_write = '.';
            if opt.is_some() {
                char_to_write = '*';
            }

            if count == width as usize {
                writeln!(f, "{}", char_to_write)?;
                count = 1;
            } else {
                count += 1;
                write!(f, "{}", char_to_write)?;
            }
        }
        write!(f, "")
    }
}

fn get_index_for_star(x: isize, y: isize, x_min: isize, y_min: isize, width: isize) -> usize {
    let curr_y = y + y_min.abs();
    let curr_x = x + x_min.abs();
    ((curr_y * width) + curr_x) as usize
}

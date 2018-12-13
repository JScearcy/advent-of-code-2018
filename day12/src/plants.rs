use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct Planter<'a> {
    plants: VecDeque<PlantState>,
    plant_patterns: HashMap<String, char>,
    output_file: Option<&'a mut File>,
}

impl<'a> Planter<'a> {
    pub fn new(input: &str, file: Option<&'a mut File>) -> Planter<'a> {
        let input_lines = input.split('\n');
        let initial_state_regex = Regex::new("initial state:\\s+(?P<initial>[#.]+)\\s*").unwrap();
        let state_change_regex =
            Regex::new("(?P<state_change>[.#]+) => (?P<new_state>[.#])").unwrap();
        let mut plants: VecDeque<PlantState> = VecDeque::new();
        let mut plant_patterns = HashMap::new();
        for input_line in input_lines {
            if input_line.contains("initial") {
                let matches = initial_state_regex.captures(input_line).unwrap();
                let initial_state: Vec<&str> = matches["initial"].split("").collect();
                let initial_state_len = initial_state.len();
                plants = initial_state
                    .iter()
                    .skip(1)
                    .take(initial_state_len - 2)
                    .map(|str_char| {
                        let chars: Vec<PlantState> = str_char.chars().collect();
                        chars[0]
                    })
                    .collect();
            } else if input_line.contains("=>") {
                let matches = state_change_regex.captures(input_line).unwrap();
                let state_change = String::from(&matches["state_change"]);
                let new_state = String::from(&matches["new_state"]).chars().take(1).fold('.', |_, str_chars| str_chars);
                plant_patterns.insert(state_change, new_state);
            }
        }
        Planter {
            plants,
            plant_patterns,
            output_file: file,
        }
    }

    pub fn run_steps(&mut self, steps: usize) {
        let mut zero_index = 0;
        let mut stable_pattern = false;
        for step in 0..steps {
            if !stable_pattern {
                let (new_zero_index, stable) = self.change_step(zero_index);
                stable_pattern = stable;
                zero_index = new_zero_index;
            } else {
                zero_index -= (steps - step) as isize;
                break;
            }
        }
        let left_index = 0 - zero_index;
        let right_index = self.plants.len() as isize - (zero_index + 1);
        let mut total = 0;
        for (idx, pot) in self.plants.iter().enumerate() {
            if pot == &'#' {
                total += idx as isize - zero_index;
            }
        }

        if let Some(file) = &mut self.output_file {
            let data = format!("{{ \"zeroIndex\": {}, \"end\": {}, \"value\":{} }}", left_index, right_index, total);
            file.write_all(data.as_bytes()).expect("could not write file");
        } else {
            println!("zero index: {}, end: {}, value: {}", left_index, right_index, total);
        }
    }

    pub fn change_step(&mut self, zero_index: isize) -> (isize, bool) {
        let mut new_state: VecDeque<PlantState> = VecDeque::new();
        let mut state = self.plants.clone();
        state.push_front('.');
        state.push_front('.');
        state.push_back('.');
        state.push_back('.');
        for (idx, _) in state.iter().enumerate() {
            let mut plant_pattern: String = String::from("");
            if idx < 2 {
                let values: Vec<&PlantState> = state.iter().take(3 + idx).collect();
                for _ in 0..(2 - idx) {
                    plant_pattern.push_str(".");
                }
                let rest_pattern: String = values.iter().map(|str_char| *str_char).collect();
                plant_pattern.push_str(&rest_pattern);
            } else {
                let values: Vec<&PlantState> = state.iter().skip(idx - 2).take(5).collect();
                plant_pattern = values.iter().map(|str_char| *str_char).collect();
                while plant_pattern.len() < 5 {
                    plant_pattern.push_str(".");
                }
            }
            if self.plant_patterns.contains_key(&plant_pattern) {
                let new_state_char = *self.plant_patterns.get(&plant_pattern).unwrap();
                new_state.push_back(new_state_char);
            } else {
                panic!("Pattern not found in states");
            }
        }
        let mut zero_shift = 2;
        loop {
            let front = new_state.pop_front().unwrap();
            if front == '#' {
                new_state.push_front(front);
                break;
            } else {
                zero_shift -= 1;
            }
        }
        loop {
            let back = new_state.pop_back().unwrap();
            if back == '#' {
                new_state.push_back(back);
                break;
            }
        }
        let new_state_str: String = new_state.iter().collect(); 
        let old_state_str: String = self.plants.iter().collect();
        let stable = new_state_str == old_state_str;
        self.plants = new_state;

        (zero_index + zero_shift, stable)
    }
}

type PlantState = char;

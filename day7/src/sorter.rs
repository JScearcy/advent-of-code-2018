// use std::cell::Cell;
use regex::Regex;
use std::cell::RefCell;
// use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::rc::Rc;

pub struct SleighSorter {
    pub order: String,
    pub time: i32,
}
impl SleighSorter {
    pub fn new(lines: &str) -> SleighSorter {
        let line_regex = Regex::new(
            "Step (?P<less>[A-Z]) must be finished before step (?P<greater>[A-Z]) can begin.",
        )
        .unwrap();
        let mut steps: BTreeMap<char, Rc<RefCell<Step>>> = BTreeMap::new();
        let lines_split = lines.split('\n');
        for line in lines_split {
            let captured = line_regex.captures(line).unwrap();
            let less_name = get_char(&captured["less"]);
            let greater_name = get_char(&captured["greater"]);
            let contains_greater = steps.contains_key(&greater_name);
            let contains_less = steps.contains_key(&less_name);
            if contains_less && contains_greater {
                let less_step_rc = steps.get(&less_name).unwrap();
                let greater_step_rc = steps.get(&greater_name).unwrap();
                let mut less_step = less_step_rc.borrow_mut();
                
                less_step.add_child(Rc::clone(greater_step_rc));
            } else if contains_less {
                let greater_step = Step::new();
                let greater_step_rc = Rc::new(RefCell::new(greater_step));
                steps.insert(greater_name, Rc::clone(&greater_step_rc));

                let less_step_rc = steps.get(&less_name).unwrap();
                let mut less_step = less_step_rc.borrow_mut();
                less_step.add_child(Rc::clone(&greater_step_rc));
            } else {
                let mut less_step = Step::new();
                if steps.contains_key(&greater_name) {
                    let greater_step = steps.get(&greater_name).unwrap();
                    less_step.add_child(Rc::clone(greater_step));
                } else {
                    let greater_step = Step::new();
                    let greater_step_rc = Rc::new(RefCell::new(greater_step));
                    steps.insert(greater_name, Rc::clone(&greater_step_rc));
                    less_step.add_child(Rc::clone(&greater_step_rc));
                }
                let less_step_rc = Rc::new(RefCell::new(less_step));
                steps.insert(less_name, Rc::clone(&less_step_rc));
            }
        }
        // This commented code is for the part 1 solution - the order changes after updated for part 2
        // let mut completed: Vec<char> = vec![];
        // 'outer: loop {
        //     for (key, value) in steps.iter() {
        //         if Rc::strong_count(&value) == 1 && !value.borrow().is_complete {
        //             let mut completed_step = value.borrow_mut();
        //             completed_step.complete();
        //             completed.push(*key);

        //             continue 'outer;
        //         }
        //     }

        //     break;
        // }

        let mut completed: Vec<char> = vec![];
        let mut total_time = 0;
        let mut labor_pool: BTreeMap<i32, (char, Rc<RefCell<Step>>)> = BTreeMap::new();
        let workers = 5;
        'outer: loop {
            let mut first_complete = true;
            let mut remove_time: i32 = 0;
            let mut new_labor_pool = BTreeMap::new();
            for (time, (key, value)) in labor_pool.iter() {
                if first_complete {
                    remove_time = *time;
                    total_time += time;
                    completed.push(*key);
                    let mut completed_step = value.borrow_mut();
                    completed_step.complete();
                    first_complete = false;
                } else {
                    let new_time = time - remove_time;
                    new_labor_pool.insert(new_time, (*key, Rc::clone(value)));
                }
            }
            // drop the labor pool to free up the references for the logic below
            drop(labor_pool);

            for (key, value) in steps.iter() {
                if Rc::strong_count(&value) == 1 && !value.borrow().is_complete {
                    if new_labor_pool.keys().len() < workers {
                        let time_to_complete = get_time_from_char(key);
                        new_labor_pool.insert(time_to_complete, (*key, Rc::clone(&value)));
                    } else {
                        break;
                    }
                }
            }

            if new_labor_pool.keys().len() == 0 {
                break 'outer;
            } else {
                labor_pool = new_labor_pool;
            }
        }

        let order: String = completed.iter().collect();

        SleighSorter {
            order,
            time: total_time,
        }
    }
}

pub struct Step {
    children: Vec<Rc<RefCell<Step>>>,
    is_complete: bool,
}
impl Step {
    pub fn new() -> Step {
        Step {
            children: vec![],
            is_complete: false,
        }
    }

    pub fn add_child(&mut self, step: Rc<RefCell<Step>>) {
        self.children.push(step);
    }

    pub fn complete(&mut self) {
        self.children = vec![];
        self.is_complete = true;
    }
}

fn get_char(chars: &str) -> char {
    let chars_vec: Vec<char> = chars.chars().take(1).collect();
    let first_char = chars_vec[0].clone();
    first_char
}

fn get_time_from_char(char_candidate: &char) -> i32 {
    let char_code = *char_candidate as u8;
    char_code as i32 - 4
}

use std::collections::HashMap;

pub type Unit = char;
pub struct Polymer {
    state: Vec<Unit>,
}
impl Polymer {
    pub fn new() -> Polymer {
        Polymer { state: vec![] }
    }

    pub fn len(&self) -> usize {
        self.state.len()
    }

    pub fn add_chain(&mut self, polymer: &str) {
        self.state = polymer.chars().collect();
        let new_state = Polymer::units_after_destruction(&self.state);
        self.state = new_state;
    }

    pub fn best_chain_len(&self) -> usize {
        let mut tested_units: HashMap<Unit, usize> = HashMap::new();
        for unit in self.state.iter() {
            if !tested_units.contains_key(&unit) {
                let temp_chain: Vec<char> = self
                    .state
                    .iter()
                    .filter(|curr_char| !Polymer::units_equal(*curr_char, &unit, false))
                    .map(|curr_char| curr_char.clone())
                    .collect();
                let temp_state = Polymer::units_after_destruction(&temp_chain);
                tested_units.insert(unit.clone(), temp_state.len());
            }
        }

        let mut tested_units_len: Vec<&usize> = tested_units.values().collect();
        tested_units_len.sort();
        *tested_units_len[0]
    }

    fn units_after_destruction(chain: &Vec<char>) -> Vec<Unit> {
        let mut new_state = vec![];
        for unit in chain.iter() {
            let last_unit_opt = new_state.pop();
            if let Some(last_unit) = last_unit_opt {
                if !Polymer::units_equal(&unit, &last_unit, true) {
                    new_state.push(last_unit);
                    new_state.push(unit.clone());
                }
            } else {
                new_state.push(unit.clone());
            }
        }

        new_state
    }

    fn units_equal(unit_one: &Unit, unit_two: &Unit, match_case: bool) -> bool {
        let unit_one_case = unit_one.is_uppercase();
        let unit_two_case = unit_two.is_uppercase();
        let matcher = |matcher_unit_one: &Unit, matcher_unit_two: &Unit| {
            let mut unit_one_upper: Unit = '1';
            let mut unit_two_upper: Unit = '2';
            for unit_case in matcher_unit_one.to_uppercase() {
                unit_one_upper = unit_case;
            }
            for unit_case in matcher_unit_two.to_uppercase() {
                unit_two_upper = unit_case;
            }
            unit_one_upper == unit_two_upper
        };
        if match_case {
            if unit_one_case != unit_two_case {
                matcher(&unit_one, &unit_two)
            } else {
                false
            }
        } else {
            matcher(&unit_one, &unit_two)
        }
    }
}

pub struct TestKitchen {
    recipe_scores: Vec<u8>,
    first_elf_idx: usize,
    second_elf_idx: usize,
    target_idx: usize,
}

impl TestKitchen {
    pub fn new(target_idx: usize) -> TestKitchen {
        TestKitchen {
            recipe_scores: vec![3, 7],
            first_elf_idx: 0,
            second_elf_idx: 1,
            target_idx,
        }
    }

    pub fn find_best_ten_scores(&mut self) {
        loop {
            let first_score = self.recipe_scores[self.first_elf_idx];
            let second_score = self.recipe_scores[self.second_elf_idx];
            let combined_score = first_score + second_score;

            if combined_score >= 10 {
                let ones_score = combined_score % 10;
                let tens_score = combined_score / 10;
                self.recipe_scores.push(tens_score);
                self.recipe_scores.push(ones_score);
            } else {
                self.recipe_scores.push(combined_score);
            }

            self.first_elf_idx = (first_score as usize + 1 + self.first_elf_idx) % self.recipe_scores.len();
            self.second_elf_idx =
                (second_score as usize + 1 + self.second_elf_idx) % self.recipe_scores.len();

            if self.recipe_scores.len() >= self.target_idx + 10 {
                break;
            }
        }

        println!(
            "{:?}",
            &self.recipe_scores[self.target_idx..self.target_idx + 10]
        );
    }

    pub fn find_instance_of_score(&mut self) {
        let input_list = self.target_idx.to_string().chars().fold(vec![], |mut list, int_str| {
            list.push(u8::from_str_radix(&int_str.to_string(), 10).unwrap());
            list
        });
        let input_list_slice = &input_list[..];
        let input_list_len = input_list.len();
        let mut match_candidate: &[u8] = &input_list[1..];
        let _matches = match_candidate == input_list_slice;
        loop {
            let (new_first_idx, new_second_idx) = TestKitchen::run_one_loop(&mut self.recipe_scores, self.first_elf_idx, self.second_elf_idx);
            self.first_elf_idx = new_first_idx;
            self.second_elf_idx = new_second_idx;
            let recipe_len = self.recipe_scores.len();

            if recipe_len > input_list_len {
                match_candidate = &self.recipe_scores[recipe_len - input_list_len..recipe_len];

                if match_candidate == input_list_slice {
                    println!("{:?} appeared after {}", input_list, recipe_len - input_list_len);
                    break;
                }
                // test against the subset of the last items - 1 in case two recipe scores were added on
                match_candidate = &self.recipe_scores[recipe_len - (input_list_len + 1)..recipe_len - 1];

                if match_candidate == input_list_slice {
                    println!("{:?} appeared after {}", input_list, recipe_len - input_list_len - 1);
                    break;
                }
            }
        }
    }

    fn run_one_loop(recipe_scores: &mut Vec<u8>, first_idx: usize, second_idx: usize) -> (usize, usize) {
            let first_score = recipe_scores[first_idx];
            let second_score = recipe_scores[second_idx];
            let combined_score = first_score + second_score;

            if combined_score >= 10 {
                let tens_score = combined_score / 10;
                recipe_scores.push(tens_score);
            }
            recipe_scores.push(combined_score % 10);
            let recipe_len = recipe_scores.len();

            let new_first_idx = (first_score as usize + 1 + first_idx) % recipe_len;
            let new_second_idx =
                (second_score as usize + 1 + second_idx) % recipe_len;
        
        (new_first_idx, new_second_idx)
    }
}

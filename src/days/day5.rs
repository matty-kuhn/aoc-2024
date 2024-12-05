use std::collections::{HashMap, HashSet};

use super::{get_lines, Day};

pub struct Day5 {
    input: String,
}

impl Day5 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    /// returns (a map of page # : #s that must follow it, and a list of all the page updates)
    fn parse_input(&self) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
        let lines = get_lines(&self.input);
        let mut ret_map = HashMap::new();
        let mut ret_vec = Vec::new();
        let mut first_part = true;
        // handle first bit
        for line in lines {
            if line.trim().is_empty() {
                first_part = false;
                continue;
            }
            if first_part {
                let mut nums = line.split('|');
                let first = nums.next().unwrap().parse().unwrap();
                let second = nums.next().unwrap().parse().unwrap();
                ret_map
                    .entry(first)
                    .or_insert_with(HashSet::new)
                    .insert(second);
                continue;
            }
            let mut temp = Vec::new();
            for num in line.split(',') {
                temp.push(num.parse().unwrap());
            }
            ret_vec.push(temp);
        }
        (ret_map, ret_vec)
    }

    // returns set of invalid index pairs
    fn get_invalid_indices(
        update: &Vec<i32>,
        rules: &HashMap<i32, HashSet<i32>>,
    ) -> HashSet<(usize, usize)> {
        let mut seen = HashMap::new();
        let mut invalids = HashSet::new();
        for (idx, page) in update.iter().enumerate() {
            if let Some(parents) = rules.get(&page) {
                for parent in parents {
                    if seen.keys().collect::<HashSet<_>>().contains(&parent) {
                        invalids.insert((idx, seen[&parent]));
                    }
                }
            }
            seen.insert(page, idx);
        }
        invalids
    }

    fn check_valid(update: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
        let mut seen = HashSet::new();
        let mut valid = true;
        for page in update {
            // skip the rest of this update
            if !valid {
                break;
            }
            if let Some(parents) = rules.get(&page) {
                for parent in parents {
                    if seen.contains(parent) {
                        valid = false;
                    }
                }
            }
            seen.insert(page);
        }
        valid
    }
}

impl Day for Day5 {
    fn part1(&self) -> String {
        let (rules, updates) = self.parse_input();
        let mut sum = 0;
        for update in updates {
            if !Self::check_valid(&update, &rules) {
                continue;
            }
            // get middle num
            let mid_idx = (update.len() - 1) / 2;
            sum += update[mid_idx]
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        let (rules, mut updates) = self.parse_input();
        let mut sum = 0;
        for update in updates.iter_mut() {
            // skip valid lists
            if Self::check_valid(&update, &rules) {
                continue;
            }
            let mut invalid_pairs = Self::get_invalid_indices(&update, &rules);
            // reorder the list
            while !invalid_pairs.is_empty() {
                for (first, second) in &invalid_pairs {
                    let temp = update[*first];
                    update[*first] = update[*second];
                    update[*second] = temp;
                }
                invalid_pairs = Self::get_invalid_indices(&update, &rules);
            }

            // get middle num
            let mid_idx = (update.len() - 1) / 2;
            sum += update[mid_idx]
        }

        sum.to_string()
    }
}

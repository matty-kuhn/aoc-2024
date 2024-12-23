use std::collections::{HashMap, HashSet};

use super::Day;

pub struct Day22 {
    input: String,
}

impl Day22 {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl Day for Day22 {
    fn part1(&self) -> String {
        // step 1: secret = ((secret << 6) ^ secret) % 16777216
        // step 2: secret = ((secret >> 5) ^ secret) % 16777216
        // step 3: secret = ((secret  << 11) ^ secret) % 16777216
        self.input
            .lines()
            .fold(0, |acc, num| {
                let mut secret = num.parse::<usize>().unwrap();
                for _ in 0..2000 {
                    secret = ((secret << 6) ^ secret) % 16777216;
                    secret = ((secret >> 5) ^ secret) % 16777216;
                    secret = ((secret << 11) ^ secret) % 16777216;
                }
                acc + secret
            })
            .to_string()

        // todo!()
    }

    fn part2(&self) -> String {
        // step 1: secret = ((secret << 6) ^ secret) % 16777216
        // step 2: secret = ((secret >> 5) ^ secret) % 16777216
        // step 3: secret = ((secret  << 11) ^ secret) % 16777216
        self.input
            .lines()
            .fold(Vec::new(), |mut acc, num| {
                let mut secret = num.parse::<i64>().unwrap();
                let mut temp = Vec::new();
                temp.push(secret % 10);
                for _ in 0..2000 {
                    secret = ((secret << 6) ^ secret) % 16777216;
                    secret = ((secret >> 5) ^ secret) % 16777216;
                    secret = ((secret << 11) ^ secret) % 16777216;
                    temp.push(secret % 10);
                }
                acc.push(temp);
                acc
            })
            .iter()
            // find the value of each 4-day delta, for each monkey, calculated by
            // (day 1 - day 0) + (day 2 - day 1) + (day 3 - day 2) + (day 4 - day 3)
            // then find the delta that provides the highest overall value
            .fold(Vec::new(), |mut acc, monkey| {
                acc.push(monkey.windows(5).fold(Vec::new(), |mut deltas, window| {
                    // let mut this_deltas = Vec::new();
                    deltas.push((
                        window.windows(2).fold(Vec::new(), |mut deltas, nums| {
                            deltas.push(nums[1] - nums[0]);
                            deltas
                        }),
                        window[4],
                    ));
                    deltas
                }));
                acc
            })
            .iter()
            .fold(
                HashMap::new(),
                |mut delta_map: HashMap<Vec<i64>, i64>, monkey_deltas| {
                    monkey_deltas
                        .iter()
                        .fold(HashSet::new(), |mut this_deltas, delta| {
                            if !this_deltas.insert((delta.0).clone()) {
                                return this_deltas;
                            }
                            *delta_map.entry(delta.0.clone()).or_default() += delta.1 as i64;
                            this_deltas
                        });
                    delta_map
                },
            )
            .iter()
            .map(|(_, v)| *v)
            .max()
            .unwrap()
            .to_string()
    }
}

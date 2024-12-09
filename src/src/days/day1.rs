use std::{collections::HashMap, iter::zip};

use crate::days::get_lines;

use super::Day;

pub struct Day1 {
    input: String,
}

impl Day1 {
    pub fn new(input: String) -> Day1 {
        Day1 { input }
    }

    fn parse_nums(&self) -> (Vec<i32>, Vec<i32>) {
        let mut ret = (vec![], vec![]);
        let in_lines = get_lines(&self.input);
        let processed = in_lines
            .iter()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for line in processed {
            // gonna treat each one as only 2 numbers bc they said so
            ret.0.push(line[0].parse().unwrap());
            ret.1.push(line[1].parse().unwrap());
        }

        ret
    }
}

impl Day for Day1 {
    fn part1(&self) -> String {
        let (mut left, mut right) = self.parse_nums();
        // sort numbers
        left.sort();
        right.sort();
        // iter over both, summing the differences
        zip(left, right)
            .fold(0, |acc, (left, right)| acc + (left - right).abs())
            .to_string()
    }

    fn part2(&self) -> String {
        let (left, right) = self.parse_nums();
        let mut right_freqs = HashMap::new();
        for num in right {
            *right_freqs.entry(num).or_insert(0) += 1;
        }

        left.iter()
            .fold(0, |acc, num| acc + num * right_freqs.get(num).unwrap_or(&0))
            .to_string()
    }
}

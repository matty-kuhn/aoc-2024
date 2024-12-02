use super::Day;
use crate::days::get_lines;

pub struct Day2 {
    input: String,
}

impl Day2 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let lines = get_lines(&self.input);
        let processed = lines
            .iter()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<_>>();

        for line in processed {
            ret.push(line.iter().map(|item| item.parse().unwrap()).collect());
        }

        ret
    }

    fn test_safe(levels: &[i32]) -> bool {
        if levels.len() <= 1 {
            return true;
        }
        let mut increasing = false;

        for i in 1..levels.len() {
            if i == 1 {
                // check the first one increasing or decreasing
                if levels[i] < levels[i - 1] {
                    increasing = false;
                } else {
                    increasing = true;
                }
            }
            // not constantly inc/dec
            if (increasing && levels[i] < levels[i - 1])
                || (!increasing && levels[i] > levels[i - 1])
            {
                return false;
            }
            let diff = (levels[i] - levels[i - 1]).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
        }

        true
    }

    fn test_safe_tolerant(levels: &[i32]) -> bool {
        // test whole thing
        if Self::test_safe(&levels) {
            return true;
        }
        // test missing front
        if Self::test_safe(&levels[1..]) {
            return true;
        }
        // test missing back
        if Self::test_safe(&levels[..levels.len()-1]) {
            return true;
        }
        // test misisng middles
        for i in 1..levels.len() {
            if Self::test_safe(&[&levels[0..i - 1], &levels[i..]].concat()) {
                return true;
            }
        }
        // println!("unsafe! {:#?}", levels);
        false
    }
}

impl Day for Day2 {
    fn part1(&self) -> String {
        let input = self.parse_input();

        input
            .iter()
            .fold(0, |acc, levels| acc + Self::test_safe(&levels) as i32)
            .to_string()
    }

    fn part2(&self) -> String {
        let input = self.parse_input();

        input
            .iter()
            .fold(0, |acc, levels| {
                acc + Self::test_safe_tolerant(&levels) as i32
            })
            .to_string()
    }
}

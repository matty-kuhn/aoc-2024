use regex::Regex;

use super::{get_lines, Day};

pub struct Day3 {
    input: String,
}

impl Day3 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn extract_muls(&self) -> Vec<(i32, i32)> {
        // two options: use state machine or use regexes
        let re = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();
        let mut ret = vec![];
        let lines = get_lines(&self.input);
        for line in lines {
            let captures = re.captures_iter(line);
            for capture in captures {
                ret.push((
                    capture["first"].parse().unwrap(),
                    capture["second"].parse().unwrap(),
                ));
            }
        }
        ret
    }

    fn extract_muls_flip_flop(&self) -> Vec<(i32, i32)> {
        // two options: use state machine or use regexes
        let re = Regex::new(r"((?<do>do\(\))|(?<dont>don't\(\))|(?<mul>mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)))").unwrap();
        let mut ret = vec![];
        let lines = get_lines(&self.input);
        let mut on = true;
        for line in lines {
            let captures = re.captures_iter(line);
            for capture in captures {
                if capture.name("do").is_some() {
                    on = true;
                    continue;
                } else if capture.name("dont").is_some() {
                    on = false;
                    continue;
                } else if !on {
                    continue;
                }
                ret.push((
                    capture["first"].parse().unwrap(),
                    capture["second"].parse().unwrap(),
                ));
            }
        }
        ret
    }
}

impl Day for Day3 {
    fn part1(&self) -> String {
        self.extract_muls()
            .iter()
            .fold(0, |acc, (first, second)| acc + first * second)
            .to_string()
    }

    fn part2(&self) -> String {
        self.extract_muls_flip_flop()
            .iter()
            .fold(0, |acc, (first, second)| acc + first * second)
            .to_string()
    }
}

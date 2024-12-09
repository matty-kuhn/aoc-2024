use super::{get_lines, Day};
use itertools::{repeat_n, Itertools};

pub struct Day7 {
    input: String,
}

impl Day7 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> LinesData {
        let lines = get_lines(&self.input);
        lines.into()
    }

    fn check_valid1(target: usize, components: &[usize]) -> bool {
        for op in Operation::get_permutations(components.len() - 1) {
            let mut result = components[0];
            for (i, &comp) in components.iter().enumerate().skip(1) {
                match op[i - 1] {
                    Operation::Add => result += comp,
                    Operation::Multiply => result *= comp,
                }
            }
            if result == target {
                return true;
            }
        }
        false
    }

    fn check_valid2(target: usize, components: &[usize]) -> bool {
        for op in Operation2::get_permutations(components.len() - 1) {
            let mut result = components[0];
            for (i, &comp) in components.iter().enumerate().skip(1) {
                match op[i - 1] {
                    Operation2::Add => result += comp,
                    Operation2::Multiply => result *= comp,
                    Operation2::Concat => {
                        let mut res_str = result.to_string();
                        res_str.push_str(&comp.to_string());
                        result = res_str.parse().unwrap();
                    }
                }
            }
            if result == target {
                return true;
            }
        }
        false
    }
}

impl Day for Day7 {
    fn part1(&self) -> String {
        // naive: figure out all permutations involving * and +
        let data = self.parse_input();
        let mut sum = 0;
        for idx in 0..data.targets.len() {
            if Self::check_valid1(data.targets[idx], &data.components[idx]) {
                sum += data.targets[idx];
            }
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        // still naive: figure out all permutations involving * and + and concat
        let data = self.parse_input();
        let mut sum = 0;
        for idx in 0..data.targets.len() {
            if Self::check_valid2(data.targets[idx], &data.components[idx]) {
                sum += data.targets[idx];
            }
        }
        sum.to_string()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn get_permutations(num_ops: usize) -> Vec<Vec<&'static Operation>> {
        static OPS: [Operation; 2] = [Operation::Add, Operation::Multiply];

        repeat_n(&OPS, num_ops).multi_cartesian_product().collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Operation2 {
    Add,
    Multiply,
    Concat,
}

impl Operation2 {
    fn get_permutations(num_ops: usize) -> Vec<Vec<&'static Operation2>> {
        static OPS: [Operation2; 3] = [Operation2::Add, Operation2::Multiply, Operation2::Concat];

        repeat_n(&OPS, num_ops).multi_cartesian_product().collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct LinesData {
    targets: Vec<usize>,
    components: Vec<Vec<usize>>,
}

impl From<Vec<&str>> for LinesData {
    fn from(value: Vec<&str>) -> Self {
        // line format:
        // <target>: component1 component2 ... componentN

        let mut targets = Vec::new();
        let mut components = Vec::new();

        for line in value {
            let mut parts = line.split(": ");
            let target = parts.next().unwrap().parse().unwrap();
            let comps = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            targets.push(target);
            components.push(comps);
        }

        Self {
            targets,
            components,
        }
    }
}

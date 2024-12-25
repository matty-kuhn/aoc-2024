use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::Day;

pub struct Day24 {
    input: String,
}

impl Day24 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> (HashMap<String, u8>, HashMap<String, Ops>) {
        let (first, second) = self.input.split_once("\n\n").unwrap();
        let mut ret_map = HashMap::new();
        for line in first.lines() {
            let (name, wire) = line.split_once(": ").unwrap();
            ret_map.insert(name.to_string(), wire.parse().unwrap());
        }
        let mut ops = HashMap::new();
        for line in second.lines() {
            let splits = line.split(" ").collect::<Vec<_>>();
            match splits[1] {
                "XOR" => ops.insert(
                    splits[4].to_string(),
                    Ops::Xor(splits[0].to_string(), splits[2].to_string()),
                ),
                "AND" => ops.insert(
                    splits[4].to_string(),
                    Ops::And(splits[0].to_string(), splits[2].to_string()),
                ),
                "OR" => ops.insert(
                    splits[4].to_string(),
                    Ops::Or(splits[0].to_string(), splits[2].to_string()),
                ),
                _ => unreachable!(),
            };
        }

        (ret_map, ops)
    }
}

fn calc_one_wire(
    wires: &mut HashMap<String, u8>,
    ops: &HashMap<String, Ops>,
    target: &String,
) -> u8 {
    // we have already found it
    if let Some(val) = wires.get(target) {
        return *val;
    }
    //get the op to calculate this
    match ops.get(target).unwrap() {
        Ops::Xor(left, right) => {
            let left = calc_one_wire(wires, ops, left);
            let right = calc_one_wire(wires, ops, right);
            wires.insert(target.to_string(), left ^ right);
            left ^ right
        }
        Ops::And(left, right) => {
            let left = calc_one_wire(wires, ops, left);
            let right = calc_one_wire(wires, ops, right);
            wires.insert(target.to_string(), left & right);
            left & right
        }
        Ops::Or(left, right) => {
            let left = calc_one_wire(wires, ops, left);
            let right = calc_one_wire(wires, ops, right);
            wires.insert(target.to_string(), left | right);
            left | right
        }
    }
}

fn calc_all_wires(wires: &mut HashMap<String, u8>, ops: &HashMap<String, Ops>) {
    for (target, _) in ops {
        calc_one_wire(wires, &ops, target);
    }
}

impl Day for Day24 {
    fn part1(&self) -> String {
        let (mut wires, ops) = self.parse_input();
        calc_all_wires(&mut wires, &ops);
        let z_wires: Vec<(&String, &u8)> = wires
            .iter()
            .filter(|(wire, _)| wire.starts_with("z"))
            .sorted_by_key(|x| x.0)
            .collect();
        let mut sum: u64 = 0;
        let mut count = 0;
        for wire in z_wires {
            sum |= (*wire.1 as u64) << count as u64;
            count += 1;
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        let (_, outputs) = self.parse_input();
        // apparently don't need to fix, just find ones that break rules, according to reddit
        let mut bads = HashSet::new();
        for op in &outputs {
            let temp: Vec<_> = vec![op.1.inner().0, op.1.inner().1]
                .into_iter()
                .sorted()
                .collect();
            let (left, right) = (temp[0], temp[1]);
            if op.0.starts_with("z") && !op.0.ends_with("45") {
                if !matches!(op.1, Ops::Xor(_, _)) {
                    bads.insert(op.0);
                }
            } else if !(left.starts_with("x") || right.starts_with("y")) {
                if matches!(op.1, Ops::Xor(_, _)) {
                    bads.insert(op.0);
                }
            } else if left.starts_with("x") && right.starts_with("y")
                || left.starts_with("y") && right.starts_with("x")
            {
                if left.ends_with("00") && right.ends_with("00") {
                    continue;
                }

                let mut ops = vec![];
                for inner in &outputs {
                    if inner.1.inner().0 == op.0 || inner.1.inner().1 == op.0 {
                        ops.push(inner.1);
                    }
                }
                if matches!(op.1, Ops::Xor(_, _)) {
                    // need to find other xor
                    let mut found = false;
                    for inner in ops {
                        if matches!(inner, Ops::Xor(_, _)) {
                            found = true
                        }
                    }
                    if !found {
                        bads.insert(op.0);
                    }
                } else if matches!(op.1, Ops::And(_, _)) {
                    // need to find or
                    let mut found = false;
                    for inner in ops {
                        if matches!(inner, Ops::Or(_, _)) {
                            found = true
                        }
                    }
                    if !found {
                        bads.insert(op.0);
                    }
                }
            }
        }
        bads.iter().sorted().join(",")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Ops {
    Xor(String, String),
    And(String, String),
    Or(String, String),
}

impl Ops {
    fn inner(&self) -> (&String, &String) {
        match self {
            Ops::Xor(left, right) | Ops::And(left, right) | Ops::Or(left, right) => (left, right),
        }
    }
}

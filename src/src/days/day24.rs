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
    for (target, op) in ops {
        calc_one_wire(wires, &ops, target);
    }
}

impl Day for Day24 {
    fn part1(&self) -> String {
        let (mut wires, ops) = self.parse_input();
        calc_all_wires(&mut wires, &ops);
        let z_wires: Vec<(&String, &u8)> = wires
            .iter()
            .filter(|(wire, val)| wire.starts_with("z"))
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
        let (og_wires, outputs) = self.parse_input();
        // apparently don't need to fix, just find ones that break rules, according to reddit
        let mut bads = HashSet::new();

        for output in &outputs {
            if output.0.contains("z") {
                if output.0 != "z45" {
                    if !matches!(output.1, Ops::Xor(_, _)) {
                        bads.insert(output.0);
                    }
                }
            } else {
                match output.1 {
                    Ops::Xor(left, right) => {
                        if !(right.contains("x")
                            || left.contains("y")
                            || left.contains("x")
                            || right.contains("y"))
                        {
                            bads.insert(output.0);
                        }
                    }
                    Ops::And(left, right) | Ops::Or(left, right) => {}
                }
            }
            match output.1 {
                Ops::Xor(left, right) => {
                    if !(left.contains("x00") && right.contains("y00")) {
                        if left.contains("x") || right.contains("y") {
                            let mut found = false;
                            for gate in &outputs {
                                match gate.1 {
                                    Ops::Xor(left, right) => {
                                        if left == output.0 || right == output.0 {
                                            found = true;
                                        }
                                    }
                                    Ops::And(_, _) | Ops::Or(_, _) => {}
                                }
                            }
                            if !found {
                                println!("{:?}", output);
                                bads.insert(output.0);
                            }
                        }
                    }
                }
                Ops::And(left, right) => {
                    if !(left.contains("x00") && right.contains("y00")) {
                        let mut found = false;
                        for gate in &outputs {
                            match gate.1 {
                                Ops::Or(left, right) => {
                                    if left == output.0 || right == output.0 {
                                        found = true;
                                    }
                                }
                                Ops::And(_, _) | Ops::Xor(_, _) => {}
                            }
                        }
                        if !found {
                            // println!("{:?}", output);
                            bads.insert(output.0);
                        }
                    }
                }
                Ops::Or(left, right) => {}
            }
        }

        bads.iter().sorted().join(",").to_string()

        // let mut fagate0_followers = vec![];
        // let fagate3s: Vec<(&String, &Ops)> = outputs
        //     .iter()
        //     .filter(|(_, gate)| matches!(gate, Ops::Xor(_, _)))
        //     .filter(|(_, gate)| {
        //         !(match gate {
        //             Ops::Xor(left, right) | Ops::And(left, right) | Ops::Or(left, right) => {
        //                 left.contains("x") || right.contains("x")
        //             }
        //         })
        //     })
        //     .map(|(out, gate)| {
        //         if !out.contains("z") {
        //             bads.insert(out);
        //         }
        //
        //         (out, gate)
        //     })
        //     .collect();
        // let fagate0s: Vec<(&String, &Ops)> = outputs
        //     .iter()
        //     .filter(|(_, gate)| match gate {
        //         Ops::Xor(left, right) | Ops::And(left, right) | Ops::Or(left, right) => {
        //             left.contains("x") || right.contains("x")
        //         }
        //     })
        //     .filter(|(_, gate)| matches!(gate, Ops::Xor(_, _)))
        //     .map(|(out, gate)| {
        //         let (left, right) = match gate {
        //             Ops::Xor(left, right) | Ops::And(left, right) | Ops::Or(left, right) => {
        //                 (left, right)
        //             }
        //         };
        //         if left == "x00" || right == "x00" {
        //             if out != "z00" {
        //                 bads.insert(out);
        //             }
        //         } else if out == "z00" {
        //             bads.insert(out);
        //         }
        //         if out.contains("z") {
        //             bads.insert(out);
        //         }
        //         if !bads.contains(out) && out != "z00" {
        //             let has_this: Vec<(&String, &Ops)> = fagate3s
        //                 .clone()
        //                 .into_iter()
        //                 .filter(|(_, gate)| match gate {
        //                     Ops::Xor(left, right)
        //                     | Ops::And(left, right)
        //                     | Ops::Or(left, right) => left == out || right == out,
        //                 })
        //                 .collect();
        //             if has_this.len() == 0 {
        //                 fagate0_followers.push(out);
        //                 bads.insert(out);
        //             }
        //         }
        //         (out, gate)
        //     })
        //     .collect();
        //
        // let outs: Vec<(&String, &Ops)> = outputs
        //     .iter()
        //     .filter(|(out, _)| out.contains("z"))
        //     .map(|(out, gate)| {
        //         if out == "z45" {
        //             if !matches!(gate, Ops::Or(_, _)) {
        //                 bads.insert(out);
        //             }
        //         } else if !matches!(gate, Ops::Xor(_, _)) {
        //             bads.insert(out);
        //         }
        //         (out, gate)
        //     })
        //     .collect();
    }
}

fn check(wires: &HashMap<String, u8>) -> bool {
    let mut x = 0;
    let x_wires: Vec<(&String, &u8)> = wires
        .iter()
        .filter(|(wire, val)| wire.starts_with("x"))
        .sorted_by_key(|x| x.0)
        .collect();
    let mut y = 0;
    let y_wires: Vec<(&String, &u8)> = wires
        .iter()
        .filter(|(wire, val)| wire.starts_with("x"))
        .sorted_by_key(|x| x.0)
        .collect();

    let mut count = 0;
    for wire in x_wires {
        x |= (*wire.1 as u64) << count as u64;
        count += 1;
    }

    let mut count = 0;
    for wire in y_wires {
        y |= (*wire.1 as u64) << count as u64;
        count += 1;
    }
    let z_wires: Vec<(&String, &u8)> = wires
        .iter()
        .filter(|(wire, val)| wire.starts_with("z"))
        .sorted_by_key(|x| x.0)
        .collect();
    let mut z: u64 = 0;
    let mut count = 0;
    for wire in z_wires {
        z |= (*wire.1 as u64) << count as u64;
        count += 1;
    }
    println!("x:{x:048b}\ny:{y:048b}\ne:{:048b}", x + y);
    println!("a:{z:048b}");
    println!("d:{:048b}", (x + y) ^ z);

    x + y == z
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Ops {
    Xor(String, String),
    And(String, String),
    Or(String, String),
}

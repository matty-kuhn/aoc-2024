#![allow(unused)]

use super::Day;
use ilog::IntLog;
use std::collections::HashMap;

pub struct Day11 {
    input: String,
}

impl Day11 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn p2_thread(&self) -> String {
        let stones = self
            .input
            .split_whitespace()
            .map(|stone| Stone {
                val: stone.parse().unwrap(),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
        let stones = stones
            .into_par_iter()
            .enumerate()
            .map(|(idx, mut stone)| {
                for x in 0..75 {
                    stone.do_blink();
                    println!("stone {idx} done {x} blink");
                }
                stone.num_children()
            })
            .collect::<Vec<usize>>();
        stones.iter().sum::<usize>().to_string()
    }
}

impl Day for Day11 {
    fn part1(&self) -> String {
        // let mut stones = self
        //     .input
        //     .split_whitespace()
        //     .map(|stone| Stone {
        //         val: stone.parse().unwrap(),
        //         ..Default::default()
        //     })
        //     .collect::<Vec<_>>();
        //
        // for x in 0..25 {
        //     do_blink(&mut stones);
        //     println!("{x} blinks done");
        // }
        //
        // stones
        //     .iter()
        //     .fold(0, |acc, stone| acc + stone.num_children())
        //     .to_string()
        let mut stones_map = HashMap::new();

        let stones = self
            .input
            .split_whitespace()
            .map(|stone| stone.parse().unwrap())
            .collect::<Vec<_>>();
        for stone in stones {
            *stones_map.entry(stone).or_default() += 1;
        }

        for _ in 0..25 {
            map_step(&mut stones_map);
        }

        stones_map.values().sum::<usize>().to_string()
    }

    fn part2(&self) -> String {
        // self.p2_thread();
        let mut stones_map = HashMap::new();
        let stones = self
            .input
            .split_whitespace()
            .map(|stone| stone.parse().unwrap())
            .collect::<Vec<_>>();
        for stone in stones {
            *stones_map.entry(stone).or_default() += 1;
        }

        for _ in 0..75 {
            map_step(&mut stones_map);
        }

        stones_map.values().sum::<usize>().to_string()
    }
}

fn map_step(map: &mut HashMap<usize, usize>) {
    let mut temp: HashMap<usize, usize> = HashMap::new();
    for (key, val) in &mut *map {
        if key == &0 {
            *temp.entry(1).or_default() += *val;
        } else if (key.log10() + 1) % 2 == 0 {
            // split in half
            let inner = key.to_string();
            let upper = &inner[..inner.len() / 2];
            let lower = &inner[inner.len() / 2..];
            *temp.entry(upper.parse().unwrap()).or_default() += *val;
            *temp.entry(lower.parse().unwrap()).or_default() += *val;
        } else {
            *temp.entry(key * 2024).or_default() += *val;
        }
    }
    *map = temp;
}

#[derive(Default, Clone, Debug)]
struct ChildStone(Option<Box<Stone>>);

impl ChildStone {
    #[inline(always)]
    fn num_children(&self) -> usize {
        match self.0 {
            Some(ref stone) => stone.num_children(),
            None => 0,
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Stone {
    val: usize,
    left: ChildStone,
    right: ChildStone,
}

impl Stone {
    #[inline(always)]
    fn num_children(&self) -> usize {
        1 + self.left.num_children() + self.right.num_children()
    }

    fn do_blink(&mut self) {
        if let Some(left) = self.left.0.as_mut() {
            left.do_blink();
        }
        if let Some(right) = self.right.0.as_mut() {
            right.do_blink();
        }
        if self.val == 0 {
            self.val = 1;
        } else if (self.val.log10() + 1) % 2 == 0 {
            // split in half
            let temp = self.val.to_string();
            let upper = &temp[..temp.len() / 2];
            let lower = &temp[temp.len() / 2..];
            let left_len = self.left.num_children();
            let right_len = self.right.num_children();
            if left_len > right_len {
                self.right = ChildStone(Some(Box::new(Stone {
                    val: lower.parse().unwrap(),
                    right: self.right.clone(),
                    ..Default::default()
                })));
                self.val = upper.parse().unwrap();
            } else {
                self.left = ChildStone(Some(Box::new(Stone {
                    val: upper.parse().unwrap(),
                    left: self.left.clone(),
                    ..Default::default()
                })));
                self.val = lower.parse().unwrap();
            }
        } else {
            self.val *= 2024;
        }
    }
}

fn do_blink(stones: &mut Vec<Stone>) {
    for stone in stones {
        stone.do_blink();
    }
}

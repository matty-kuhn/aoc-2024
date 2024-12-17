use super::Day;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::{atomic::AtomicBool, Arc};

pub struct Day17 {
    input: String,
}

impl Day17 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> State {
        let mut ret = State {
            a: 0,
            b: 0,
            c: 0,
            output: vec![],
            instrs: vec![],
            ip: 0,
        };
        for line in self.input.lines() {
            if line.contains("A") {
                ret.a = line.split(':').nth(1).unwrap().trim().parse().unwrap();
            }
            if line.contains("B") {
                ret.b = line.split(':').nth(1).unwrap().trim().parse().unwrap();
            }
            if line.contains("C") {
                ret.c = line.split(':').nth(1).unwrap().trim().parse().unwrap();
            }
            if line.contains("P") {
                let mut split = line.split(':');
                split.next();
                let chars: Vec<_> = split.next().unwrap().trim().split(',').collect();
                for idx in 0..chars.len() {
                    ret.instrs.push(chars[idx].parse().unwrap())
                }
            }
        }

        ret
    }
}

impl Day for Day17 {
    fn part1(&self) -> String {
        let mut state = self.parse_input();
        state.run();
        state.output.iter().fold(String::new(), |mut acc, num| {
            acc.push_str(&format!("{num},"));
            acc
        })
    }

    fn part2(&self) -> String {
        let state = self.parse_input();
        find_replicate_rev(state).to_string()
    }
}

// super stolen, silly off by 2 lol
fn find_replicate_rev(state: State) -> u64 {
    dbg!(&state);
    let mut checks = vec![(0, 0)];
    let mut sols = vec![];
    while let Some((count, guess)) = checks.pop() {
        for idx in 0..8 {
            let mut temp = state.clone();
            temp.a = guess + idx;
            temp.ip = 0;
            temp.output = vec![];
            temp.run();
            // no output
            if temp.output.is_empty() {
                continue;
            }

            if state.instrs[(state.instrs.len() - 1) - count] == temp.output[0] {
                if count + 1 == temp.instrs.len() {
                    sols.push(guess + idx);
                    continue;
                }
                checks.push((count + 1, (8 * (guess + idx))));
            }
        }
    }
    dbg!(&sols);
    *sols.iter().min().unwrap()
}

#[derive(Debug, Clone)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    output: Vec<u64>,
    instrs: Vec<u64>,
    ip: usize,
}

impl State {
    fn run(&mut self) {
        while self.ip < self.instrs.len() - 1 {
            self.step(self.instrs[self.ip] as u8, self.instrs[self.ip + 1]);
        }
    }

    fn step(&mut self, inst: u8, op: u64) {
        match inst {
            0 => {
                self.a >>= self.literal_to_combo(op);
            }
            1 => {
                self.b ^= op;
            }
            2 => {
                self.b = self.literal_to_combo(op) % 8;
            }
            3 => {
                if self.a != 0 {
                    self.ip = op as usize;
                    return;
                }
            }
            4 => {
                self.b ^= self.c;
            }
            5 => {
                self.output.push(self.literal_to_combo(op) % 8);
            }
            6 => {
                self.b = self.a >> self.literal_to_combo(op);
            }
            7 => {
                self.c = self.a >> self.literal_to_combo(op);
            }
            _ => panic!("invalid inst"),
        }
        self.ip += 2;
    }

    fn literal_to_combo(&self, op: u64) -> u64 {
        if op <= 3 {
            op
        } else if op == 4 {
            self.a
        } else if op == 5 {
            self.b
        } else if op == 6 {
            self.c
        } else {
            panic!("invalid op")
        }
    }
}

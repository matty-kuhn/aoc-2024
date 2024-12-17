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
        // runner(state).to_string()
        find_replicate_rev(state).to_string()
    }
}

fn runner(state: State) -> u64 {
    let run = Arc::new(AtomicBool::new(true));
    rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build_global()
        .unwrap();
    // tried to do it right way and found this as too high, so lets just try counting down lol
    (0..((164278899142584 / 2) / 100_000_000))
        // .rev()
        // .collect::<Vec<_>>()
        .into_par_iter()
        .map(|init| {
            let init = init * 100_000_000;
            if run.load(std::sync::atomic::Ordering::SeqCst) == false {
                0
            } else {
                let state_clone = state.clone();
                let run_clone = run.clone();
                runner_inner(state_clone, run_clone, init)
            }
        })
        .filter(|x| *x > 0)
        .min()
        .unwrap()
}

fn runner_inner(mut state: State, run: Arc<AtomicBool>, init: u64) -> u64 {
    state.find_replicate_sync(init, run)
}

fn find_replicate_rev(mut state: State) -> u64 {
    let mut checks = vec![(0, 0)];
    while let Some((count, guess)) = checks.pop() {
        for idx in 0..8 {
            let mut temp = state.clone();
            temp.a = 8 * guess + idx;
            temp.ip = 0;
            temp.output = vec![];
            temp.run();
            // no output
            if temp.output.is_empty() {
                continue;
            }

            if state.instrs[(state.instrs.len() - 1) - count] == temp.output[0] {
                if count + 1 == temp.instrs.len() {
                    return guess;
                }
                checks.push((count + 1, (8 * guess + idx)));
            }
        }
    }
    0
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
    fn find_replicate_sync(&mut self, init: u64, run: Arc<AtomicBool>) -> u64 {
        let mut curr = init;
        self.a = curr;
        // while run.load(std::sync::atomic::Ordering::SeqCst) {
        loop {
            if self.replicate_inner() {
                println!("found!: {curr}");
                run.swap(false, std::sync::atomic::Ordering::SeqCst);
                return curr;
            }
            if curr == init + 100_000_000 {
                return 0;
            }
            curr += 1;
            self.a = curr;
            self.b = 0;
            self.c = 0;
            self.output = vec![];
            self.ip = 0;
        }
        // 0
    }

    fn replicate_inner(&mut self) -> bool {
        let mut curr = 0;
        while self.ip < self.instrs.len() - 1 {
            self.step(self.instrs[self.ip] as u8, self.instrs[self.ip + 1]);
            if self.output.len() > curr {
                if self.output.len() > self.instrs.len() {
                    return false;
                }
                if self.instrs[curr] != self.output[curr] {
                    return false;
                }
                curr += 1;
            }
        }
        if self.instrs != self.output {
            return false;
        }
        true
    }

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

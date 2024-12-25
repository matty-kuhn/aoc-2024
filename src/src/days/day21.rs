use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::{collections::HashMap, sync::LazyLock};

use super::Day;

pub struct Day21 {
    input: String,
}

impl Day21 {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl Day for Day21 {
    fn part1(&self) -> String {
        self.input
            .lines()
            .fold(0, |acc, input| {
                // get the first robot's traversal of keypad
                let num_keypad_paths = get_sequence_nums(input.to_owned());
                // println!("{num_keypad_paths:?}");

                // get the path to do that one
                let mut exp = num_keypad_paths.clone();
                for _ in 0..2 {
                    let min_len = Arc::new(AtomicUsize::new(usize::MAX));
                    exp = exp
                        .par_iter()
                        .map(|path| {
                            get_sequence_dirs(path.clone())
                                .iter()
                                .map(|p| (p.clone(), p.len()))
                                .collect::<Vec<(String, usize)>>()
                        })
                        .flatten()
                        .map(|(p, len)| {
                            if len < min_len.load(std::sync::atomic::Ordering::SeqCst) {
                                min_len.store(len, std::sync::atomic::Ordering::SeqCst);
                            }
                            (p, len)
                        })
                        .filter_map(|(p, len)| {
                            if len == min_len.load(std::sync::atomic::Ordering::SeqCst) {
                                Some(p)
                            } else {
                                None
                            }
                            // Some(p)
                        })
                        .collect();
                }
                acc + (score_code(input) * exp[0].len())
            })
            .to_string()
    }

    fn part2(&self) -> String {
        self.input
            .lines()
            .fold(0, |acc, input| {
                acc + get_sequence_nums(input.to_owned())
                    .into_iter()
                    .map(|p| shortest(&p))
                    .min()
                    .unwrap()
                    * score_code(input)
            })
            .to_string()
    }
}

fn shortest(input: &str) -> usize {
    // thx for cpoypasta reddit
    let from_to_cmds = [
        (('A', 'A'), "A"),
        (('A', '^'), "<A"),
        (('A', '>'), "vA"),
        (('A', '<'), "v<<A"),
        (('A', 'v'), "<vA"),
        (('^', 'A'), ">A"),
        (('^', '^'), "A"),
        (('^', '>'), "v>A"),
        (('^', '<'), "v<A"),
        (('^', 'v'), "vA"),
        (('v', 'A'), "^>A"),
        (('v', '^'), "^A"),
        (('v', '>'), ">A"),
        (('v', '<'), "<A"),
        (('v', 'v'), "A"),
        (('<', 'A'), ">>^A"),
        (('<', '^'), ">^A"),
        (('<', '>'), ">>A"),
        (('<', '<'), "A"),
        (('<', 'v'), ">A"),
        (('>', 'A'), "^A"),
        (('>', '^'), "<^A"),
        (('>', '>'), "A"),
        (('>', '<'), "<<A"),
        (('>', 'v'), "<A"),
    ]
    .into_iter()
    .map(|(k, v)| (k, v.to_string()))
    .collect::<HashMap<_, _>>();

    let mut cache = from_to_cmds
        .clone()
        .into_iter()
        .map(|(k, v)| ((k.0, k.1, 0), v.len()))
        .collect();

    let mut res = 0;
    let mut prev = 'A';
    for ch in input.chars() {
        res += shortest_chars(prev, ch, 24, &from_to_cmds, &mut cache);
        prev = ch;
    }
    res
}

fn shortest_chars(
    curr_pos: char,
    target: char,
    depth: usize,
    known_cmds: &HashMap<(char, char), String>,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if let Some(&res) = cache.get(&(curr_pos, target, depth)) {
        return res;
    }
    let path = known_cmds.get(&(curr_pos, target)).unwrap();
    let mut res = 0;
    let mut prev = 'A';
    for ch in path.chars() {
        res += shortest_chars(prev, ch, depth - 1, &known_cmds, cache);
        prev = ch;
    }
    cache.insert((curr_pos, target, depth), res);
    res
}

// Part 1 stuff

fn score_code(input: &str) -> usize {
    // first 3 chars of input are number
    input[..3].parse().unwrap()
}

#[memoize::memoize]
fn get_sequence_nums(input: String) -> Vec<String> {
    let mut res: Vec<String> = vec![String::new()];
    let input = format!("A{}", input);
    let mut curr = 0;
    let mut target = 1;

    loop {
        if target == input.len() || curr == input.len() - 1 {
            break;
        }
        let curr_ch = input.chars().nth(curr).unwrap();
        let target_ch = input.chars().nth(target).unwrap();
        let path = traverse_num_keypad(curr_ch, target_ch);
        let mut temp = Vec::new();
        for adds in path {
            for r in &res {
                temp.push(format!("{}{}", r, adds.iter().collect::<String>()));
            }
        }
        res = temp;
        curr = target;
        target += 1;
    }
    res
}

#[memoize::memoize]
fn get_sequence_dirs(input: String) -> Vec<String> {
    let mut res: Vec<String> = vec![String::new()];
    let input = format!("A{}", input);
    let mut curr = 0;
    let mut target = 1;

    loop {
        if target == input.len() || curr == input.len() - 1 {
            break;
        }
        let curr_ch = input.chars().nth(curr).unwrap();
        let target_ch = input.chars().nth(target).unwrap();
        let path = traverse_dir_keypad(curr_ch, target_ch);
        let mut temp = Vec::new();
        for adds in path {
            for r in &res {
                temp.push(format!("{}{}", r, adds.iter().collect::<String>()));
            }
        }
        res = temp;
        curr = target;
        target += 1;
    }
    res
}

#[memoize::memoize]
fn traverse_dir_keypad(curr_pos: char, target: char) -> Vec<Vec<char>> {
    static KEYPAD: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];
    static COORDS: LazyLock<HashMap<char, (usize, usize)>> = LazyLock::new(|| {
        let mut map = HashMap::new();
        for (i, row) in KEYPAD.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                map.insert(*c, (j, i));
            }
        }
        map
    });
    if curr_pos == target {
        return vec![vec!['A']];
    }
    let curr_pos_coords = COORDS.get(&curr_pos).unwrap().clone();
    let target_coords = COORDS.get(&target).unwrap();

    let mut temp = Vec::new();
    // if we need to move up, handle left/right first
    // if we need to move down, do that first
    if curr_pos_coords.1 > target_coords.1 {
        if curr_pos_coords.0 > target_coords.0 {
            // move left
            for _ in 0..(curr_pos_coords.0 - target_coords.0) {
                temp.push('<');
            }
        } else {
            // move right or stay in place
            for _ in 0..(target_coords.0 - curr_pos_coords.0) {
                temp.push('>');
            }
        }
        // finally move up
        temp.push('^');
    } else if curr_pos_coords.1 < target_coords.1 {
        // move down
        temp.push('v');
        // handle left/right
        if curr_pos_coords.0 > target_coords.0 {
            // move left
            for _ in 0..(curr_pos_coords.0 - target_coords.0) {
                temp.push('<');
            }
        } else {
            // move right or stay in place
            for _ in 0..(target_coords.0 - curr_pos_coords.0) {
                temp.push('>');
            }
        }
    } else {
        // just move left/right
        if curr_pos_coords.0 > target_coords.0 {
            // move left
            for _ in 0..(curr_pos_coords.0 - target_coords.0) {
                temp.push('<');
            }
        } else {
            // move right or stay in place
            for _ in 0..(target_coords.0 - curr_pos_coords.0) {
                temp.push('>');
            }
        }
    }
    let mut res: Vec<Vec<char>> = temp
        .clone()
        .into_iter()
        .permutations(temp.len())
        .unique()
        .filter(|walk| {
            let mut curr_coords = curr_pos_coords.clone();
            for dir in walk {
                match dir {
                    '^' => {
                        curr_coords.1 -= 1;
                        if curr_coords == *COORDS.get(&' ').unwrap() {
                            return false;
                        }
                    }
                    'v' => {
                        curr_coords.1 += 1;
                        if curr_coords == *COORDS.get(&' ').unwrap() {
                            return false;
                        }
                    }
                    '<' => {
                        curr_coords.0 -= 1;
                        if curr_coords == *COORDS.get(&' ').unwrap() {
                            return false;
                        }
                    }
                    '>' => {
                        curr_coords.0 += 1;
                        if curr_coords == *COORDS.get(&' ').unwrap() {
                            return false;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            true
        })
        .collect();
    for r in res.iter_mut() {
        r.push('A');
    }

    res
}

#[memoize::memoize]
fn traverse_num_keypad(curr_pos: char, target: char) -> Vec<Vec<char>> {
    static KEYPAD: [[char; 3]; 4] = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A'],
    ];
    static COORDS: LazyLock<HashMap<char, (usize, usize)>> = LazyLock::new(|| {
        let mut map = HashMap::new();
        for (i, row) in KEYPAD.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                map.insert(*c, (j, i));
            }
        }
        map
    });
    // println!("{} {}", curr_pos, target);
    if curr_pos == target {
        return vec![vec!['A']];
    }
    let curr_pos_coords = COORDS.get(&curr_pos).unwrap().clone();
    let target_coords = COORDS.get(&target).unwrap();

    let mut temp = Vec::new();
    // get a list of directions we need to move
    if curr_pos_coords.1 < target_coords.1 {
        if curr_pos_coords.0 > target_coords.0 {
            // move left
            for _ in 0..(curr_pos_coords.0 - target_coords.0) {
                temp.push('<');
            }
        } else {
            // move right or stay in place
            for _ in 0..(target_coords.0 - curr_pos_coords.0) {
                temp.push('>');
            }
        }
        // finally move down
        for _ in 0..(target_coords.1 - curr_pos_coords.1) {
            temp.push('v');
        }
    } else if curr_pos_coords.1 > target_coords.1 {
        // move up
        for _ in 0..(curr_pos_coords.1 - target_coords.1) {
            temp.push('^');
        }
        // handle left/right
        if curr_pos_coords.0 > target_coords.0 {
            // move left
            for _ in 0..(curr_pos_coords.0 - target_coords.0) {
                temp.push('<');
            }
        } else {
            // move right or stay in place
            for _ in 0..(target_coords.0 - curr_pos_coords.0) {
                temp.push('>');
            }
        }
    } else {
        // just move left/right
        if curr_pos_coords.0 > target_coords.0 {
            // move left
            for _ in 0..(curr_pos_coords.0 - target_coords.0) {
                temp.push('<');
            }
        } else {
            // move right or stay in place
            for _ in 0..(target_coords.0 - curr_pos_coords.0) {
                temp.push('>');
            }
        }
    }

    // get every possible ordering of directions
    let mut res: Vec<Vec<char>> = temp
        .clone()
        .into_iter()
        .permutations(temp.len())
        .unique()
        .filter(|walk| {
            let mut curr_coords = curr_pos_coords.clone();
            for dir in walk {
                match dir {
                    '^' => {
                        curr_coords.1 -= 1;
                        if curr_coords == *COORDS.get(&' ').unwrap() {
                            return false;
                        }
                    }
                    'v' => {
                        curr_coords.1 += 1;
                        if curr_coords == *COORDS.get(&' ').unwrap() {
                            return false;
                        }
                    }
                    '<' => {
                        curr_coords.0 -= 1;
                        if curr_coords == *COORDS.get(&' ').unwrap() {
                            return false;
                        }
                    }
                    '>' => {
                        curr_coords.0 += 1;
                        if curr_coords == *COORDS.get(&' ').unwrap() {
                            return false;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            true
        })
        .collect();
    for r in res.iter_mut() {
        r.push('A');
    }

    res
}

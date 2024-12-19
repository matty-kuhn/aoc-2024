use std::collections::{HashMap, HashSet};

use super::Day;

pub struct Day19 {
    input: String,
}

impl Day19 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> (HashSet<&str>, Vec<&str>, usize) {
        let mut nl = false;
        let mut ret_set = HashSet::new();
        let mut ret_vec = Vec::new();
        let mut max_needle = 0;
        for line in self.input.lines() {
            if nl {
                ret_vec.push(line);
                continue;
            }
            if line.trim().is_empty() {
                nl = true;
                continue;
            }
            ret_set = line
                .split(", ")
                .inspect(|n| {
                    if n.len() > max_needle {
                        max_needle = n.len();
                    }
                })
                .collect();
        }
        (ret_set, ret_vec, max_needle)
    }
}

impl Day for Day19 {
    fn part1(&self) -> String {
        let (needles, haystacks, max_needle) = self.parse_input();
        haystacks
            .into_iter()
            .filter(|hay| find_combo_bool(hay, &needles, 0, 1, max_needle))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let (needles, haystacks, _) = self.parse_input();
        haystacks
            .into_iter()
            .fold(0, |acc, hay| {
                acc + find_combo(hay, &needles, &mut HashMap::new())
            })
            .to_string()
    }
}
fn find_combo_bool(
    haystack: &str,
    needles: &HashSet<&str>,
    chunk_ptr: usize,
    chunk_len: usize,
    max_needle: usize,
) -> bool {
    // pretty much coin change: haystack is possible, if haystack[up to some x] is possible, and haystack[x to end] is in needles

    if chunk_len > max_needle {
        // curr chunk is too big to match a needle
        return false;
    }

    if chunk_len + chunk_ptr > haystack.len() {
        // next chunk would go off the end
        return false;
    }

    let Some(_) = needles.get(&haystack[chunk_ptr..(chunk_ptr + chunk_len)]) else {
        // this chunk cannot be represented by a needle, so try lengthen chunk by 1
        return find_combo_bool(haystack, needles, chunk_ptr, chunk_len + 1, max_needle);
    };

    // needle is good!
    // this was last match
    if chunk_ptr + chunk_len == haystack.len() {
        return true;
    }

    // need to consume more -> find next chunk, find longer chunk in this pos
    find_combo_bool(haystack, needles, chunk_ptr + chunk_len, 1, max_needle)
        || find_combo_bool(haystack, needles, chunk_ptr, chunk_len + 1, max_needle)
}

// returns true if possible, false otherwise
fn find_combo<'a>(
    haystack: &'a str,
    needles: &HashSet<&'a str>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    // shoutout reddit, i was off by one :(
    if haystack.is_empty() {
        return 1;
    }

    if let Some(memoed) = memo.get(&haystack) {
        return *memoed;
    }

    let mut res = 0;
    for towel in needles {
        if let Some(stripped) = haystack.strip_prefix(towel) {
            res += find_combo(stripped, needles, memo);
        }
    }
    memo.insert(haystack, res);
    res
}

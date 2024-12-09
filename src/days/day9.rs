use super::Day;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

pub struct Day9 {
    input: String,
}

impl Day9 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn checksum(blocks: &[Block]) -> usize {
        let mut sum = 0;
        for (idx, block) in blocks.iter().enumerate() {
            let Some(block_id) = block.get_id() else {
                continue;
            };
            sum += block_id * idx;
        }
        sum
    }
}

impl Day for Day9 {
    fn part1(&self) -> String {
        let mut nodes = Block::from_str(self.input.trim());
        let mut front = 0;
        let mut back = nodes.len() - 1;
        while front < back {
            // loop until we hit a .
            while let Block::File(_) = nodes[front] {
                front += 1;
            }
            // we have an empty node

            // loop back until we get an id
            loop {
                match nodes[back] {
                    Block::File(id) => {
                        nodes[front] = Block::File(id);
                        nodes[back] = Block::Empty;
                        back -= 1;
                        // dont add since that happens in the top loop
                        break;
                    }
                    Block::Empty => back -= 1,
                }
            }
        }
        Self::checksum(&nodes).to_string()
    }

    fn part2(&self) -> String {
        let (mut nodes, mut empties) = Block::from_str_with_empties(self.input.trim());
        let mut back = nodes.len() - 1;
        let mut visited = vec![false; nodes[back].get_id().unwrap() + 1];
        // don't need to check moving 0
        loop {
            // loop until we get to a number
            let curr_id = loop {
                match nodes[back].get_id() {
                    Some(id) => {
                        // already inserted this
                        if visited[id] {
                            back -= 1;
                            continue;
                        }
                        visited[id] = true;
                        break id;
                    }
                    None => back -= 1,
                }
            };
            if curr_id == 0 {
                // don't need to add these to chcksum bc they are all 0
                break;
            }
            let mut block_len = 0;
            // loop until we find a change
            loop {
                let Some(id) = nodes[back].get_id() else {
                    back += 1;
                    break;
                };
                if id != curr_id {
                    back += 1;
                    break;
                }
                block_len += 1;
                back -= 1;
            }

            // now back == first ind of last block
            // check if there is an empty block big enough for us - find leftmost
            let mut leftmost = usize::MAX;
            let mut leftmost_len = usize::MAX;
            // cheat that search space
            for len in block_len..=9 {
                let Some(indices) = empties.get_mut(&len) else {
                    continue;
                };
                let peek = indices.peek().unwrap().0;
                if peek > back {
                    // don't move right
                    continue;
                }
                if peek < leftmost {
                    leftmost = peek;
                    leftmost_len = len;
                }
            }
            if leftmost == usize::MAX {
                // not found
                continue;
            }
            let Some(indices) = empties.get_mut(&leftmost_len) else {
                // should never hit
                panic!("wahhhhh");
            };
            let ins_ind = indices.pop().unwrap();
            if indices.is_empty() {
                empties.remove(&leftmost_len);
            }

            // move this block to that empty chunk
            // 1. remove from back
            // 2. add to block
            for x in 0..block_len {
                nodes[ins_ind.0 + x] = Block::File(curr_id);
                nodes[back + x] = Block::Empty;
            }

            // need to move the ins_ind to its new len list
            if block_len < leftmost_len {
                empties
                    .entry(leftmost_len - block_len)
                    .or_insert_with(BinaryHeap::new)
                    .push(Reverse(ins_ind.0 + block_len));
            }
            // back -= 1;
        }

        Self::checksum(&nodes).to_string()
    }
}

#[derive(Debug)]
enum Block {
    File(usize),
    Empty,
}

impl Block {
    fn get_id(&self) -> Option<usize> {
        match self {
            Block::File(id) => Some(*id),
            Block::Empty => None,
        }
    }

    /// returns the disk as a vec of blocks, and a map of empty chunks, mapped as length : ordered list of start indices
    fn from_str(value: &str) -> Vec<Self> {
        static RADIX: u32 = 10;
        let mut ret = Vec::new();
        for (idx, ch) in value.chars().enumerate() {
            let id = idx / 2;
            let size = ch.to_digit(RADIX).unwrap() as usize;
            for _ in 0..size {
                if idx % 2 == 0 {
                    ret.push(Self::File(id));
                } else {
                    ret.push(Self::Empty);
                }
            }
        }
        ret
    }

    /// returns the disk as a vec of blocks, and a map of empty chunks, mapped as length : ordered list of start indices
    fn from_str_with_empties(
        value: &str,
    ) -> (Vec<Self>, HashMap<usize, BinaryHeap<Reverse<usize>>>) {
        static RADIX: u32 = 10;
        let mut ret = Vec::new();
        let mut ret_map = HashMap::new();
        for (idx, ch) in value.chars().enumerate() {
            let id = idx / 2;
            let size = ch.to_digit(RADIX).unwrap() as usize;
            let mut doing_empty = None;
            for _ in 0..size {
                if idx % 2 == 0 {
                    ret.push(Self::File(id));
                } else {
                    // start index
                    if doing_empty.is_none() {
                        doing_empty = Some(ret.len());
                    }
                    ret.push(Self::Empty);
                }
            }
            if let Some(empty_start_ind) = doing_empty {
                ret_map
                    .entry(size)
                    .or_insert_with(BinaryHeap::new)
                    .push(Reverse(empty_start_ind));
            }
        }
        (ret, ret_map)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::File(id) => write!(f, "{id}"),
            Block::Empty => write!(f, "."),
        }
    }
}

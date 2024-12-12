use std::collections::{HashMap, HashSet, VecDeque};

use super::Day;

macro_rules! peer_coords {
    ($row: ident, $col: ident) => {
        [
            ($row as isize - 1, $col as isize),
            ($row as isize, $col as isize - 1),
            ($row as isize, $col as isize + 1),
            ($row as isize + 1, $col as isize),
        ]
    };
}

fn get_peers(peers: &mut [bool; 4], row: usize, col: usize, grid: &[Vec<char>], peer_target: char) {
    let coords = peer_coords!(row, col);
    for (idx, coord) in coords.iter().enumerate() {
        if coord.0 < 0 || coord.0 as usize >= grid.len() {
            continue;
        }
        if coord.1 < 0 || coord.1 as usize >= grid[0].len() {
            continue;
        }
        peers[idx] = grid[coord.0 as usize][coord.1 as usize] == peer_target;
    }
}

pub struct Day12 {
    input: String,
}

impl Day12 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Vec<Vec<char>> {
        let mut ret_vec = Vec::new();
        for line in self.input.lines() {
            let mut line_vec = Vec::new();
            for c in line.chars() {
                line_vec.push(c);
            }
            ret_vec.push(line_vec);
        }

        ret_vec
    }
}

impl Day for Day12 {
    fn part1(&self) -> String {
        let grid = self.parse_input();
        let mut visited = HashSet::new();
        let mut prices = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if visited.contains(&(row, col)) {
                    continue;
                }
                let mut queue = VecDeque::new();
                queue.push_back((row, col));
                let mut area = 0;
                let mut sides = 0;
                while let Some((start_row, start_col)) = queue.pop_front() {
                    if visited.contains(&(start_row, start_col)) {
                        continue;
                    }
                    visited.insert((start_row, start_col));
                    area += 1;
                    let mut peers = [false; 4];
                    let peer_coords = peer_coords!(start_row, start_col);
                    get_peers(
                        &mut peers,
                        start_row,
                        start_col,
                        &grid,
                        grid[start_row][start_col],
                    );
                    for (idx, peer) in peer_coords.iter().enumerate() {
                        // num edges goes up by each false peer
                        if !peers[idx] {
                            sides += 1;
                            // this also means the plot doesn't go that way so no recursion
                            continue;
                        }
                        queue.push_back((peer.0 as usize, peer.1 as usize));
                    }
                }
                prices += area * sides;
            }
        }

        prices.to_string()
    }

    fn part2(&self) -> String {
        let grid = self.parse_input();
        let mut visited = HashSet::new();
        let mut prices = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if visited.contains(&(row, col)) {
                    continue;
                }
                let mut queue = VecDeque::new();
                queue.push_back((row, col));
                let mut area = 0;
                // need this to track perimeter, every direction change is + 1 side
                let mut perimiter = HashMap::new();
                while let Some((start_row, start_col)) = queue.pop_front() {
                    if visited.contains(&(start_row, start_col)) {
                        continue;
                    }
                    visited.insert((start_row, start_col));
                    area += 1;
                    let mut peers = [false; 4];
                    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
                    let peer_coords = peer_coords!(start_row, start_col);
                    get_peers(
                        &mut peers,
                        start_row,
                        start_col,
                        &grid,
                        grid[start_row][start_col],
                    );
                    for (idx, peer) in peer_coords.iter().enumerate() {
                        if peers[idx] {
                            queue.push_back((peer.0 as usize, peer.1 as usize));
                            continue;
                        }
                        // it is different, add to perimeter with the direction we used to get here
                        perimiter
                            .entry(DIRS[idx])
                            .or_insert_with(HashSet::new)
                            .insert((start_row, start_col));
                    }
                }
                let mut sides = 0;
                // we have build up whole perimiter
                for (_dir, starts) in perimiter.iter() {
                    let mut seen = HashSet::new();
                    for (start_row, start_col) in starts {
                        if seen.contains(&(*start_row, *start_col)) {
                            continue;
                        }
                        sides += 1;
                        let mut queue = VecDeque::new();
                        queue.push_back((*start_row, *start_col));
                        while let Some((row, col)) = queue.pop_front() {
                            if seen.contains(&(row, col)) {
                                continue;
                            }
                            seen.insert((row, col));
                            let mut peers = [false; 4];
                            let peer_coords = peer_coords!(row, col);
                            get_peers(&mut peers, row, col, &grid, grid[row][col]);
                            for peer in peer_coords.iter() {
                                if starts.contains(&(peer.0 as usize, peer.1 as usize)) {
                                    queue.push_back((peer.0 as usize, peer.1 as usize));
                                }
                            }
                        }
                    }
                }

                prices += area * sides;
            }
        }

        prices.to_string()
    }
}

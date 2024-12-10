use super::Day;
use std::{collections::HashSet, iter::zip};

pub struct Day10 {
    input: String,
}

impl Day10 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    // returns the grid of points, along with a list of all 0 locations
    fn parse_input(&self) -> (Vec<Vec<u8>>, PointData) {
        let mut ret_vec = Vec::new();
        let mut pts = PointData {
            row: vec![],
            col: vec![],
        };
        for (row_idx, line) in self.input.lines().enumerate() {
            let mut row = Vec::new();
            for (col_idx, ch) in line.chars().enumerate() {
                let num = ch.to_digit(10).unwrap() as u8;
                if num == 0 {
                    pts.row.push(row_idx);
                    pts.col.push(col_idx);
                }
                row.push(num);
            }
            ret_vec.push(row);
        }

        (ret_vec, pts)
    }
}

impl Day for Day10 {
    fn part1(&self) -> String {
        let (grid, zeroes) = self.parse_input();

        zip(zeroes.row, zeroes.col)
            .fold(0, |acc, (row, col)| {
                acc + dfs(row, col, &grid, 1, 9, &mut Some(&mut HashSet::new()))
            })
            .to_string()
    }

    fn part2(&self) -> String {
        let (grid, zeroes) = self.parse_input();

        zip(zeroes.row, zeroes.col)
            .fold(0, |acc, (row, col)| {
                acc + dfs(row, col, &grid, 1, 9, &mut None)
            })
            .to_string()
    }
}

struct PointData {
    row: Vec<usize>,
    col: Vec<usize>,
}

// peers is a slice of len 4
// X 0 X
// 1 X 2
// x 3 x
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

// peers is a slice of len 4
// X 0 X
// 1 X 2
// X 3 X
// all start false, return true if that index == the target
// could probably do this with cool bitshift stuff, would all fit in 1 u8
#[inline(always)]
fn get_peers(peers: &mut [bool; 4], row: usize, col: usize, grid: &[Vec<u8>], peer_target: u8) {
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

// returns count of target reachable from start of dfs
fn dfs(
    row: usize,
    col: usize,
    grid: &[Vec<u8>],
    peer_target: u8,
    target: u8,
    // super epic double solution whhaow
    visited: &mut Option<&mut HashSet<(usize, usize)>>,
) -> usize {
    let mut peers = [false; 4];
    let peer_coords = peer_coords!(row, col);
    get_peers(&mut peers, row, col, grid, peer_target);
    if peer_target == target {
        return peers
            .iter()
            .enumerate()
            .map(|(idx, b)| {
                (*b && visited
                    .as_mut()
                    .map(|v| v.insert((peer_coords[idx].0 as usize, peer_coords[idx].1 as usize)))
                    .unwrap_or(true)) as usize
            })
            .sum();
    }

    peers
        .iter()
        .enumerate()
        .map(|(idx, good)| {
            if !good {
                0
            } else {
                dfs(
                    peer_coords[idx].0 as usize,
                    peer_coords[idx].1 as usize,
                    grid,
                    peer_target + 1,
                    target,
                    visited,
                )
            }
        })
        .sum()
}

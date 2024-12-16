use super::Day;
use pathfinding::directed::astar::{astar, astar_bag_collect};
use std::collections::HashSet;

pub struct Day16 {
    input: String,
}

impl Day16 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Map {
        let mut grid = Vec::new();

        for line in self.input.lines() {
            let mut temp = Vec::new();
            for ch in line.chars() {
                temp.push(ch);
            }
            grid.push(temp);
        }

        let start = (1, grid.len() - 2);
        let end = (grid[0].len() - 2, 1);
        Map { grid, start, end }
    }
}

impl Day for Day16 {
    fn part1(&self) -> String {
        let map = self.parse_input();

        astar(
            &(map.start, Direction::Right),
            |p| {
                let mut succs = vec![];
                for dir in Direction::iter() {
                    let (new_x, new_y) = dir.mod_coords(&p.0);
                    if map.grid[new_y][new_x] != '#' {
                        let cost = if *dir == p.1 { 1 } else { 1001 };
                        succs.push((((new_x, new_y), *dir), cost));
                    }
                }
                succs
            },
            |p| map.end.0.abs_diff(p.0 .0) + map.end.1.abs_diff(p.0 .1),
            |p| p.0 == map.end,
        )
        .unwrap()
        .1
        .to_string()
    }

    fn part2(&self) -> String {
        let map = self.parse_input();

        astar_bag_collect(
            &(map.start, Direction::Right),
            |p| {
                let mut succs = vec![];
                for dir in Direction::iter() {
                    let (new_x, new_y) = dir.mod_coords(&p.0);
                    if map.grid[new_y][new_x] != '#' {
                        let cost = if *dir == p.1 { 1 } else { 1001 };
                        succs.push((((new_x, new_y), *dir), cost));
                    }
                }
                succs
            },
            |p| map.end.0.abs_diff(p.0 .0) + map.end.1.abs_diff(p.0 .1),
            |p| p.0 == map.end,
        )
        .unwrap()
        .0
        .iter()
        .fold(HashSet::new(), |mut acc, sol| {
            sol.iter().for_each(|coord| {
                acc.insert(coord.0);
            });
            acc
        })
        .len()
        .to_string()
    }
}

struct Map {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    // returns min cost
    // this works for literally every example input i can find, just not my actual input
    // fn bfs_to_end(&self) -> usize {
    // let mut search = VecDeque::new();
    // search.push_back((self.start, Direction::Right, 0));
    // let mut visited = HashSet::new();
    // let mut min_score = usize::MAX;
    //
    // while let Some(next_search) = search.pop_front() {
    //     if visited.contains(&(next_search.0, next_search.1)) {
    //         continue;
    //     }
    //     visited.insert((next_search.0, next_search.1));
    //     let (curr_x, curr_y) = next_search.0;
    //     let (next_x, next_y) = next_search.1.mod_coords((curr_x, curr_y));
    //     if self.grid[next_y][next_x] == 'E' {
    //         //reached end, +1 and check min score
    //         if next_search.2 + 1 < min_score {
    //             min_score = next_search.2 + 1;
    //         }
    //     } else if self.grid[next_y][next_x] == '.' {
    //         let add_search = ((next_x, next_y), next_search.1, next_search.2 + 1);
    //         if !visited.contains(&((next_x, next_y), next_search.1)) {
    //             search.push_back(add_search);
    //         }
    //     }
    //
    //     // now add on all next directions
    //     for dir in Direction::iter() {
    //         let (new_x, new_y) = dir.mod_coords((curr_x, curr_y));
    //         if self.grid[new_y][new_x] != '#' {
    //             let add_search = ((curr_x, curr_y), *dir, next_search.2 + 1000);
    //             if !visited.contains(&((curr_x, curr_y), *dir)) {
    //                 search.push_back(add_search);
    //             }
    //         }
    //     }
    // }
    //
    // min_score
    // }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn iter() -> impl Iterator<Item = &'static Self> {
        static VARS: [Direction; 4] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        VARS.iter()
    }
}

impl Direction {
    fn mod_coords(&self, coord: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (coord.0, coord.1 - 1),
            Direction::Down => (coord.0, coord.1 + 1),
            Direction::Left => (coord.0 - 1, coord.1),
            Direction::Right => (coord.0 + 1, coord.1),
        }
    }
}

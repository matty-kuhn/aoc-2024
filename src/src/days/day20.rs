use pathfinding::directed::astar::astar;

use super::Day;

pub struct Day20 {
    input: String,
}

impl Day20 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Map {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let grid: Vec<Vec<char>> = self
            .input
            .lines()
            .enumerate()
            .map(|(lidx, l)| {
                l.chars()
                    .enumerate()
                    .map(|(cidx, c)| {
                        if c == 'S' {
                            start = (cidx, lidx);
                            '.'
                        } else if c == 'E' {
                            end = (cidx, lidx);
                            '.'
                        } else {
                            c
                        }
                    })
                    .collect()
            })
            .collect();

        let path = astar(
            &start,
            |p| {
                let mut ret = vec![];
                for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let (x, y) = (p.0 as isize + dx, p.1 as isize + dy);
                    if x >= 0 && y >= 0 {
                        let (x, y) = (x as usize, y as usize);
                        if grid[y][x] == '.' {
                            ret.push(((x, y), 1 + p.1));
                        }
                    }
                }
                ret
            },
            |_| 1,
            |p| p == &end,
        )
        .unwrap()
        .0;

        Map { path }
    }
}

impl Day for Day20 {
    fn part1(&self) -> String {
        let map = self.parse_input();

        // walk path, and then find any point that can reach another point within 2 blocks
        // i misundertood q so needed to read reddit to figure out what it was asking
        map.path
            .iter()
            .enumerate()
            .map(|(time_to_pt, pt)|
            // check the entire rest of the path after this
            map.path.iter()
                .enumerate()
                .skip(time_to_pt + 1)
                .map(move |(time_to_pt2, pt2)| {
                let d = (pt.0 as isize - pt2.0 as isize).abs() + (pt.1 as isize - pt2.1 as isize).abs();
                if d <=2 && time_to_pt2 - time_to_pt - d as usize >= 100 {
                    1
                } else {0}
            })
        )
            .flatten()
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let map = self.parse_input();

        map.path
            .iter()
            .enumerate()
            .map(|(time_to_pt, pt)|
            // check the entire rest of the path after this
            map.path.iter()
                .enumerate()
                .skip(time_to_pt + 1)
                .map(move |(time_to_pt2, pt2)| {
                let d = (pt.0 as isize - pt2.0 as isize).abs() + (pt.1 as isize - pt2.1 as isize).abs();
                if d <=20 && time_to_pt2 - time_to_pt - d as usize >= 100 {
                    1
                } else {0}
            })
        )
            .flatten()
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug)]
struct Map {
    path: Vec<(usize, usize)>,
}

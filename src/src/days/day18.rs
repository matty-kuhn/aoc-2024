use std::fmt::Display;

use pathfinding::directed::astar::astar;
use regex::Regex;

use super::Day;

pub struct Day18 {
    input: String,
}

impl Day18 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Vec<(usize, usize)> {
        let re = Regex::new(r"([0-9]+),([0-9]+)").unwrap();
        let captures = re.captures_iter(&self.input);
        captures
            .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
            .collect()
    }

    // fn build_maze_partial(&self,
}

impl Day for Day18 {
    fn part1(&self) -> String {
        #[cfg(debug_assertions)]
        const MAX: usize = 6;
        #[cfg(not(debug_assertions))]
        const MAX: usize = 70;
        #[cfg(debug_assertions)]
        const STEPS: usize = 12;
        #[cfg(not(debug_assertions))]
        const STEPS: usize = 1024;
        let input = self.parse_input();
        let map = Map::build_maze::<MAX>(&input[..STEPS]);
        astar(
            &(0, 0),
            |p| map.get_successors(p),
            |p| MAX.abs_diff(p.0) + MAX.abs_diff(p.0),
            |p| p.0 == MAX && p.1 == MAX,
        )
        .unwrap()
        .1
        .to_string()
    }

    fn part2(&self) -> String {
        #[cfg(debug_assertions)]
        const MAX: usize = 6;
        #[cfg(not(debug_assertions))]
        const MAX: usize = 70;
        #[cfg(debug_assertions)]
        const STEPS: usize = 12;
        #[cfg(not(debug_assertions))]
        const STEPS: usize = 1024;
        let input = self.parse_input();
        let mut steps = STEPS;
        let mut map = Map::build_maze::<MAX>(&input[..steps]);
        while astar(
            &(0, 0),
            |p| map.get_successors(p),
            |p| MAX.abs_diff(p.0) + MAX.abs_diff(p.0),
            |p| p.0 == MAX && p.1 == MAX,
        )
        .is_some()
        {
            steps += 1;
            map.build_maze_partial(&input[STEPS..steps]);
        }
        format!("{:?}", input[steps - 1])
    }
}

struct Map {
    grid: Vec<Vec<char>>,
}

impl Map {
    fn build_maze<const MAX: usize>(input: &[(usize, usize)]) -> Self {
        let mut grid = vec![];
        for row in 0..=MAX {
            let mut temp = vec![];
            for col in 0..=MAX {
                if input.contains(&(col, row)) {
                    temp.push('#');
                } else {
                    temp.push('.');
                }
            }
            grid.push(temp);
        }
        Self { grid }
    }

    fn build_maze_partial(&mut self, input: &[(usize, usize)]) {
        for pt in input {
            self.grid[pt.1][pt.0] = '#';
        }
    }

    fn get_successors(&self, pt: &(usize, usize)) -> Vec<((usize, usize), usize)> {
        if pt.0 >= self.grid[0].len() || pt.1 >= self.grid.len() {
            return vec![];
        }
        let mut ret = vec![];
        if pt.0 > 0 && self.grid[pt.1][pt.0 - 1] != '#' {
            ret.push(((pt.0 - 1, pt.1), 1));
        }
        if pt.1 > 0 && self.grid[pt.1 - 1][pt.0] != '#' {
            ret.push(((pt.0, pt.1 - 1), 1));
        }
        if pt.0 < self.grid[0].len() - 1 && self.grid[pt.1][pt.0 + 1] != '#' {
            ret.push(((pt.0 + 1, pt.1), 1));
        }
        if pt.1 < self.grid.len() - 1 && self.grid[pt.1 + 1][pt.0] != '#' {
            ret.push(((pt.0, pt.1 + 1), 1));
        }
        ret
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{col}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

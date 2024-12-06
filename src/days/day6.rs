use std::collections::HashSet;

use super::{get_lines, Day};

pub struct Day6 {
    input: String,
}

impl Day6 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    /// returns (grid, startingposition, startingdirection)
    fn parse_input(&self) -> (Vec<Vec<GridTag>>, Point, Direction) {
        let lines = get_lines(&self.input);
        let mut ret = Vec::new();
        let mut coord = None;
        let mut direction = None;
        for (row_ind, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (column, chr) in line.chars().enumerate() {
                if chr == '#' {
                    row.push(GridTag::Obstacle);
                } else {
                    row.push(GridTag::Open);
                    if let Some(dir) = Direction::parse_from(chr) {
                        coord = Some(Point {
                            col: column as isize,
                            row: row_ind as isize,
                        });
                        direction = Some(dir);
                    }
                }
            }
            ret.push(row);
        }

        (ret, coord.unwrap(), direction.unwrap())
    }

    fn check_cycle(map: &[Vec<GridTag>], start_pos: &Point, start_dir: Direction) -> bool {
        // taking advantage of assumption that if we ever are in the same position/direction twice, we are cycling
        let row_max = map.len() as isize - 1;
        let col_max = map[0].len() as isize - 1;
        let mut curr_pos = *start_pos;
        let mut curr_dir = start_dir;

        let mut visited = HashSet::new();
        loop {
            if !visited.insert(((curr_pos.col, curr_pos.row), curr_dir)) {
                return true;
            }
            let temp = curr_pos;
            curr_dir.mod_coords(&mut curr_pos);
            if curr_pos.col > col_max
                || curr_pos.col < 0
                || curr_pos.row > row_max
                || curr_pos.row < 0
            {
                return false;
            }
            if matches!(
                map[curr_pos.row as usize][curr_pos.col as usize],
                GridTag::Open
            ) {
                continue;
            }
            // it is an Obstacle
            curr_dir.turn_right();
            // backtrack to where we just were, since we turn right in place
            curr_pos = temp;
        }
    }
    fn get_traversed_points(
        map: &[Vec<GridTag>],
        start_pos: &Point,
        start_dir: Direction,
    ) -> HashSet<Point> {
        let row_max = map.len() as isize - 1;
        let col_max = map[0].len() as isize - 1;
        let mut curr_pos = *start_pos;
        let mut curr_dir = start_dir;

        let mut visited = HashSet::new();
        loop {
            let temp = curr_pos;
            curr_dir.mod_coords(&mut curr_pos);
            if curr_pos.col > col_max
                || curr_pos.col < 0
                || curr_pos.row > row_max
                || curr_pos.row < 0
            {
                visited.insert(temp);
                return visited;
            }
            if matches!(
                map[curr_pos.row as usize][curr_pos.col as usize],
                GridTag::Open
            ) {
                visited.insert(temp);
                continue;
            }
            // it is an Obstacle
            curr_dir.turn_right();
            // backtrack to where we just were, since we turn right in place
            curr_pos = temp;
            visited.insert(temp);
        }
    }

    fn get_traversed_points_dir(
        map: &[Vec<GridTag>],
        start_pos: &Point,
        start_dir: Direction,
    ) -> HashSet<(Point, Direction)> {
        let row_max = map.len() as isize - 1;
        let col_max = map[0].len() as isize - 1;
        let mut curr_pos = *start_pos;
        let mut curr_dir = start_dir;

        let mut visited = HashSet::new();
        loop {
            let temp = curr_pos;
            curr_dir.mod_coords(&mut curr_pos);
            if curr_pos.col > col_max
                || curr_pos.col < 0
                || curr_pos.row > row_max
                || curr_pos.row < 0
            {
                visited.insert((temp, curr_dir));
                return visited;
            }
            if matches!(
                map[curr_pos.row as usize][curr_pos.col as usize],
                GridTag::Open
            ) {
                visited.insert((temp, curr_dir));
                continue;
            }
            // it is an Obstacle
            curr_dir.turn_right();
            // backtrack to where we just were, since we turn right in place
            curr_pos = temp;
            visited.insert((temp, curr_dir));
        }
    }
}

impl Day for Day6 {
    fn part1(&self) -> String {
        let (map, start_pos, start_dir) = self.parse_input();
        Self::get_traversed_points(&map, &start_pos, start_dir)
            .len()
            .to_string()
    }

    fn part2(&self) -> String {
        // options:
        // naive: just try every single space, and check which ones cause cycles
        //      this takes an insane amount of time (~1000x as long as pt 1) (3.8s)
        // opt 1: only modify a square if it touches the original path
        //      this halved the time (1.9s)
        // opt 2: only modify a square if it is in the path (matches visited pt + direction)
        //      abt halved again (750ms)
        let (map, start_pos, start_dir) = self.parse_input();
        let row_max = map.len() as isize - 1;
        let col_max = map[0].len() as isize - 1;
        let traversed_pts = Self::get_traversed_points_dir(&map, &start_pos, start_dir);

        let mut cycles = 0;
        let mut tries = HashSet::new();
        let mut map_clone = map.clone();
        for (pt, dir) in traversed_pts {
            let mut pt_cop = pt;
            dir.mod_coords(&mut pt_cop);
            if pt_cop.col > col_max || pt_cop.col < 0 || pt_cop.row > row_max || pt_cop.row < 0 {
                // exited the map, skip this check
                continue;
            }
            // already tried this spot
            if tries.contains(&pt_cop) {
                continue;
            }
            map_clone[pt_cop.row as usize][pt_cop.col as usize] = GridTag::Obstacle;
            if Self::check_cycle(&map_clone, &start_pos, start_dir) {
                cycles += 1;
            }
            map_clone[pt_cop.row as usize][pt_cop.col as usize] = GridTag::Open;
            tries.insert(pt_cop);
        }
        dbg!(tries.len());

        cycles.to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    // coord: (column, row)
    fn mod_coords(&self, coord: &mut Point) {
        match self {
            Self::Up => coord.row -= 1,
            Self::Down => coord.row += 1,
            Self::Left => coord.col -= 1,
            Self::Right => coord.col += 1,
        }
    }

    fn parse_from(value: char) -> Option<Self> {
        if value == '^' {
            Some(Self::Up)
        } else if value == '>' {
            Some(Self::Right)
        } else if value == '<' {
            Some(Self::Left)
        } else if value == 'v' {
            Some(Self::Down)
        } else {
            None
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        };
    }
}

#[derive(Clone, Copy, Debug)]
enum GridTag {
    Open,
    Obstacle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    col: isize,
    row: isize,
}

impl Point {
    fn touchers(&self) -> [Point; 4] {
        let Point { col, row } = self;
        [
            Point {
                col: col + 1,
                row: *row,
            },
            Point {
                col: col - 1,
                row: *row,
            },
            Point {
                col: *col,
                row: row + 1,
            },
            Point {
                col: *col,
                row: row - 1,
            },
        ]
    }
}

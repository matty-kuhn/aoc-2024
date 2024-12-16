use super::{get_lines, Day};
use core::fmt::{self, Display, Formatter};

pub struct Day15 {
    input: String,
}

impl Day15 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> (Map, Vec<Direction>) {
        let mut spaces = vec![];
        let mut directions = vec![];
        let mut curr_pos = (0, 0);
        let lines = get_lines(&self.input);
        let mut directions_start = 0;
        for (idx, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                directions_start = idx + 1;
                break;
            }
            let row = line
                .chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '#' => Space::Wall,
                    '.' => Space::Empty,
                    'O' => Space::Box,
                    '@' => {
                        curr_pos = (col_idx, idx);
                        Space::Empty
                    }
                    _ => panic!("Invalid character in input"),
                })
                .collect();
            spaces.push(row);
        }
        for line in lines[directions_start..].iter() {
            for c in line.chars() {
                directions.push(match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Invalid character in input"),
                })
            }
        }
        (Map { spaces, curr_pos }, directions)
    }

    fn double_input(&self) -> String {
        let mut ret = String::new();
        let lines = get_lines(&self.input);
        let mut directions_start = 0;
        for (idx, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                directions_start = idx + 1;
                break;
            }
            let mut new_line = String::new();
            for ch in line.chars() {
                match ch {
                    '#' => new_line.push_str("##"),
                    '.' => new_line.push_str(".."),
                    'O' => new_line.push_str("[]"),
                    '@' => new_line.push_str("@."),
                    _ => panic!("Invalid character in input"),
                }
            }
            ret.push_str(&new_line);
            ret.push_str("\n");
        }
        for line in lines[directions_start..].iter() {
            ret.push_str("\n");
            ret.push_str(line);
        }

        ret
    }

    fn parse_input_doubled(&self) -> (Map2, Vec<Direction>) {
        let mut spaces = vec![];
        let mut directions = vec![];
        let mut curr_pos = (0, 0);
        let doubled_input = self.double_input();
        let lines = get_lines(&doubled_input);
        let mut directions_start = 0;
        for (idx, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                directions_start = idx + 1;
                break;
            }
            let row = line
                .chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '#' => Space2::Wall,
                    '.' => Space2::Empty,
                    '[' => Space2::LeftBox,
                    ']' => Space2::RightBox,
                    '@' => {
                        curr_pos = (col_idx, idx);
                        Space2::Empty
                    }
                    _ => panic!("Invalid character in input"),
                })
                .collect();
            spaces.push(row);
        }
        for line in lines[directions_start..].iter() {
            for c in line.chars() {
                directions.push(match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Invalid character in input"),
                })
            }
        }
        (Map2 { spaces, curr_pos }, directions)
    }
}

impl Day for Day15 {
    fn part1(&self) -> String {
        let (mut map, directions) = self.parse_input();
        for direction in directions {
            map.process_move(direction);
        }
        map.get_all_gps().to_string()
    }

    fn part2(&self) -> String {
        let (mut map, directions) = self.parse_input_doubled();
        for direction in directions {
            map.process_move(direction);
        }
        map.get_all_gps().to_string()
    }
}

struct Map2 {
    spaces: Vec<Vec<Space2>>,
    curr_pos: (usize, usize),
}

impl Map2 {
    fn get_all_gps(&self) -> usize {
        let mut sum = 0;
        for (row_idx, row) in self.spaces.iter().enumerate() {
            for (col_idx, space) in row.iter().enumerate() {
                if space == &Space2::LeftBox {
                    sum += 100 * row_idx + col_idx;
                }
            }
        }
        sum
    }

    fn process_move(&mut self, dir: Direction) {
        let (new_x, new_y) = dir.mod_coords(self.curr_pos);
        // the easy cases first
        if self.spaces[new_y][new_x] == Space2::Wall {
            return;
        }
        if self.spaces[new_y][new_x] == Space2::Empty {
            self.curr_pos = (new_x, new_y);
            return;
        }
        // if there is a box, we make that square empty, and every square up to the next empty
        // becomes a box. if there is no empty, then we don't do any changes
        let mut copy = self.spaces.clone();
        let mut idx = 0;
        // let check = dir.mod_coords((new_x, new_y));
        let mut coords_to_move = vec![self.curr_pos];
        loop {
            if idx >= coords_to_move.len() {
                break;
            }
            let check = dir.mod_coords(coords_to_move[idx]);
            match self.spaces[check.1][check.0] {
                Space2::Empty => {}
                Space2::Wall => return,
                Space2::LeftBox => {
                    if !coords_to_move.contains(&(check.0, check.1)) {
                        coords_to_move.push((check.0, check.1));
                    }
                    if !coords_to_move.contains(&(check.0 + 1, check.1)) {
                        coords_to_move.push((check.0 + 1, check.1));
                    }
                }
                Space2::RightBox => {
                    if !coords_to_move.contains(&(check.0, check.1)) {
                        coords_to_move.push((check.0, check.1));
                    }
                    if !coords_to_move.contains(&(check.0 - 1, check.1)) {
                        coords_to_move.push((check.0 - 1, check.1));
                    }
                }
            }
            idx += 1;
        }
        for coord in &coords_to_move {
            copy[coord.1][coord.0] = Space2::Empty;
        }
        for coord in &coords_to_move {
            let prev = dir.mod_coords(*coord);
            copy[prev.1][prev.0] = self.spaces[coord.1][coord.0];
        }
        self.spaces = copy;
        self.curr_pos = (new_x, new_y);
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Space2 {
    Wall,
    Empty,
    LeftBox,
    RightBox,
}

struct Map {
    spaces: Vec<Vec<Space>>,
    curr_pos: (usize, usize),
}

impl Map {
    fn get_all_gps(&self) -> usize {
        let mut sum = 0;
        for (row_idx, row) in self.spaces.iter().enumerate() {
            for (col_idx, space) in row.iter().enumerate() {
                if space == &Space::Box {
                    sum += 100 * row_idx + col_idx;
                }
            }
        }
        sum
    }

    fn process_move(&mut self, dir: Direction) {
        let (new_x, new_y) = dir.mod_coords(self.curr_pos);
        // the easy cases first
        if self.spaces[new_y][new_x] == Space::Wall {
            return;
        }
        if self.spaces[new_y][new_x] == Space::Empty {
            self.curr_pos = (new_x, new_y);
            return;
        }
        // if there is a box, we make that square empty, and every square up to the next empty
        // becomes a box. if there is no empty, then we don't do any changes
        let set_empty = (new_x, new_y);
        let mut set_box = vec![];
        let mut check = dir.mod_coords((new_x, new_y));
        loop {
            match self.spaces[check.1][check.0] {
                Space::Empty => {
                    set_box.push(check);
                    break;
                }
                Space::Wall => return,
                Space::Box => {
                    set_box.push(check);
                    check = dir.mod_coords(check);
                }
            }
        }
        self.spaces[set_empty.1][set_empty.0] = Space::Empty;
        self.curr_pos = set_empty;
        for space in set_box {
            self.spaces[space.1][space.0] = Space::Box;
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Space {
    Wall,
    Empty,
    Box,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn mod_coords(&self, coords: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (coords.0, coords.1 - 1),
            Direction::Down => (coords.0, coords.1 + 1),
            Direction::Left => (coords.0 - 1, coords.1),
            Direction::Right => (coords.0 + 1, coords.1),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (row_idx, row) in self.spaces.iter().enumerate() {
            for (col_idx, space) in row.iter().enumerate() {
                if (col_idx, row_idx) == self.curr_pos {
                    write!(f, "@")?;
                    continue;
                }
                write!(
                    f,
                    "{}",
                    match space {
                        Space::Wall => '#',
                        Space::Empty => '.',
                        Space::Box => 'O',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Map2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (row_idx, row) in self.spaces.iter().enumerate() {
            for (col_idx, space) in row.iter().enumerate() {
                if (col_idx, row_idx) == self.curr_pos {
                    write!(f, "@")?;
                    continue;
                }
                write!(f, "{}", space)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Space2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space2::Wall => '#',
                Space2::Empty => '.',
                Space2::LeftBox => '[',
                Space2::RightBox => ']',
            }
        )
    }
}

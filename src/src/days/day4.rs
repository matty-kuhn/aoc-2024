use super::{get_lines, Day};

pub struct Day4 {
    input: String,
}

impl Day4 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    // O(n)
    fn process_input(&self) -> Vec<Vec<char>> {
        let lines = get_lines(&self.input);
        let mut ret = vec![];
        for line in lines {
            let mut temp = vec![];

            for ch in line.chars() {
                temp.push(ch);
            }
            ret.push(temp);
        }

        ret
    }

    /// return the indicies of all x's
    /// (column,row)
    // O(n)
    fn walk(input: &[Vec<char>], needle: &char) -> Vec<(isize, isize)> {
        let mut ret = vec![];
        for (row_ind, row) in input.iter().enumerate() {
            for (col_ind, col) in row.iter().enumerate() {
                if col == needle {
                    ret.push((col_ind as isize, row_ind as isize));
                }
            }
        }
        ret
    }

    /// actually find all the xmas
    fn find_xmas(input: &[Vec<char>], x_coords: &[(isize, isize)]) -> usize {
        let mut ret = 0;
        for x_coord in x_coords {
            for dir in Direction::iter() {
                ret += Self::process_single_xmas(input, x_coord, None, None, None, dir);
            }
        }
        ret
    }

    /// returns the number of xmases started by a single x
    fn process_single_xmas(
        input: &[Vec<char>],
        x_coord: &(isize, isize),
        m_coord: Option<&(isize, isize)>,
        a_coord: Option<&(isize, isize)>,
        s_coord: Option<&(isize, isize)>,
        curr_direction: Direction,
    ) -> usize {
        let row_max = input.len() as isize - 1;
        let col_max = input[0].len() as isize - 1;

        // handle no m
        if m_coord.is_none() {
            let modded_coords = curr_direction.mod_coords(x_coord);
            if modded_coords.0 < 0
                || modded_coords.1 < 0
                || modded_coords.0 > col_max
                || modded_coords.1 > row_max
            {
                return 0;
            }
            if input[modded_coords.1 as usize][modded_coords.0 as usize] != 'M' {
                return 0;
            }
            return Self::process_single_xmas(
                input,
                x_coord,
                Some(&modded_coords),
                None,
                None,
                curr_direction,
            );
        }

        // handle no a
        if a_coord.is_none() {
            let Some(m_coord_inner) = m_coord else {
                // no more letters in this direction
                return 0;
            };
            let modded_coords = curr_direction.mod_coords(m_coord_inner);
            if modded_coords.0 < 0
                || modded_coords.1 < 0
                || modded_coords.0 > col_max
                || modded_coords.1 > row_max
            {
                return 0;
            }
            if input[modded_coords.1 as usize][modded_coords.0 as usize] != 'A' {
                return 0;
            }
            return Self::process_single_xmas(
                input,
                x_coord,
                m_coord,
                Some(&modded_coords),
                None,
                curr_direction,
            );
        }

        // handle no s
        if s_coord.is_none() {
            let Some(a_coord_inner) = a_coord else {
                // no more letters in this direction
                return 0;
            };
            let modded_coords = curr_direction.mod_coords(a_coord_inner);
            if modded_coords.0 < 0
                || modded_coords.1 < 0
                || modded_coords.0 > col_max
                || modded_coords.1 > row_max
            {
                return 0;
            }
            if input[modded_coords.1 as usize][modded_coords.0 as usize] != 'S' {
                return 0;
            }
            return 1;
        }

        0
    }

    fn find_x_mas(input: &[Vec<char>], a_coords: &[(isize, isize)]) -> usize {
        let row_max = input.len() as isize - 1;
        let col_max = input[0].len() as isize - 1;
        let mut ret = 0;
        for a_coord in a_coords {
            // need to find bot MAS
            // get the 4 corners, in opposite pairs
            let top_left = Direction::DiagLeftUp.mod_coords(a_coord);
            if top_left.0 < 0 || top_left.1 < 0 || top_left.0 > col_max || top_left.1 > row_max {
                continue;
            }
            let bottom_right = Direction::DiagRightDown.mod_coords(a_coord);
            if bottom_right.0 < 0
                || bottom_right.1 < 0
                || bottom_right.0 > col_max
                || bottom_right.1 > row_max
            {
                continue;
            }
            let top_right = Direction::DiagRightUp.mod_coords(a_coord);
            if top_right.0 < 0 || top_right.1 < 0 || top_right.0 > col_max || top_right.1 > row_max
            {
                continue;
            }
            let bottom_left = Direction::DiagLeftDown.mod_coords(a_coord);
            if bottom_left.0 < 0
                || bottom_left.1 < 0
                || bottom_left.0 > col_max
                || bottom_left.1 > row_max
            {
                continue;
            }
            if !((input[top_right.1 as usize][top_right.0 as usize] == 'M'
                && input[bottom_left.1 as usize][bottom_left.0 as usize] == 'S')
                || (input[top_right.1 as usize][top_right.0 as usize] == 'S'
                    && input[bottom_left.1 as usize][bottom_left.0 as usize] == 'M'))
            {
                // missed first diag
                continue;
            }
            if !((input[top_left.1 as usize][top_left.0 as usize] == 'M'
                && input[bottom_right.1 as usize][bottom_right.0 as usize] == 'S')
                || (input[top_left.1 as usize][top_left.0 as usize] == 'S'
                    && input[bottom_right.1 as usize][bottom_right.0 as usize] == 'M'))
            {
                // missed second diag
                continue;
            }

            ret += 1;
        }
        ret
    }
}

impl Day for Day4 {
    fn part1(&self) -> String {
        let grid = self.process_input();
        let x_coords = Self::walk(&grid, &'X');
        Self::find_xmas(&grid, &x_coords).to_string()
    }

    fn part2(&self) -> String {
        let grid = self.process_input();
        let a_coords = Self::walk(&grid, &'A');
        Self::find_x_mas(&grid, &a_coords).to_string()
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagRightUp,
    DiagRightDown,
    DiagLeftUp,
    DiagLeftDown,
}

impl Direction {
    // coord: (column, row)
    fn mod_coords(&self, coord: &(isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (coord.0, coord.1 - 1),
            Self::Down => (coord.0, coord.1 + 1),
            Self::Left => (coord.0 - 1, coord.1),
            Self::Right => (coord.0 + 1, coord.1),
            Self::DiagRightUp => (coord.0 + 1, coord.1 - 1),
            Self::DiagRightDown => (coord.0 + 1, coord.1 + 1),
            Self::DiagLeftUp => (coord.0 - 1, coord.1 - 1),
            Self::DiagLeftDown => (coord.0 - 1, coord.1 + 1),
        }
    }

    fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::DiagRightUp,
            Self::DiagRightDown,
            Self::DiagLeftUp,
            Self::DiagLeftDown,
        ]
        .iter()
        .copied()
    }
}

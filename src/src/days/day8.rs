use std::collections::{HashMap, HashSet};

use super::{get_lines, Day};

pub struct Day8 {
    input: String,
}

impl Day8 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Vec<Vec<char>> {
        get_lines(&self.input)
            .iter()
            .map(|line| line.chars().collect())
            .collect()
    }
}

impl Day for Day8 {
    fn part1(&self) -> String {
        let map = self.parse_input();
        let row_max = map.len();
        let col_max = map[0].len();

        let mut antenna_locs = HashMap::new();
        let mut antinode_locs = HashSet::new();

        for (row_idx, row) in map.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell != '.' {
                    let prev_coords = antenna_locs.entry(cell).or_insert_with(Vec::new);
                    prev_coords.push((row_idx, col_idx));
                    let curr_coords_idx = prev_coords.len() - 1;
                    if curr_coords_idx == 0 {
                        // first antenna
                        continue;
                    }

                    for (idx, (pre_row, pre_col)) in prev_coords.iter().enumerate() {
                        // this is the one we just added
                        if idx == curr_coords_idx {
                            continue;
                        }
                        let row_diff = (row_idx as i32 - *pre_row as i32).abs();
                        let antinode1_row = row_idx as i32 + row_diff;
                        let antinode2_row = *pre_row as i32 - row_diff;
                        let col_diff = col_idx as i32 - *pre_col as i32;
                        let antinode1_col = col_idx as i32 + col_diff;
                        let antinode2_col = *pre_col as i32 - col_diff;

                        if antinode1_row >= 0
                            && antinode1_row < row_max as i32
                            && antinode1_col >= 0
                            && antinode1_col < col_max as i32
                        {
                            antinode_locs.insert((antinode1_row as usize, antinode1_col as usize));
                        }
                        if antinode2_row >= 0
                            && antinode2_row < row_max as i32
                            && antinode2_col >= 0
                            && antinode2_col < col_max as i32
                        {
                            antinode_locs.insert((antinode2_row as usize, antinode2_col as usize));
                        }
                    }
                }
            }
        }

        antinode_locs.len().to_string()
    }

    fn part2(&self) -> String {
        let map = self.parse_input();
        let row_max = map.len();
        let col_max = map[0].len();

        let mut antenna_locs = HashMap::new();
        let mut antinode_locs = HashSet::new();

        for (row_idx, row) in map.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell != '.' {
                    let prev_coords = antenna_locs.entry(cell).or_insert_with(Vec::new);
                    prev_coords.push((row_idx, col_idx));
                    let curr_coords_idx = prev_coords.len() - 1;
                    if curr_coords_idx == 0 {
                        // first antenna
                        continue;
                    }

                    for (idx, (pre_row, pre_col)) in prev_coords.iter().enumerate() {
                        // this is gonna be repeated a lot but we need it lol
                        antinode_locs.insert((*pre_row, *pre_col));

                        if idx == curr_coords_idx {
                            // this is the one we just added, so it is an antinode
                            antinode_locs.insert((row_idx, col_idx));
                            continue;
                        }
                        let row_diff = (row_idx as i32 - *pre_row as i32).abs();
                        let col_diff = col_idx as i32 - *pre_col as i32;
                        let mut antinode1_row = row_idx as i32 + row_diff;
                        let mut antinode1_col = col_idx as i32 + col_diff;
                        // do antinode 1 aka increasing
                        loop {
                            if antinode1_row >= 0
                                && antinode1_row < row_max as i32
                                && antinode1_col >= 0
                                && antinode1_col < col_max as i32
                            {
                                antinode_locs
                                    .insert((antinode1_row as usize, antinode1_col as usize));
                            } else {
                                // loop until we are out of bounds
                                break;
                            }
                            antinode1_row += row_diff;
                            antinode1_col += col_diff;
                        }
                        let mut antinode2_row = *pre_row as i32 - row_diff;
                        let mut antinode2_col = *pre_col as i32 - col_diff;
                        // do antinode 2 aka decreasing
                        loop {
                            if antinode2_row >= 0
                                && antinode2_row < row_max as i32
                                && antinode2_col >= 0
                                && antinode2_col < col_max as i32
                            {
                                antinode_locs
                                    .insert((antinode2_row as usize, antinode2_col as usize));
                            } else {
                                // loop until we are out of bounds
                                break;
                            }
                            antinode2_row -= row_diff;
                            antinode2_col -= col_diff;
                        }
                    }
                }
            }
        }

        antinode_locs.len().to_string()
    }
}

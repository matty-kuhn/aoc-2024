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
        Map {
            grid: self.input.lines().fold(Vec::new(), |mut acc, line| {
                acc.push(line.chars().map(|ch| (ch, 1)).collect());
                acc
            }),
        }
    }
}

impl Day for Day20 {
    fn part1(&self) -> String {
        let mut map = self.parse_input();

        // loading points as (((coord x, coord y), cheats_left), 1)

        // worst path, no cheating
        let worst_path = astar(
            &(map.get_start().0, map.get_start().1 - 2),
            |p| map.get_peers(p.0, p.1),
            |p| {
                let end = map.get_end();
                (end.0 as isize - p.0 .0 as isize).pow(2) as usize
                    + (end.1 as isize - p.0 .1 as isize).pow(2) as usize
            },
            |p| p.0 == map.get_end(),
        )
        .unwrap();

        // best path, optimal cheat
        let mut paths = 0;
        for y in 1..map.grid.len() - 1 {
            for x in 1..map.grid[0].len() - 1 {
                if map.grid[y][x].0 == '#' {
                    map.grid[y][x].0 = '.';
                }

                let Some(best_path) = astar(
                    &(map.get_start().0, map.get_start().1 - 2),
                    |p| map.get_peers(p.0, p.1),
                    |p| {
                        let end = map.get_end();
                        (end.0 as isize - p.0 .0 as isize).pow(2) as usize
                            + (end.1 as isize - p.0 .1 as isize).pow(2) as usize
                    },
                    |p| p.0 == map.get_end(),
                ) else {
                    map.grid[y][x].0 = '#';
                    continue;
                };

                if best_path.1 < worst_path.1 {
                    paths += 1;
                }

                map.grid[y][x].0 = '#';
            }
            println!("done {y}");
        }
        // println!(
        //     "cost worst {} best {} savings {}",
        //     worst_path.1,
        //     best_path.1,
        //     worst_path.1 - best_path.1
        // );
        // todo!();
        // map.print_path(&best_path.0);
        // while worst_path.1 as i64 - best_path.1 as i64 >= 100 {
        //     println!(
        //         "cost worst {} best {} savings {}",
        //         worst_path.1,
        //         best_path.1,
        //         worst_path.1 as i64 - best_path.1 as i64
        //     );
        //     paths += 1;
        //     // modify the map to make the cheated square be max weight, so we cheat somewhere else
        //     // keep doing that until time_savings < 100
        //     for pt in &best_path.0 {
        //         // find first pt with < 2 cheats left, raise cost
        //         if pt.1 < 2 {
        //             map.grid[pt.0 .1][pt.0 .0].1 = 10_000_000_000;
        //             break;
        //         }
        //     }
        //     best_path = astar(
        //         &map.get_start(),
        //         |p| map.get_peers(p.0, p.1),
        //         |p| {
        //             let end = map.get_end();
        //             (end.0 as isize - p.0 .0 as isize).pow(2) as usize
        //                 + (end.1 as isize - p.0 .1 as isize).pow(2) as usize
        //         },
        //         |p| p.0 == map.get_end(),
        //     )
        //     .unwrap();
        //     // map.print_path(&best_path.0);
        // }
        paths.to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}

struct Map {
    grid: Vec<Vec<(char, usize)>>,
    // grid: Vec<Vec<char>>,
}

impl Map {
    fn print_path(&self, path: &[((usize, usize), u8)]) {
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if path.contains(&((col_idx, row_idx), 2)) {
                    print!("O");
                    continue;
                }
                if path.contains(&((col_idx, row_idx), 1)) {
                    print!("1");
                    continue;
                }
                if path.contains(&((col_idx, row_idx), 0)) {
                    print!("2");
                    continue;
                }
                print!("{}", col.0);
            }
            println!()
        }
    }

    fn is_empty(ch: char) -> bool {
        ch == '.' || ch == 'S' || ch == 'E'
    }

    fn get_peers(
        &self,
        node: (usize, usize),
        mut cheats_left: u8,
    ) -> Vec<(((usize, usize), u8), usize)> {
        let mut ret = vec![];
        if node.0 > 0 {
            let curr = self.grid[node.1][node.0 - 1];
            // check left
            if Self::is_empty(curr.0) {
                // we cheated right before this, so we need to use up last cheat
                if cheats_left == 1 {
                    cheats_left -= 1;
                }
                ret.push((((node.0 - 1, node.1), cheats_left), curr.1));
            } else {
                // not empty but we can cheat
                if cheats_left > 1 {
                    ret.push((((node.0 - 1, node.1), cheats_left - 1), curr.1));
                }
            }
        }
        if node.0 < self.grid[0].len() - 1 {
            let curr = self.grid[node.1][node.0 + 1];
            // check right
            if Self::is_empty(curr.0) {
                // we cheated right before this, so we need to use up last cheat
                if cheats_left == 1 {
                    cheats_left -= 1;
                }
                ret.push((((node.0 + 1, node.1), cheats_left), curr.1));
            } else {
                // not empty but we can cheat
                if cheats_left > 1 {
                    ret.push((((node.0 + 1, node.1), cheats_left - 1), curr.1));
                }
            }
        }
        if node.1 > 0 {
            let curr = self.grid[node.1 - 1][node.0];
            // check up
            if Self::is_empty(curr.0) {
                // we cheated right before this, so we need to use up last cheat
                if cheats_left == 1 {
                    cheats_left -= 1;
                }
                ret.push((((node.0, node.1 - 1), cheats_left), curr.1));
            } else {
                // not empty but we can cheat
                if cheats_left > 1 {
                    ret.push((((node.0, node.1 - 1), cheats_left - 1), curr.1));
                }
            }
        }
        if node.1 < self.grid.len() - 1 {
            let curr = self.grid[node.1 + 1][node.0];
            // check down
            if Self::is_empty(curr.0) {
                // we cheated right before this, so we need to use up last cheat
                if cheats_left == 1 {
                    cheats_left -= 1;
                }
                ret.push((((node.0, node.1 + 1), cheats_left), curr.1));
            } else {
                // not empty but we can cheat
                if cheats_left > 1 {
                    ret.push((((node.0, node.1 + 1), cheats_left - 1), curr.1));
                }
            }
        }

        ret
    }

    fn get_start(&self) -> ((usize, usize), u8) {
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if col.0 == 'S' {
                    return ((col_idx, row_idx), 2);
                }
            }
        }
        unreachable!()
    }
    fn get_end(&self) -> (usize, usize) {
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if col.0 == 'E' {
                    return (col_idx, row_idx);
                }
            }
        }
        unreachable!()
    }
}

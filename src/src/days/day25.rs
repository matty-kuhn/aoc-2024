use super::Day;

pub struct Day25 {
    input: String,
}

impl Day25 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_lines(&self) -> (Vec<Key>, Vec<Lock>) {
        let mut keys = vec![];
        let mut locks = vec![];
        let items: Vec<_> = self.input.split("\n\n").collect();

        for item in items {
            let item_chars: Vec<Vec<_>> = item.lines().map(|line| line.chars().collect()).collect();
            let mut transposed = (0..item_chars[0].len()).map(|_| vec![]).collect::<Vec<_>>();
            for row in item_chars {
                for (idx, trans_row) in row.into_iter().zip(&mut transposed) {
                    trans_row.push(idx);
                }
            }
            // dbg!(transposed);
            if transposed[0].starts_with(&['#']) {
                //lock
                locks.push(Lock(
                    transposed[0]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                    transposed[1]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                    transposed[2]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                    transposed[3]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                    transposed[4]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                ));
            } else {
                keys.push(Key(
                    transposed[0]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                    transposed[1]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                    transposed[2]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                    transposed[3]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                    transposed[4]
                        .iter()
                        .fold(0, |acc, ch| if ch == &'#' { acc + 1 } else { acc })
                        - 1,
                ));
            }
        }

        (keys, locks)
    }
}

impl Day for Day25 {
    fn part1(&self) -> String {
        let (keys, locks) = self.parse_lines();
        // dbg!(&keys, &locks);
        let mut matches = 0;
        for key in &keys {
            for lock in &locks {
                if key.0 + lock.0 <= 5
                    && key.1 + lock.1 <= 5
                    && key.2 + lock.2 <= 5
                    && key.3 + lock.3 <= 5
                    && key.4 + lock.4 <= 5
                {
                    matches += 1;
                }
            }
        }
        matches.to_string()
    }

    fn part2(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
struct Key(u8, u8, u8, u8, u8);
#[derive(Debug)]
struct Lock(u8, u8, u8, u8, u8);

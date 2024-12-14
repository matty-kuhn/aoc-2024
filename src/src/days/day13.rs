use super::{get_lines, Day};
use regex::Regex;

pub struct Day13 {
    input: String,
}

impl Day13 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Vec<Game> {
        // Single entry:
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        let mut games = vec![];
        let mut x_target = 0;
        let mut y_target = 0;
        let mut a_button = (0, 0);
        let mut b_button = (0, 0);
        for line in get_lines(&self.input) {
            if line.trim().is_empty() {
                games.push(Game {
                    x_target,
                    y_target,
                    a_button,
                    b_button,
                });
                x_target = 0;
                y_target = 0;
                a_button = (0, 0);
                b_button = (0, 0);

                continue;
            }
            let buttona_re =
                Regex::new(r"Button A: X\+(?<x>[0-9]{1,3}), Y\+(?<y>[0-9]{1,3})").unwrap();
            let captures = buttona_re.captures_iter(line);
            for capture in captures {
                a_button = (capture["x"].parse().unwrap(), capture["y"].parse().unwrap());
                continue;
            }
            let buttonb_re =
                Regex::new(r"Button B: X\+(?<x>[0-9]{1,3}), Y\+(?<y>[0-9]{1,3})").unwrap();
            let captures = buttonb_re.captures_iter(line);
            for capture in captures {
                b_button = (capture["x"].parse().unwrap(), capture["y"].parse().unwrap());
                continue;
            }
            let prize_re =
                Regex::new(r"Prize: X\=(?<x>[0-9]{1,10}), Y\=(?<y>[0-9]{1,10})").unwrap();
            let captures = prize_re.captures_iter(line);
            for capture in captures {
                x_target = capture["x"].parse().unwrap();
                y_target = capture["y"].parse().unwrap();
                continue;
            }
        }

        games
    }
}

impl Day for Day13 {
    fn part1(&self) -> String {
        self.parse_input()
            .iter()
            .fold(0, |acc, game| acc + game.sovle::<0>())
            .to_string()
    }

    fn part2(&self) -> String {
        self.parse_input()
            .iter()
            .fold(0, |acc, game| acc + game.sovle::<10000000000000>())
            .to_string()
    }
}

#[derive(Debug)]
struct Game {
    x_target: isize,
    y_target: isize,
    a_button: (isize, isize),
    b_button: (isize, isize),
}

impl Game {
    // need to solve the linear equations
    // a = (prizex * by - prizey * bx) / (ax * by - ay * bx)
    // b = (prizex * ay - prizey * ax) / (bx * ay - by * ax)
    // shoutout reddit ^
    fn sovle<const FIXER: isize>(&self) -> isize {
        let a_num =
            (self.x_target + FIXER) * self.b_button.1 - (self.y_target + FIXER) * self.b_button.0;
        let a_denom = self.a_button.0 * self.b_button.1 - self.a_button.1 * self.b_button.0;
        let b_num =
            (self.x_target + FIXER) * self.a_button.1 - (self.y_target + FIXER) * self.a_button.0;
        let b_denom = self.b_button.0 * self.a_button.1 - self.b_button.1 * self.a_button.0;
        // check it works
        if a_num % a_denom != 0 || b_num % b_denom != 0 {
            return 0;
        }
        let a = a_num / a_denom;
        let b = b_num / b_denom;
        a * 3 + b
    }
}

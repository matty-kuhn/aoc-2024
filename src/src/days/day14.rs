use std::collections::HashMap;

use super::Day;

pub struct Day14 {
    input: String,
}

impl Day14 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Robots {
        let mut ret = Robots {
            start_pos: Vec::new(),
            velocity: Vec::new(),
        };
        let re =
            regex::Regex::new(r"p=(?<startx>[0-9]{1,3}),(?<starty>[0-9]{1,3}) v=(?<deltax>-?[0-9]{1,3}),(?<deltay>-?[0-9]{1,3})")
                .unwrap();
        let captures = re.captures_iter(&self.input);
        for capture in captures {
            ret.start_pos.push((
                capture["startx"].parse().unwrap(),
                capture["starty"].parse().unwrap(),
            ));
            ret.velocity.push((
                capture["deltax"].parse().unwrap(),
                capture["deltay"].parse().unwrap(),
            ));
        }

        ret
    }

    fn display_map(map: &HashMap<(i64, i64), i64>) {
        for y in 0..=MAX_Y {
            for x in 0..=MAX_X {
                if x == MID_X || y == MID_Y {
                    print!(" ");
                    continue;
                }
                if let Some(count) = map.get(&(x, y)) {
                    print!("{}", count);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn save_bitmap(map: &HashMap<(i64, i64), i64>, iter: usize) {
        use bmp::{Image, Pixel};
        let mut img = Image::new((MAX_X + 1) as u32, (MAX_Y + 1) as u32);
        for y in 0..=MAX_Y {
            for x in 0..=MAX_X {
                if let Some(_) = map.get(&(x, y)) {
                    img.set_pixel(x as u32, y as u32, Pixel::new(0, 0, 0));
                } else {
                    img.set_pixel(x as u32, y as u32, Pixel::new(255, 255, 255));
                }
            }
        }
        let _ = img.save(format!("{iter}.bmp"));
    }

    fn count_quads(map: &HashMap<(i64, i64), i64>) -> (i64, i64, i64, i64) {
        let mut count_top_left = 0;
        let mut count_top_right = 0;
        let mut count_bottom_left = 0;
        let mut count_bottom_right = 0;
        // top left
        for y in 0..MID_Y {
            for x in 0..MID_X {
                if let Some(count) = map.get(&(x, y)) {
                    count_top_left += count;
                }
            }
        }
        // top right
        for y in 0..MID_Y {
            for x in (MID_X + 1)..=MAX_X {
                if let Some(count) = map.get(&(x, y)) {
                    count_top_right += count;
                }
            }
        }
        // bottom left
        for y in (MID_Y + 1)..=MAX_Y {
            for x in 0..MID_X {
                if let Some(count) = map.get(&(x, y)) {
                    count_bottom_left += count;
                }
            }
        }
        // bottom right
        for y in (MID_Y + 1)..=MAX_Y {
            for x in (MID_X + 1)..=MAX_X {
                if let Some(count) = map.get(&(x, y)) {
                    count_bottom_right += count;
                }
            }
        }
        (
            count_top_left,
            count_top_right,
            count_bottom_left,
            count_bottom_right,
        )
    }
}
const ITERATIONS: i64 = 100;
#[cfg(debug_assertions)]
const MAX_X: i64 = 10;
#[cfg(debug_assertions)]
const MAX_Y: i64 = 6;
#[cfg(not(debug_assertions))]
const MAX_X: i64 = 100;
#[cfg(not(debug_assertions))]
const MAX_Y: i64 = 102;
const MID_X: i64 = MAX_X / 2;
const MID_Y: i64 = MAX_Y / 2;

impl Day for Day14 {
    fn part1(&self) -> String {
        let mut counts = HashMap::new();
        let robots = self.parse_input();

        for idx in 0..robots.start_pos.len() {
            let start_pos = robots.start_pos[idx];
            let velocity = robots.velocity[idx];
            let mut final_pos = (
                start_pos.0 + velocity.0 * ITERATIONS,
                start_pos.1 + velocity.1 * ITERATIONS,
            );
            // println!("pre-adjust: {final_pos:?}");
            if velocity.0 > 0 {
                final_pos.0 = final_pos.0 % (MAX_X + 1);
                if final_pos.0 > MAX_X {
                    println!("adjust x pos: {final_pos:?}");
                }
            }
            if velocity.1 > 0 {
                final_pos.1 = final_pos.1 % (MAX_Y + 1);
                if final_pos.1 > MAX_Y {
                    println!("adjust y pos: {final_pos:?}");
                }
            }
            if velocity.0 < 0 {
                final_pos.0 = final_pos.0 % (MAX_X + 1);
                if final_pos.0 < 0 {
                    final_pos.0 += MAX_X + 1;
                }
            }
            if velocity.1 < 0 {
                final_pos.1 = final_pos.1 % (MAX_Y + 1);
                if final_pos.1 < 0 {
                    final_pos.1 += MAX_Y + 1;
                }
            }
            *counts.entry(final_pos).or_default() += 1;
        }
        Self::display_map(&counts);
        let counts = Self::count_quads(&counts);
        // dbg!(&counts);
        (counts.0 * counts.1 * counts.2 * counts.3).to_string()
    }

    fn part2(&self) -> String {
        let mut robots = self.parse_input();

        for iter in 0..10000 {
            let mut counts = HashMap::new();
            for idx in 0..robots.start_pos.len() {
                let start_pos = robots.start_pos[idx];
                let velocity = robots.velocity[idx];
                let mut final_pos = (start_pos.0 + velocity.0, start_pos.1 + velocity.1);
                // println!("pre-adjust: {final_pos:?}");
                if velocity.0 > 0 {
                    final_pos.0 = final_pos.0 % (MAX_X + 1);
                    if final_pos.0 > MAX_X {
                        println!("adjust x pos: {final_pos:?}");
                    }
                }
                if velocity.1 > 0 {
                    final_pos.1 = final_pos.1 % (MAX_Y + 1);
                    if final_pos.1 > MAX_Y {
                        println!("adjust y pos: {final_pos:?}");
                    }
                }
                if velocity.0 < 0 {
                    final_pos.0 = final_pos.0 % (MAX_X + 1);
                    if final_pos.0 < 0 {
                        final_pos.0 += MAX_X + 1;
                    }
                }
                if velocity.1 < 0 {
                    final_pos.1 = final_pos.1 % (MAX_Y + 1);
                    if final_pos.1 < 0 {
                        final_pos.1 += MAX_Y + 1;
                    }
                }
                *counts.entry(final_pos).or_default() += 1;
                robots.start_pos[idx] = final_pos;
            }
            Self::display_map(&counts);
            Self::save_bitmap(&counts, iter);
        }
        "done".to_string()
    }
}

#[derive(Debug)]
struct Robots {
    start_pos: Vec<(i64, i64)>,
    velocity: Vec<(i64, i64)>,
}

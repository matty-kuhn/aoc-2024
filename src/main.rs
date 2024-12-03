use std::time;

use clap::Parser;
use days::{day_builder, Day, CURRENT_DAY};

pub mod days;

#[derive(Parser, Debug)]
struct Cli {
    /// The day to run
    day: Option<i8>,
    /// the part to run
    part: Option<u8>,
}

fn main() {
    let args = Cli::parse();
    let day = args.day.unwrap_or(-1);
    let mut days: Vec<(i8, Box<dyn Day>)> = Vec::new();
    if cfg!(debug_assertions) {
        if day == -1 {
            for day_num in 1..=CURRENT_DAY {
                days.push((
                    day_num,
                    day_builder(day_num, &format!("day{}_test", day_num)),
                ));
            }
        } else {
            days.push((day, day_builder(day, &format!("day{}_test", day))));
        }
    } else {
        if day == -1 {
            for day_num in 1..=CURRENT_DAY {
                days.push((day_num, day_builder(day_num, &format!("day{}", day_num))));
            }
        } else {
            days.push((day, day_builder(day, &format!("day{}", day))));
        }
    }
    for (idx, day) in days {
        println!("Day {}", idx);
        if let Some(part) = args.part {
            match part {
                1 => {
                    let part1 = time::Instant::now();
                    let part1_res = day.part1();
                    let part1_elapsed = part1.elapsed();
                    println!("\tPart 1: {}\n\telapsed: {:.6?}", part1_res, part1_elapsed);
                }
                2 => {
                    let part2 = time::Instant::now();
                    let part2_res = day.part2();
                    let part2_elapsed = part2.elapsed();
                    println!("\tPart 2: {}\n\telapsed: {:.6?}", part2_res, part2_elapsed);
                }
                _ => panic!("Part {} not implemented yet", part),
            }
            continue;
        } else {
            let part1 = time::Instant::now();
            let part1_res = day.part1();
            let part1_elapsed = part1.elapsed();
            println!("\tPart 1: {}\n\telapsed: {:.6?}", part1_res, part1_elapsed);
            let part2 = time::Instant::now();
            let part2_res = day.part2();
            let part2_elapsed = part2.elapsed();
            println!("\tPart 2: {}\n\telapsed: {:.6?}", part2_res, part2_elapsed);
        }
    }
}

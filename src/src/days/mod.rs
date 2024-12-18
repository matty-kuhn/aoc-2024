use day1::Day1;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day2::Day2;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use std::fs;

pub const CURRENT_DAY: i8 = 18;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub trait Day {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub fn get_day_input(day: &str) -> String {
    fs::read_to_string(format!("../inputs/{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn get_lines(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

pub fn day_builder(day: i8, input_name: &str) -> Box<dyn Day> {
    match day {
        1 => Box::new(Day1::new(get_day_input(input_name))) as Box<dyn Day>,
        2 => Box::new(Day2::new(get_day_input(input_name))) as Box<dyn Day>,
        3 => Box::new(Day3::new(get_day_input(input_name))) as Box<dyn Day>,
        4 => Box::new(Day4::new(get_day_input(input_name))) as Box<dyn Day>,
        5 => Box::new(Day5::new(get_day_input(input_name))) as Box<dyn Day>,
        6 => Box::new(Day6::new(get_day_input(input_name))) as Box<dyn Day>,
        7 => Box::new(Day7::new(get_day_input(input_name))) as Box<dyn Day>,
        8 => Box::new(Day8::new(get_day_input(input_name))) as Box<dyn Day>,
        9 => Box::new(Day9::new(get_day_input(input_name))) as Box<dyn Day>,
        10 => Box::new(Day10::new(get_day_input(input_name))) as Box<dyn Day>,
        11 => Box::new(Day11::new(get_day_input(input_name))) as Box<dyn Day>,
        12 => Box::new(Day12::new(get_day_input(input_name))) as Box<dyn Day>,
        13 => Box::new(Day13::new(get_day_input(input_name))) as Box<dyn Day>,
        14 => Box::new(Day14::new(get_day_input(input_name))) as Box<dyn Day>,
        15 => Box::new(Day15::new(get_day_input(input_name))) as Box<dyn Day>,
        16 => Box::new(Day16::new(get_day_input(input_name))) as Box<dyn Day>,
        17 => Box::new(Day17::new(get_day_input(input_name))) as Box<dyn Day>,
        18 => Box::new(Day18::new(get_day_input(input_name))) as Box<dyn Day>,
        19 => Box::new(Day19::new(get_day_input(input_name))) as Box<dyn Day>,
        20 => Box::new(Day20::new(get_day_input(input_name))) as Box<dyn Day>,
        21 => Box::new(Day21::new(get_day_input(input_name))) as Box<dyn Day>,
        22 => Box::new(Day22::new(get_day_input(input_name))) as Box<dyn Day>,
        23 => Box::new(Day23::new(get_day_input(input_name))) as Box<dyn Day>,
        24 => Box::new(Day24::new(get_day_input(input_name))) as Box<dyn Day>,
        25 => Box::new(Day25::new(get_day_input(input_name))) as Box<dyn Day>,
        _ => panic!("Day {} not implemented yet", day),
    }
}

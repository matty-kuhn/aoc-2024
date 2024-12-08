use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use std::fs;

pub const CURRENT_DAY: i8 = 8;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub trait Day {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub fn get_day_input(day: &str) -> String {
    fs::read_to_string(format!("inputs/{}.txt", day))
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
        _ => panic!("Day {} not implemented yet", day),
    }
}

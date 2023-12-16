use std::time::Duration;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

pub fn solve(day: &str, input: &str) -> (String, String, Duration) {
    match day {
        "1" => day1::solve(input),
        "2" => day2::solve(input),
        "3" => day3::solve(input),
        "4" => day4::solve(input),
        "5" => day5::solve(input),
        "6" => day6::solve(input),
        "7" => day7::solve(input),
        "8" => day8::solve(input),
        "9" => day9::solve(input),
        "10" => day10::solve(input),
        "11" => day11::solve(input),
        "12" => day12::solve(input),
        "13" => day13::solve(input),
        "14" => day14::solve(input),
        _ => panic!("No such day solution implemented."),
    }
}

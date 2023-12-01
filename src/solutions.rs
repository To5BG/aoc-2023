use std::time::Duration;

mod day1;
mod day2;

pub fn solve(day: &str, input: &str) -> (String, String, Duration) {
    match day {
        "1" => day1::solve(input),
        "2" => day2::solve(input),
        _ => panic!("No such day solution implemented."),
    }
}

use std::time::Duration;

mod day1;

pub fn solve(day: &str, input: &str) -> (String, String, Duration) {
    match day {
        "1" => day1::solve(input),
        _ => panic!("No such day solution implemented."),
    }
}

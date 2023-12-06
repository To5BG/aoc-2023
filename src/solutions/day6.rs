use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let p1 = input
        .split('\n')
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .zip(
            input
                .split('\n')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter_map(|n| n.parse::<u64>().ok()),
        )
        .map(|t| {
            (1..t.0)
                .find_map(|n| (n * (t.0 - n) > t.1).then_some(t.0 - 2 * n + 1))
                .unwrap()
        })
        .reduce(|a, b| a * b);
    let large = (
        input.split('\n').next().unwrap().replace(' ', "")[5..]
            .parse::<u64>()
            .unwrap(),
        input.split('\n').nth(1).unwrap().replace(' ', "")[9..]
            .parse::<u64>()
            .unwrap(),
    );
    let p2 = (1..large.0)
        .find_map(|n| (n * (large.0 - n) > large.1).then_some(large.0 - 2 * n + 1))
        .unwrap();
    (p1.unwrap().to_string(), p2.to_string(), t.elapsed())
}

use regex::Regex;
use std::time::{Duration, Instant};

const SET: &[(&str, usize)] = &[("red", 12), ("green", 13), ("blue", 14)];

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let r = Regex::new(r"(\d+) (\w+),?").unwrap();
    let p1 = input
        .lines()
        .enumerate()
        .filter(|l| {
            l.1.split(';').all(|p| {
                r.captures_iter(p).all(|m| {
                    m.get(1).unwrap().as_str().parse::<usize>().unwrap()
                        <= SET
                            .iter()
                            .find_map(|v| (v.0 == m.get(2).unwrap().as_str()).then_some(v.1))
                            .unwrap_or(100)
                })
            })
        })
        .fold(0, |acc, e| acc + e.0 + 1);
    let p2 = input
        .lines()
        .map(|l| {
            r.captures_iter(l).fold((0, 0, 0), |mut acc, e| {
                let amount = e.get(1).unwrap().as_str().parse::<usize>().unwrap();
                match SET
                    .iter()
                    .enumerate()
                    .find_map(|v| (v.1 .0 == e.get(2).unwrap().as_str()).then_some(v.0))
                    .unwrap()
                {
                    0 => acc.0 = usize::max(acc.0, amount),
                    1 => acc.1 = usize::max(acc.1, amount),
                    2 => acc.2 = usize::max(acc.2, amount),
                    _ => (),
                }
                acc
            })
        }) //.for_each(|e| println!("{},{},{}", e.0, e.1, e.2));
        .fold(0, |acc, e| acc + e.0 * e.1 * e.2);
    (p1.to_string(), p2.to_string(), t.elapsed())
}

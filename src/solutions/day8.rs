use std::collections::HashMap;
use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let (dirs, map) = input
        .split_once("\n\n")
        .map(|t| {
            (
                t.0.chars().collect::<Vec<char>>(),
                t.1.lines()
                    .map(|l| (&l[0..3], (&l[7..10], &l[12..15])))
                    .collect::<HashMap<&str, (&str, &str)>>(),
            )
        })
        .unwrap();
    (
        traverse(&map, &dirs, "AAA", false).to_string(),
        map.keys()
            .filter_map(|s| s.ends_with('A').then_some(*s))
            .fold(1, |a, e| {
                let b = traverse(&map, &dirs, e, true);
                a * b / gcd(a, b)
            })
            .to_string(),
        t.elapsed(),
    )
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn traverse(map: &HashMap<&str, (&str, &str)>, dir: &Vec<char>, start: &str, pt2: bool) -> usize {
    let (mut steps, mut curr) = (0_usize, start);
    loop {
        if curr == "ZZZ" || (pt2 && curr.ends_with('Z')) {
            return steps;
        }
        curr = if dir[steps % dir.len()] == 'L' {
            map.get(curr).unwrap().0
        } else {
            map.get(curr).unwrap().1
        };
        steps += 1;
    }
}

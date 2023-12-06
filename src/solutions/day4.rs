use std::time::{Duration, Instant};

use hashbrown::HashSet;

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let cards = input
        .lines()
        .map(|l| {
            l.split_once('|')
                .map(|o| (&o.0[8..(o.0.len() - 1)], &o.1[1..]))
                .unwrap()
        })
        .map(|t| {
            (
                t.0.split(' ')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<HashSet<u32>>(),
                t.1.split(' ')
                    .filter_map(|s| s.parse::<u32>().ok())
                    .collect::<HashSet<u32>>(),
            )
        })
        .map(|t| t.0.iter().filter(|e| t.1.contains(*e)).count() as u32)
        .collect::<Vec<u32>>();
    let p1 = cards
        .iter()
        .map(|c| if c == &0 { 0 } else { (2_u64).pow(c - 1) })
        .sum::<u64>();
    let p2 = cards
        .into_iter()
        .fold(
            (
                0,
                "1".repeat(11)
                    .chars()
                    .map(|c| (c as u64) - 48)
                    .collect::<Vec<_>>(),
                0_usize,
            ),
            |mut acc, e| {
                let curr = acc.1[acc.2 % 11];
                for i in 1..=e {
                    acc.1[((i + acc.2 as u32) % 11) as usize] += curr;
                }
                acc.0 += curr;
                acc.1[acc.2 % 11] = 1;
                acc.2 += 1;
                acc
            },
        )
        .0;
    (p1.to_string(), p2.to_string(), t.elapsed())
}

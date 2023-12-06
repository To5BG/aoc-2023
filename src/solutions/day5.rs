use std::time::{Duration, Instant};

use itertools::Itertools;

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let ranges = input
        .split("\n\n")
        .skip(1)
        .map(|a| {
            a.lines()
                .skip(1)
                .map(|l| {
                    let mut it = l.split_whitespace().map(|n| n.parse::<i64>().unwrap());
                    let (d, s) = (it.next().unwrap(), it.next().unwrap());
                    (d - s, s, it.next().unwrap() + s)
                })
                .collect::<Vec<(i64, i64, i64)>>()
        })
        .collect::<Vec<Vec<(i64, i64, i64)>>>();
    let seeds = &mut input[7..input.find('\n').unwrap()]
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let p1 = seeds
        .clone()
        .map(|seed| {
            ranges.iter().fold(seed, |curr, sec| {
                #[allow(clippy::unnecessary_lazy_evaluations)]
                sec.iter()
                    .find_map(|r| (curr >= r.1 && curr < r.2).then(|| r.0 + curr))
                    .unwrap_or(curr)
            })
        })
        .reduce(i64::min);
    let seed_ranges = seeds
        .tuples()
        .map(|(st, len)| (st, st + len))
        .collect::<Vec<(i64, i64)>>();
    let p2 = ranges
        .iter()
        .fold(seed_ranges, |seeds, sec| {
            seeds
                .iter()
                .flat_map(|&(start, end)| {
                    let (mut seen, mut curr) = (Vec::new(), Vec::from([(start, end)]));
                    for &(delta, range_s, range_e) in sec {
                        let mut m = Vec::new();
                        for (seed_s, seed_e) in curr {
                            let (b1, b2) = (seed_s.max(range_s), seed_e.min(range_e));
                            if seed_s < range_s {
                                m.push((seed_s, seed_e.min(range_s)));
                            }
                            if b1 < b2 {
                                seen.push((b1 + delta, b2 + delta));
                            }
                            if range_e < seed_e {
                                m.push((seed_s.max(range_e), seed_e));
                            }
                        }
                        curr = m;
                    }
                    seen.extend(curr);
                    seen
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .iter()
        .fold(i64::MAX, |acc, e| acc.min(e.0));
    (p1.unwrap().to_string(), p2.to_string(), t.elapsed())
}

use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let (p1, p2) = input
        .lines()
        .map(|l| {
            let mut v = l
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let (mut res, mut res2, mut f) = (*v.last().unwrap(), *v.first().unwrap(), false);
            loop {
                v = v.windows(2).map(|t| t[1] - t[0]).collect::<Vec<i32>>();
                res += v.last().unwrap();
                res2 += v.first().unwrap() * (if f { 1 } else { -1 });
                f ^= true;
                if v.len() == 1 || v.iter().all(|f| f == &0) {
                    break;
                }
            }
            (res, res2)
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();
    (p1.to_string(), p2.to_string(), t.elapsed())
}

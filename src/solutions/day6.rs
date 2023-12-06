use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let s = input.replace(' ', "");
    let (idx, idx2) = (input.find('\n').unwrap(), s.find('\n').unwrap());
    let p1 = &input[5..idx]
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .zip(input[(idx + 10)..].split_whitespace())
        .map(find)
        .product::<i64>();
    let p2 = find((
        s[5..idx2].parse::<i64>().unwrap(),
        &s[(idx2 + 10)..(s.len() - 1)],
    ));
    (p1.to_string(), p2.to_string(), t.elapsed())
}

fn find(f: (i64, &str)) -> i64 {
    let w = f.1.parse::<i64>().unwrap();
    (((f.0 - f32::sqrt((f.0.pow(2) - 4 * w) as f32) as i64) / 2)..f.0)
        .find_map(|n| (n * (f.0 - n) > w).then_some(f.0 - 2 * n + 1))
        .unwrap()
}

use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let lines = input.lines().map(str::as_bytes).collect::<Vec<&[u8]>>();
    let p1 = lines
        .iter()
        .map(|line| {
            let mut it = line
                .iter()
                .filter_map(|c| c.is_ascii_digit().then_some(c - 48));
            let first = it.next().unwrap();
            (first * 10 + it.last().unwrap_or(first)) as u32
        })
        .sum::<u32>();
    let num = Vec::from([
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]);
    let p2 = lines
        .iter()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|i| match line[i] {
                d if (48..58).contains(&d) => Some(d - 48),
                _ => {
                    let mut c = 0;
                    num.iter().find_map(|s| {
                        c += 1;
                        line[i..].starts_with(s.as_bytes()).then_some(c)
                    })
                }
            });
            let first = it.next().unwrap();
            (first * 10 + it.last().unwrap_or(first)) as u32
        })
        .sum::<u32>();
    (p1.to_string(), p2.to_string(), t.elapsed())
}

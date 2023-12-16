use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let pats = input.split("\n\n").collect::<Vec<&str>>();
    (s(&pats, 0), s(&pats, 1), t.elapsed())
}

fn s(pat: &[&str], smudges: u32) -> String {
    pat.iter()
        .map(|s| {
            find(
                s.lines()
                    .map(|l| l.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>(),
                smudges,
            )
        })
        .fold(0, |acc, e| if e > 0 { acc + e } else { acc + 100 * -e })
        .to_string()
}

fn find(pat: Vec<Vec<char>>, smudges: u32) -> i32 {
    let (h, w) = (pat.len(), pat[0].len());
    (1..w)
        .find(|c| {
            let mut count = 0;
            for cc in 0..(w - c).min(*c) {
                for patr in pat.iter() {
                    if patr[c + cc] != patr[c - cc - 1] {
                        if count == smudges {
                            return false;
                        }
                        count += 1;
                    }
                }
            }
            count == smudges
        })
        .map(|o| o as i32)
        .or((1..h)
            .find(|r| {
                let mut count = 0;
                for rr in 0..(h - r).min(*r) {
                    for cc in 0..w {
                        if pat[r + rr][cc] != pat[r - rr - 1][cc] {
                            if count == smudges {
                                return false;
                            }
                            count += 1;
                        }
                    }
                }
                count == smudges
            })
            .map(|o| -(o as i32)))
        .unwrap()
}

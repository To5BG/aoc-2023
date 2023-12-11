use itertools::Itertools;
use std::time::{Duration, Instant};

const SET: &[(char, u64)] = &[('A', 13), ('K', 12), ('Q', 11), ('J', 10), ('T', 9), ('9', 8), ('8', 7), ('7', 6), 
    ('6', 5), ('5', 4), ('4', 3), ('3', 2), ('2', 1),
];
const N14P5: u64 = 537824;

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    (s(input, false), s(input, true), t.elapsed())
}

fn s(input: &str, pt2: bool) -> String {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|t| (value(t.0, pt2), t.1.parse::<u32>().unwrap()))
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .enumerate()
        .map(|t| t.1 .1 * (t.0 + 1) as u32)
        .sum::<u32>()
        .to_string()
}

fn value(deck: &str, pt2: bool) -> u64 {
    let (mut v, mut res, mut mult, mut j) = (vec![0; 14], 0, N14P5, 0);
    for c in deck.chars() {
        mult /= 14;
        if pt2 && c == 'J' {
            j += 1;
        } else {
            let val = SET.iter().find_map(|s| (s.0 == c).then_some(s.1)).unwrap();
            res += mult * val;
            let prev = v[val as usize];
            if prev != 0 {
                res += (prev * 2 - 1) * N14P5;
            }
            v[val as usize] = prev + 1;
        }
    }
    if pt2 && j != 0 {
        let h = v.into_iter().reduce(u64::max).unwrap();
        if h == 0 {
            return 16 * N14P5;
        }
        res += N14P5 * j * (h * 2 + j - 2);
    }
    res
}

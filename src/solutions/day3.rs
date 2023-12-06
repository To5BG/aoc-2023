use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

// alrdy failed func prog... nice
pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let n = f32::sqrt(input.len() as f32) as i32;
    let mut inp = input.to_string();
    inp.insert_str(0, &"s".repeat((n + 1) as usize));
    inp.push_str(&"s".repeat((n + 1) as usize));
    let arr = inp
        .chars()
        .map(|c| if c == '.' || c == '\n' { 's' } else { c })
        .collect::<Vec<char>>();
    let (mut acc, mut p1, mut mult, mut f, mut st) = (0, 0, 1, false, (0, 0));
    let mut map: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
    for i in (1..=n).rev() {
        for j in (0..n).rev() {
            let curr = arr[(i * (n + 1) + j) as usize];
            if curr.is_ascii_digit() {
                if mult == 1 {
                    for t in [(0, 1), (1, 0), (-1, 0)] {
                        let (r, c) = ((i + t.0), (j + t.1));
                        let ch = arr[(r * (n + 1) + c) as usize];
                        if ch == '*' {
                            st = (r, c);
                        }
                        f |= ch.is_ascii_punctuation();
                    }
                }
                for t in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                    let (r, c) = ((i + t.0), (j + t.1));
                    let ch = arr[(r * (n + 1) + c) as usize];
                    if ch == '*' {
                        st = (r, c);
                    }
                    f |= ch.is_ascii_punctuation();
                }
                acc += mult * curr.to_digit(10).unwrap();
                mult *= 10;
            } else {
                if curr == '*' {
                    st = (i, j);
                }
                p1 += acc * (f || curr.is_ascii_punctuation()) as u32;
                if st != (0, 0) {
                    map.entry(st).or_default().push(acc);
                }
                (f, st, mult, acc) = (false, (0, 0), 1, 0);
            }
        }
    }
    p1 += f as u32 * acc;
    let p2 = map
        .values()
        .filter_map(|p| (p.len() == 2).then_some(p.iter().copied().product::<u32>()))
        .sum::<u32>();
    (p1.to_string(), p2.to_string(), t.elapsed())
}

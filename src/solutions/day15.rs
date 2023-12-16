use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let p1 = input.split(',').map(hash).sum::<usize>();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    input.split(',').for_each(|s| {
        let (l, r) = s.split_once('=').or(s.split_once('-')).unwrap();
        let b = &mut boxes[hash(l)];
        match (r.parse::<usize>(), b.iter().position(|e| e.0 == l)) {
            (Ok(fs), Some(idx)) => b[idx] = (l, fs),
            (Ok(fs), None) => b.push((l, fs)),
            (Err(_), Some(idx)) => {
                b.remove(idx);
            }
            _ => (),
        }
    });
    let p2 = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            (i + 1)
                * b.iter()
                    .enumerate()
                    .map(|(ei, e)| (ei + 1) * e.1)
                    .sum::<usize>()
        })
        .sum::<usize>();
    (p1.to_string(), p2.to_string(), t.elapsed())
}

fn hash(s: &str) -> usize {
    let mut res = 0;
    for c in s.chars() {
        res = ((res + (c as usize)) * 17) % 256;
    }
    res
}

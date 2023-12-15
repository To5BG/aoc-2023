use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let mut g = Vec::<(usize, usize)>::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let (mut r, mut c) = (vec![true; lines.len()], vec![true; lines.len()]);
    for (i, l) in input.lines().enumerate() {
        for (j, _) in l.match_indices('#') {
            r[i] = false;
            c[j] = false;
            g.push((i, j));
        }
    }
    (s(g.clone(), &r, &c, 2), s(g, &r, &c, 1000000), t.elapsed())
}

pub fn s(mut galaxies: Vec<(usize, usize)>, rows: &[bool], cols: &[bool], e: usize) -> String {
    let mut p = 0;
    for (r, _) in rows.iter().enumerate().filter(|p| *p.1).rev() {
        for g in &mut galaxies {
            if g.0 > r {
                g.0 += e - 1;
            }
        }
    }
    for (c, _) in cols.iter().enumerate().filter(|p| *p.1).rev() {
        for g in &mut galaxies {
            if g.1 > c {
                g.1 += e - 1;
            }
        }
    }
    for l in 0..galaxies.len() {
        for r in (l + 1)..galaxies.len() {
            let (lg, rg) = (galaxies[l], galaxies[r]);
            p += usize::abs_diff(lg.0, rg.0) + usize::abs_diff(lg.1, rg.1);
        }
    }
    p.to_string()
}

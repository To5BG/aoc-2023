use std::collections::HashSet;
use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (mut seen, mut main_loop) = (HashSet::<(i32, i32)>::new(), HashSet::<(i32, i32)>::new());
    let (mut p1, mut p2, n) = (-1, 0, input.split_once('\n').unwrap().0.len() as i32);
    let (mut q, s) = (Vec::<(i32, i32)>::new(), input.find('S').unwrap());
    q.push((s as i32 / (n + 1), s as i32 % (n + 1)));
    while !q.is_empty() {
        let mut q_cloud = Vec::new();
        while let Some(curr) = q.pop() {
            for next in dirs(curr, &map) {
                if next.0 >= 0 && next.1 >= 0 && next.0 < n && next.1 < n && seen.insert(next) {
                    main_loop.insert(next);
                    q_cloud.push(next);
                }
            }
        }
        (p1, q) = (p1 + 1, q_cloud);
    }
    for i in 0..n {
        let mut inside = false;
        for j in 0..n {
            let c = map[i as usize][j as usize];
            if !main_loop.contains(&(i, j)) {
                p2 += inside as usize;
            } else if c == '|' || c == 'L' || c == 'J' {
                inside ^= true;
            }
        }
    }
    (p1.to_string(), p2.to_string(), t.elapsed())
}

fn dirs(st: (i32, i32), map: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    match map[st.0 as usize][st.1 as usize] {
        '-' => Vec::from([(st.0, st.1 - 1), (st.0, st.1 + 1)]),
        'L' => Vec::from([(st.0 - 1, st.1), (st.0, st.1 + 1)]),
        'J' => Vec::from([(st.0, st.1 - 1), (st.0 - 1, st.1)]),
        '|' => Vec::from([(st.0 - 1, st.1), (st.0 + 1, st.1)]),
        '7' => Vec::from([(st.0, st.1 - 1), (st.0 + 1, st.1)]),
        'F' => Vec::from([(st.0 + 1, st.1), (st.0, st.1 + 1)]),
        'S' => {
            let mut v = Vec::new();
            for dir in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
                let next = (st.0 + dir.0, st.1 + dir.1);
                if dirs(next, map).contains(&st) {
                    v.push(next);
                }
            }
            v
        }
        _ => Vec::new(),
    }
}

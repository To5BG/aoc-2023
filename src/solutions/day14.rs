use std::collections::HashMap;
use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (conf, conf_inv) = ((map.len() as i32 - 1, 0, 1), (0, map.len() as i32 - 1, -1));
    let (mut states, mut map2) = (HashMap::new(), map.clone());
    for cycle in 1..1000000000 {
        tilt(&mut map2, conf);
        tilt(&mut map2, conf_inv);
        if let Some(idx) = states.insert(map2.clone(), cycle) {
            if (1000000000 - cycle) % (cycle - idx) == 0 {
                break;
            }
        }
    }
    (pts(&mut map, Some(conf)), pts(&mut map2, None), t.elapsed())
}

fn tilt(map: &mut Vec<Vec<char>>, conf: (i32, i32, i32)) {
    for c in 0..map[0].len() {
        let (mut acc, mut r) = (0, conf.0);
        loop {
            match map[r as usize][c] {
                'O' => {
                    acc += 1;
                    map[r as usize][c] = '.';
                }
                '#' if acc != 0 => {
                    (0..acc).for_each(|i| map[(r + (i + 1) * conf.2) as usize][c] = 'O');
                    acc = 0;
                }
                _ => (),
            }
            if r == conf.1 {
                (0..acc).for_each(|i| map[(r + i * conf.2) as usize][c] = 'O');
                break;
            }
            r -= conf.2;
        }
    }
    for r in map {
        let (mut acc, mut c) = (0, conf.0);
        loop {
            match r[c as usize] {
                'O' => {
                    acc += 1;
                    r[c as usize] = '.';
                }
                '#' if acc != 0 => {
                    (0..acc).for_each(|i| r[(c + (i + 1) * conf.2) as usize] = 'O');
                    acc = 0;
                }
                _ => (),
            }
            if c == conf.1 {
                (0..acc).for_each(|i| r[(c + i * conf.2) as usize] = 'O');
                break;
            }
            c -= conf.2;
        }
    }
}

fn pts(map: &mut Vec<Vec<char>>, conf: Option<(i32, i32, i32)>) -> String {
    if let Some(c) = conf {
        tilt(map, c);
    }
    (0..map.len())
        .map(|r| {
            let mut acc = 0_usize;
            map[r].iter().for_each(|c| {
                if *c == 'O' {
                    acc += 1;
                }
            });
            (acc, map.len() - r)
        })
        .fold(0, |acc, e| acc + e.0 * e.1)
        .to_string()
}

use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let dims = (map.len() as i32, map[0].len() as i32);
    let p1 = simulate(&map, (0, 0, 0, 1), dims);
    let p2 = (0..map.len() as i32)
        .map(|i| {
            let left_start = (map.len() as i32, i, -1, 0);
            (simulate(&map, (0, i, 1, 0), dims)).max(simulate(&map, left_start, dims))
        })
        .reduce(usize::max)
        .unwrap()
        .max(
            (0..map[0].len() as i32)
                .map(|i| {
                    let right_start = (i, map.len() as i32, 0, -1);
                    (simulate(&map, (i, 0, 0, 1), dims)).max(simulate(&map, right_start, dims))
                })
                .reduce(usize::max)
                .unwrap(),
        );
    (p1.to_string(), p2.to_string(), t.elapsed())
}

fn simulate(map: &Vec<Vec<char>>, st: (i32, i32, i32, i32), dim: (i32, i32)) -> usize {
    let mut seen = vec![vec![[false; 4]; map[0].len()]; map.len()];
    let mut beams = Vec::from([st]);
    while !beams.is_empty() {
        let (mut bi, mut to_add) = (0, Vec::new());
        while bi < beams.len() {
            let curr = &mut beams[bi];
            if curr.0 < 0 || curr.0 >= dim.0 || curr.1 < 0 || curr.1 >= dim.1 {
                beams.remove(bi);
                continue;
            }
            let e = &mut seen[curr.0 as usize][curr.1 as usize]
                [((curr.2 * 2 + curr.3) as f32 * 0.75 + 2.0) as usize];
            if *e {
                beams.remove(bi);
                continue;
            }
            *e = true;
            match map[curr.0 as usize][curr.1 as usize] {
                '.' => (curr.0, curr.1) = (curr.0 + curr.2, curr.1 + curr.3),
                '-' if curr.2 == 0 => (curr.0, curr.1) = (curr.0 + curr.2, curr.1 + curr.3),
                '|' if curr.3 == 0 => (curr.0, curr.1) = (curr.0 + curr.2, curr.1 + curr.3),
                '/' => *curr = (curr.0 - curr.3, curr.1 - curr.2, -curr.3, -curr.2),
                '\\' => *curr = (curr.0 + curr.3, curr.1 + curr.2, curr.3, curr.2),
                '-' => {
                    to_add.push((curr.0, curr.1 + curr.2, curr.3, curr.2));
                    *curr = (curr.0, curr.1 - curr.2, curr.3, -curr.2);
                }
                '|' => {
                    to_add.push((curr.0 + curr.3, curr.1, curr.3, curr.2));
                    *curr = (curr.0 - curr.3, curr.1, -curr.3, curr.2);
                }
                _ => unreachable!(),
            }
            bi += 1;
        }
        beams.extend(to_add);
    }
    seen.into_iter()
        .map(|l| l.into_iter().filter(|s| s.iter().any(|ss| *ss)).count())
        .sum::<usize>()
}

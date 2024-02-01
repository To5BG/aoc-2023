use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

const DIRMAP: &[usize] = &[0, 1, 0, 3, 2];

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    (find_cost(&map, 1, 3), find_cost(&map, 4, 10), t.elapsed())
}

pub fn find_cost(map: &[Vec<u32>], min_step: isize, max_step: isize) -> String {
    let end = ((map.len() - 1), (map[0].len() - 1));
    let mut pq = BinaryHeap::from_iter([(Reverse(0), ((0, 0), 0))]);
    let mut dist = vec![vec![[u32::MAX; 4]; map[0].len()]; map.len()];
    (0..4).for_each(|i| dist[0][0][i] = 0);
    while let Some((c, (curr, par_dir))) = pq.pop() {
        if end == curr {
            return c.0.to_string();
        }
        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next_dir = DIRMAP[((dir.0 + 1) * 2 + dir.1) as usize];
            if curr != (0, 0) && (par_dir + next_dir) % 2 == 0 {
                continue;
            }
            let (mut new_cost, mut next) = (c.0, curr);
            for step in 1..=max_step {
                if let Some(nextx) = next.0.checked_add_signed(dir.0) {
                    if let Some(nexty) = next.1.checked_add_signed(dir.1) {
                        next = (nextx, nexty);
                        if nextx >= map.len() || nexty >= map[0].len() {
                            break;
                        }
                        new_cost += map[nextx][nexty];
                        if step >= min_step && new_cost < dist[nextx][nexty][next_dir] {
                            dist[nextx][nexty][next_dir] = new_cost;
                            pq.push((Reverse(new_cost), (next, next_dir)));
                        }
                    }
                }
            }
        }
    }
    u32::MAX.to_string()
}

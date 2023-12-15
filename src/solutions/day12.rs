use std::collections::HashMap;
use std::time::{Duration, Instant};

pub fn solve(input: &str) -> (String, String, Duration) {
    let t = Instant::now();
    let mut map = HashMap::<usize, usize>::new();
    let (p1, p2) = input
        .lines()
        .map(|l| {
            let (_v, _r) = l.split_once(' ').unwrap();
            let springs = _v.chars().collect::<Vec<char>>();
            let steps = _r
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            map.clear();
            let _p1 = dp(&mut map, &springs, &steps, None);
            let large_springs = ("?".to_string() + _v).repeat(5)[1..]
                .chars()
                .collect::<Vec<char>>();
            let large_steps = (",".to_string() + _r).repeat(5)[1..]
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            map.clear();
            let _p2 = dp(&mut map, &large_springs, &large_steps, None);
            (_p1, _p2)
        })
        .fold((0, 0), |acc, e| (acc.0 + e.0, acc.1 + e.1));
    (p1.to_string(), p2.to_string(), t.elapsed())
}

fn dp(
    mem: &mut HashMap<usize, usize>,
    springs: &[char],
    steps: &[usize],
    ext: Option<usize>,
) -> usize {
    if steps.is_empty() && ext.is_some() {
        return 0;
    }
    if springs.is_empty() {
        return match (steps.len(), ext) {
            (0, None) => 1,
            (1, Some(x)) if x == steps[0] => 1,
            _ => 0,
        };
    }
    let k = 10000 * ext.unwrap_or(0) + 100 * springs.len() + steps.len();
    if let Some(&prev) = mem.get(&k) {
        return prev;
    }
    let res = match (springs.first().unwrap(), ext) {
        ('.', None) => dp(mem, &springs[1..], steps, None),
        ('.', Some(n)) if n == steps[0] => dp(mem, &springs[1..], &steps[1..], None),
        ('.', Some(_)) => 0,
        ('?', None) => dp(mem, &springs[1..], steps, Some(1)) + dp(mem, &springs[1..], steps, None),
        ('?', Some(n)) if n == steps[0] => {
            dp(mem, &springs[1..], &steps[1..], None) + dp(mem, &springs[1..], steps, Some(n + 1))
        }
        ('#', None) => dp(mem, &springs[1..], steps, Some(1)),
        ('#' | '?', Some(n)) => dp(mem, &springs[1..], steps, Some(n + 1)),
        _ => unreachable!(),
    };
    mem.insert(k, res);
    res
}

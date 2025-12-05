use aoc2025::day02::Interval;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();
    let lines = str.split(",").collect::<Vec<&str>>();
    let intervals: Vec<Interval> = lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    // 11111   121212
    let sum: u64 = intervals
        .iter()
        .flat_map(|i| i.start..=i.end)
        .filter(|&n| {
            let s = n.to_string();
            let len = s.len();
            if len % 2 != 0 {
                return false;
            }
            let first = &s[..len / 2];
            let second = &s[len / 2..];
            first == second
        })
        .sum();

    dbg!(&sum);
}

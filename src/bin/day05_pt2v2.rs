use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();
    let (intervals_str, _) = str.split_once("\n\n").unwrap();

    let mut intervals: Vec<(u64, u64)> = intervals_str
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    intervals.sort_unstable_by_key(|k| k.0);

    let merged = intervals
        .into_iter()
        .fold(Vec::new(), |mut acc: Vec<(u64, u64)>, interval| {
            match acc.last_mut() {
                Some(last) if interval.0 <= last.1 + 1 => {
                    last.1 = last.1.max(interval.1);
                }
                _ => {
                    acc.push(interval);
                }
            }
            acc
        });

    let sum: u64 = merged.iter().map(|(s, e)| e - s + 1).sum();

    println!("{}", sum);
}

use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();
    let (intervals, _) = str.split_once("\n\n").unwrap();

    let mut intervals: Vec<(u64, u64)> = intervals
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    let mut finally_intervals: Vec<(u64, u64)> = vec![];

    for interval in intervals {
        let (start, end) = interval;

        let overlapping_indices: Vec<usize> = finally_intervals
            .iter()
            .enumerate()
            .filter(|(_, (s, e))| start >= *s && start <= *e)
            .map(|(i, _)| i)
            .collect();

        if let Some(&idx) = overlapping_indices.first() {
            if let Some(existing) = finally_intervals.get_mut(idx) {
                existing.0 = existing.0.min(start);
                existing.1 = existing.1.max(end);
            }
        } else {
            finally_intervals.push(interval);
        }
    }

    let sum = finally_intervals
        .iter()
        .map(|(s, e)| e - s + 1)
        .sum::<u64>();

    println!("{}", sum);
}

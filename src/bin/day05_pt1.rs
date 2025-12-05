use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();
    let (intervals, numbers) = str.split_once("\n\n").unwrap();

    let intervals: Vec<(u64, u64)> = intervals
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let numbers: Vec<u64> = numbers.lines().map(|line| line.parse().unwrap()).collect();
    let mut count = 0;
    for number in numbers {
        let mut in_interval = false;
        for (a, b) in intervals.iter() {
            if number >= *a && number <= *b {
                in_interval = true;
                break;
            }
        }
        if in_interval {
            count += 1;
        }
    }
    println!("{}", count);
}

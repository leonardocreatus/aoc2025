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

    // https://algo.monster/liteproblems/459
    // https://medium.com/@_monitsharma/daily-leetcode-problems-problem-459-repeated-substring-pattern-4af7f9bb0723

    // 121212
    // 2121212121
    let sum: u64 = intervals
        .iter()
        .flat_map(|i| i.start..=i.end)
        .filter(|&n| {
            let s = n.to_string();
            let doubled = format!("{}{}", s, s);
            let search_area = &doubled[1..doubled.len() - 1];
            search_area.contains(&s)
        })
        .sum();

    dbg!(&sum);
}

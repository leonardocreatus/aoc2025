use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let content = fs::read_to_string(filename).unwrap();
    let mut lines: Vec<&str> = content.trim().lines().collect();

    let operations = lines
        .pop()
        .expect("Input must have at least one line")
        .split_whitespace();

    let numbers: Vec<Vec<u64>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let total: u64 = operations
        .enumerate()
        .map(|(i, op)| {
            let col_iter = numbers.iter().map(|row| row[i]);
            match op {
                "*" => col_iter.product(),
                "+" => col_iter.sum(),
                _ => 0,
            }
        })
        .sum();

    println!("{}", total);
}

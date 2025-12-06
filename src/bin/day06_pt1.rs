use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();

    let mut lines: Vec<&str> = str.trim().lines().collect();

    let operations = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let numbers: Vec<Vec<u64>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let mut total: u64 = 0;
    for i in 0..operations.len() {
        let op = operations[i];
        let mut result: u64 = match op {
            "*" => 1,
            "+" => 0,
            _ => 0,
        };
        for j in 0..numbers.len() {
            let value = numbers[j][i];
            match op {
                "*" => result *= value,
                "+" => result += value,
                _ => {}
            }
        }
        total += result;
    }
    println!("{}", total);
}

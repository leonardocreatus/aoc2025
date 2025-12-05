use aoc2025::day01::Input;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();
    let lines = str.split("\n").collect::<Vec<&str>>();
    let inputs: Vec<Input> = lines
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut value: u32 = 50;
    let mut count_zeros = 0;
    for input in inputs {
        match input.direction {
            'L' => {
                let step = input.steps % 100;
                if step > value {
                    value += 100;
                }
                value -= step;
            }
            'R' => {
                value += input.steps;
            }
            _ => panic!("Invalid direction"),
        }
        value %= 100;
        if value == 0 {
            count_zeros += 1;
        }
    }
    println!("Value: {}", value);
    println!("Count zeros: {}", count_zeros);
}

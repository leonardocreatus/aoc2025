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

    let mut value: i64 = 50;
    let mut count_zeros: i64 = 0;

    for input in inputs {
        let old_zeros = count_zeros;
        let old_value = value;
        let rotations = input.steps.div_euclid(100);
        let steps: i64 = input.steps.rem_euclid(100).into();

        if input.direction == 'L' && (steps > value && value != 0) {
            count_zeros += 1;
        }

        if input.direction == 'R' && (steps > (100 - value)) {
            count_zeros += 1;
        }

        match input.direction {
            'L' => value -= steps as i64,
            'R' => value += steps as i64,
            _ => panic!("Invalid direction"),
        }

        count_zeros += rotations as i64;

        if value % 100 == 0 {
            count_zeros += 1;
        }

        value = value.rem_euclid(100);

        println!(
            "[{}] {}, {}{} -> {}",
            old_zeros, old_value, input.direction, input.steps, value,
        )
    }

    println!(" >> {}", count_zeros);
}

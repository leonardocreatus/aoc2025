use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;
struct Machine {
    target: u32,
    bottons: Vec<u32>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splites = s.split_whitespace();
        let target_str = splites.next().unwrap().trim_matches('[').trim_matches(']');
        let target = target_str
            .chars()
            .enumerate()
            .fold(0, |acc, (i, c)| if c == '#' { acc | (1 << i) } else { acc });

        let _ = splites.next_back().unwrap();
        let bottons = splites
            .map(|s| {
                s.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .fold(0, |acc, n| acc | (1 << n))
            })
            .collect::<Vec<u32>>();
        Ok(Machine { target, bottons })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "input.txt"
    };

    let contents = fs::read_to_string(filename).expect("Erro ao ler arquivo");
    let machines = contents
        .lines()
        .map(|line| line.parse::<Machine>().unwrap())
        .collect::<Vec<Machine>>();

    let res = machines
        .iter()
        .map(|m| {
            let mut pressed: Vec<u32> = m.bottons.clone();
            let mut it = 1;

            while !pressed.contains(&m.target) {
                let mut new_pressed = Vec::new();
                for value in pressed.iter() {
                    for button in m.bottons.iter() {
                        new_pressed.push(value ^ button);
                    }
                }
                pressed = new_pressed;
                it += 1;
            }
            it
        })
        .sum::<u32>();
    println!("Result: {}", res);
}

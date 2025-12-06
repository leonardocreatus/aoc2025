use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();

    let mut lines: Vec<&str> = str.trim().lines().collect();

    let operations = lines.pop().unwrap().chars().collect::<Vec<char>>();

    let operations_position = operations
        .iter()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(i, _)| i)
        .rev()
        .collect::<Vec<usize>>();

    let operations = operations
        .into_iter()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

    let inputs: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| {
            let mut parts = Vec::new();
            let mut last_pos = line.len() + 1;
            for &pos in &operations_position {
                let str = &line[pos..last_pos - 1];
                parts.push(str);
                last_pos = pos;
            }
            parts.reverse();
            parts
        })
        .collect();

    let transposed_inputs: Vec<Vec<String>> = (0..inputs[0].len())
        .map(|i| inputs.iter().map(|line| line[i].to_owned()).collect())
        .collect();

    let mut pos = 0;
    let mut res = 0;
    for values in transposed_inputs {
        let qtd = values.get(0).unwrap().len();

        let mut numbers = vec![String::new(); qtd];

        for i in 0..qtd {
            for v in values.iter() {
                numbers[i].push(v.chars().skip(i).next().unwrap());
            }
        }

        let initial = if operations.get(pos).copied() == Some('+') {
            0
        } else {
            1
        };

        let sum = numbers
            .iter()
            .map(|s| s.trim().parse::<u64>().unwrap())
            .fold(initial, |acc, x| {
                if operations.get(pos).copied() == Some('+') {
                    acc + x
                } else {
                    acc * x
                }
            });

        res += sum;
        pos += 1;
    }
    println!("{}", res);
}

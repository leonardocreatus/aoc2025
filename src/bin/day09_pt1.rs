use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} <input_file>", args[0]);
        return;
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Erro ao ler arquivo");
    let lines: Vec<&str> = contents.lines().collect();

    if lines.is_empty() {
        return;
    }

    let points: Vec<(i64, i64)> = lines
        .iter()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let greater_area = points
        .into_iter()
        .tuple_combinations()
        .map(|(p1, p2)| ((p1.0 - p2.0 + 1) * (p1.1 - p2.1 + 1)).abs())
        .max()
        .unwrap_or(0);

    println!("{}", greater_area);
}

use std::collections::HashMap;
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

    let start_pos = lines[0].find('S').expect("No 'S' found in first line");

    let mut current_counts: HashMap<usize, u64> = HashMap::new();
    current_counts.insert(start_pos, 1);
    for line in &lines[1..] {
        let mut next_counts: HashMap<usize, u64> = HashMap::new();
        for (&pos, &count) in &current_counts {
            let char_at_pos = line.chars().nth(pos).unwrap_or('.');

            if char_at_pos == '^' {
                if pos > 0 {
                    *next_counts.entry(pos - 1).or_insert(0) += count;
                }
                if pos < line.len() - 1 {
                    *next_counts.entry(pos + 1).or_insert(0) += count;
                }
            } else {
                *next_counts.entry(pos).or_insert(0) += count;
            }
        }
        current_counts = next_counts;
        if current_counts.is_empty() {
            break;
        }
    }
    let total_timelines: u64 = current_counts.values().sum();

    println!("Total timelines: {}", total_timelines);
}

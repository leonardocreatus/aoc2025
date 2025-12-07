use std::collections::HashSet;
use std::env;
use std::fs;

fn trace_path(pos: usize, diagram: &[&str]) -> usize {
    if diagram.is_empty() {
        return 1;
    }

    let line = diagram[0];
    let splitters_pos: HashSet<usize> = line.match_indices('^').map(|(i, _)| i).collect();

    if splitters_pos.contains(&pos) {
        let right = if pos < line.len() - 1 {
            trace_path(pos + 1, &diagram[1..])
        } else {
            0
        };
        let left = if pos > 0 {
            trace_path(pos - 1, &diagram[1..])
        } else {
            0
        };

        return left + right;
    }
    trace_path(pos, &diagram[1..])
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    if lines.is_empty() {
        return;
    }

    let start_pos = lines[0].find('S').expect("No 'S' found in first line");

    let result = trace_path(start_pos, &lines[1..]);

    println!("{}", result);
}

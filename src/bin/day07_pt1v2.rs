use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let mut lines = contents.lines();

    let start_pos = lines
        .next()
        .and_then(|line| line.find('S'))
        .expect("NO 'S' found in first line");

    let total_splits: usize = lines
        .scan(HashSet::from([start_pos]), |positions, line| {
            if positions.is_empty() {
                return None;
            }

            let splitters: HashSet<usize> = line.match_indices('^').map(|(i, _)| i).collect();
            let mut hits = 0;

            let current_positions: Vec<usize> = positions.iter().copied().collect();

            for pos in current_positions {
                if splitters.contains(&pos) {
                    hits += 1;
                    positions.remove(&pos);

                    if pos > 0 {
                        positions.insert(pos - 1);
                    }
                    if pos < line.len() - 1 {
                        positions.insert(pos + 1);
                    }
                }
            }

            Some(hits)
        })
        .sum();

    println!("{}", total_splits);
}

use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let content = fs::read_to_string(filename).unwrap();
    let content = content.trim();

    let mut lines: Vec<&str> = content.lines().collect();

    let operations_line = lines.pop().expect("File must have at least one line");

    let op_indices: Vec<usize> = operations_line
        .match_indices(|c: char| !c.is_whitespace())
        .map(|(i, _)| i)
        .collect();

    let operations: Vec<char> = operations_line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let mut ranges = Vec::new();
    for i in 0..op_indices.len() {
        let start = op_indices[i];
        let end = if i + 1 < op_indices.len() {
            op_indices[i + 1] - 1
        } else {
            usize::MAX
        };
        ranges.push(start..end);
    }

    let mut total_sum: u64 = 0;

    for (op_idx, &start_col) in op_indices.iter().enumerate() {
        let op = operations[op_idx];

        let end_col_limit = if op_idx + 1 < op_indices.len() {
            op_indices[op_idx + 1] - 1
        } else {
            usize::MAX
        };

        let block_width = if end_col_limit == usize::MAX {
            lines[0].len() - start_col
        } else {
            end_col_limit - start_col
        };

        let mut block_numbers: Vec<String> = vec![String::with_capacity(lines.len()); block_width];

        for line in &lines {
            let line_chars: Vec<char> = line.chars().collect();
            for i in 0..block_width {
                if let Some(&c) = line_chars.get(start_col + i) {
                    block_numbers[i].push(c);
                }
            }
        }

        let initial_val = if op == '+' { 0 } else { 1 };

        let block_result = block_numbers
            .iter()
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.parse::<u64>().unwrap())
                }
            })
            .fold(
                initial_val,
                |acc, x| {
                    if op == '+' { acc + x } else { acc * x }
                },
            );

        total_sum += block_result;
    }

    println!("{}", total_sum);
}

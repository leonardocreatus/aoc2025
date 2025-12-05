use std::env;
use std::fs;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<char>> = str
        .par_lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let h = grid.len();
    let w = grid[0].len();

    let sum = grid
        .par_iter()
        .enumerate()
        .flat_map(|(r, row)| row.par_iter().enumerate().map(move |(c, &ch)| (r, c, ch)))
        .filter(|(r, c, ch)| {
            if *ch != '@' {
                return false;
            }

            let neighbors = get_3x3_window(*r, *c, h, w);
            let at_count = neighbors
                .iter()
                .filter(|&&(nr, nc)| grid[nr][nc] == '@')
                .count();
            at_count < 4
        })
        .count();

    println!("{}", sum);
}

fn get_3x3_window(r: usize, c: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for dr in -1..=1 {
        for dc in -1..=1 {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0
                && nr < h as isize
                && nc >= 0
                && nc < w as isize
                && (nr != r as isize || nc != c as isize)
            {
                res.push((nr as usize, nc as usize));
            }
        }
    }
    res
}

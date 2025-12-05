use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();
    let mut grid: Vec<Vec<char>> = str
        .par_lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let h = grid.len();
    let w = grid[0].len();

    let mut coordinates = grid
        .par_iter()
        .enumerate()
        .flat_map(|(r, row)| row.par_iter().enumerate().map(move |(c, &ch)| (r, c, ch)))
        .filter(|(_, _, ch)| *ch == '@')
        .collect::<Vec<_>>();

    let neighbors: HashMap<(usize, usize), Vec<(usize, usize)>> = coordinates
        .par_iter()
        .map(|(r, c, _)| ((*r, *c), get_3x3_window(*r, *c, h, w)))
        .collect();

    let mut removed: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let removed_count = removed.len();
        for (r, c, _) in coordinates.iter() {
            let n = neighbors.get(&(*r, *c)).unwrap();
            let at_count = n.iter().filter(|&&(nr, nc)| grid[nr][nc] == '@').count();
            if at_count < 4 {
                removed.insert((*r, *c));
            }
        }

        for (r, c) in &removed {
            grid[*r][*c] = '.';
        }

        coordinates.retain(|(r, c, _)| !removed.contains(&(*r, *c)));
        if removed_count == removed.len() {
            break;
        }
    }

    println!("{}", removed.len());
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

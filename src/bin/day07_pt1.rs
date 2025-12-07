use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();

    let splits = str
        .lines()
        .fold((HashSet::<usize>::new(), 0), |(mut acc, mut sum), line| {
            if acc.is_empty() {
                return (
                    HashSet::from([line.match_indices("S").next().unwrap().0]),
                    sum,
                );
            }

            let pos_with_splitter: HashSet<usize> =
                line.match_indices('^').map(|(i, _)| i).collect();

            let intersection: HashSet<usize> =
                acc.intersection(&pos_with_splitter).map(|&x| x).collect();

            if !intersection.is_empty() {
                for &pos in intersection.iter() {
                    acc.remove(&pos);
                    if pos < line.len() - 1 {
                        acc.insert(pos + 1);
                    }

                    if pos > 0 {
                        acc.insert(pos - 1);
                    }
                }
                sum += intersection.len();
            }

            (acc, sum)
        });

    println!("{}", splits.1);
}

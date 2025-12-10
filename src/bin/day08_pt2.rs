use itertools::Itertools;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Point { x, y, z })
    }
}

impl Point {
    fn distance(&self, other: &Point) -> u64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let dz = self.z as f64 - other.z as f64;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

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

    let points: Vec<Point> = lines.iter().map(|line| line.parse().unwrap()).collect();

    let mut distances_per_points: HashMap<u64, Vec<(Point, Point)>> = HashMap::new();

    for (p1, p2) in points.iter().tuple_combinations() {
        let distance = p1.distance(p2);
        distances_per_points
            .entry(distance)
            .or_insert_with(Vec::new)
            .push((*p1, *p2));
    }

    let mut distances = distances_per_points.keys().collect::<Vec<_>>();
    distances.sort();

    let mut circuits: Vec<HashSet<Point>> = points.iter().map(|p| HashSet::from([*p])).collect();
    let (mut a, mut b): (Option<Point>, Option<Point>) = (None, None);

    for distance in distances {
        // println!("Press Enter to continue...");
        // std::io::stdin().read_line(&mut String::new()).unwrap();

        let points = distances_per_points.get(&distance).unwrap();
        for (p1, p2) in points {
            let mut idx_a = None;
            let mut idx_b = None;

            for (i, circuit) in circuits.iter().enumerate() {
                if circuit.contains(&p1) {
                    idx_a = Some(i);
                }
                if circuit.contains(&p2) {
                    idx_b = Some(i);
                }
            }

            match (idx_a, idx_b) {
                (Some(ia), Some(ib)) => {
                    if ia != ib {
                        a = Some(*p1);
                        b = Some(*p2);
                        let (min_i, max_i) = if ia < ib { (ia, ib) } else { (ib, ia) };
                        let circuit_to_merge = circuits.remove(max_i);
                        circuits[min_i].extend(circuit_to_merge);
                    }
                }
                _ => {}
            }
        }
    }

    // len_circuits.sort();
    // len_circuits.reverse();
    // let total_points = len_circuits[0] * len_circuits[1] * len_circuits[2];

    println!("Total: {}", a.unwrap().x * b.unwrap().x);
}

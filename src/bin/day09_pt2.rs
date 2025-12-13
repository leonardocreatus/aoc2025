use itertools::Itertools;
use std::env;
use std::fs;

type P = (i64, i64);

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "input.txt"
    };

    let contents = fs::read_to_string(filename).expect("Erro ao ler arquivo");

    let points: Vec<P> = contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    if points.is_empty() {
        return;
    }

    let mut edges = Vec::new();
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        edges.push((p1, p2));
    }

    let mut candidates: Vec<(i64, P, P)> = points
        .iter()
        .tuple_combinations()
        .map(|(&p1, &p2)| {
            let width = (p1.0 - p2.0).abs() + 1;
            let height = (p1.1 - p2.1).abs() + 1;
            (width * height, p1, p2)
        })
        .collect();

    candidates.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    println!("Total de candidatos a analisar: {}", candidates.len());

    for (area, p1, p2) in candidates {
        let min_x = p1.0.min(p2.0);
        let max_x = p1.0.max(p2.0);
        let min_y = p1.1.min(p2.1);
        let max_y = p1.1.max(p2.1);

        if is_valid_rectangle(min_x, max_x, min_y, max_y, &edges) {
            println!("Maior área encontrada: {}", area);
            return;
        }
    }
}

fn is_valid_rectangle(rx1: i64, rx2: i64, ry1: i64, ry2: i64, edges: &[(P, P)]) -> bool {
    let mid_x = (rx1 as f64 + rx2 as f64) / 2.0;
    let mid_y = (ry1 as f64 + ry2 as f64) / 2.0;

    let mut inside = false;
    for (p1, p2) in edges {
        let (x1, y1) = (p1.0 as f64, p1.1 as f64);
        let (x2, y2) = (p2.0 as f64, p2.1 as f64);

        if ((y1 > mid_y) != (y2 > mid_y)) && (mid_x < (x2 - x1) * (mid_y - y1) / (y2 - y1) + x1) {
            inside = !inside;
        }
    }
    if !inside {
        return false;
    }

    for (p1, p2) in edges {
        let ex1 = p1.0.min(p2.0);
        let ex2 = p1.0.max(p2.0);
        let ey1 = p1.1.min(p2.1);
        let ey2 = p1.1.max(p2.1);

        if ex1 == ex2 {
            if ex1 > rx1 && ex1 < rx2 {
                let overlap_min = ey1.max(ry1);
                let overlap_max = ey2.min(ry2);
                if overlap_min < overlap_max {
                    return false;
                }
            }
        } else if ey1 == ey2 {
            if ey1 > ry1 && ey1 < ry2 {
                let overlap_min = ex1.max(rx1);
                let overlap_max = ex2.min(rx2);
                if overlap_min < overlap_max {
                    return false;
                }
            }
        }
    }

    true
}

use std::collections::HashMap;
use std::env;
use std::fs;

type Memo = HashMap<String, u64>;
type Graph = HashMap<String, Vec<String>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "input.txt"
    };

    let contents = fs::read_to_string(filename).expect("Erro ao ler arquivo");

    let graph: Graph = contents.lines().fold(HashMap::new(), |mut acc, line| {
        if let Some((device, outputs)) = line.split_once(": ") {
            let targets: Vec<String> = outputs.split_whitespace().map(|s| s.to_string()).collect();
            acc.insert(device.to_string(), targets);
        }
        acc
    });

    let mut memo: Memo = HashMap::new();

    let total_paths = count_paths("you", "out", &graph, &mut memo);

    println!("Result: {}", total_paths);
}

fn count_paths(current: &str, target: &str, graph: &Graph, memo: &mut Memo) -> u64 {
    if current == target {
        return 1;
    }

    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total += count_paths(neighbor, target, graph, memo);
        }
    }

    memo.insert(current.to_string(), total);

    total
}

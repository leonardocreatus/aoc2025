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

    let mut memo_dac_fft: Memo = HashMap::new();
    let mut memo_fft_dac: Memo = HashMap::new();
    let mut memo_svr_fft: Memo = HashMap::new();
    let mut memo_svr_dac: Memo = HashMap::new();
    let mut memo_fft_out: Memo = HashMap::new();
    let mut memo_dac_out: Memo = HashMap::new();

    let paths_dac_fft = count_paths("dac", "fft", &graph, &mut memo_dac_fft);
    let paths_fft_dac = count_paths("fft", "dac", &graph, &mut memo_fft_dac);
    let paths_svr_fft = count_paths("svr", "fft", &graph, &mut memo_svr_fft);
    let paths_svr_dac = count_paths("svr", "dac", &graph, &mut memo_svr_dac);
    let paths_fft_out = count_paths("fft", "out", &graph, &mut memo_fft_out);
    let paths_dac_out = count_paths("dac", "out", &graph, &mut memo_dac_out);

    let svr_dac_fft_out = paths_svr_dac * paths_dac_fft * paths_fft_out;
    let svr_fft_dac_out = paths_svr_fft * paths_fft_dac * paths_dac_out;

    let total_paths = svr_dac_fft_out + svr_fft_dac_out;

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

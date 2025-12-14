use std::env;
use std::fs;
use std::str::FromStr;
use z3::{Optimize, SatResult, ast::Int};

#[derive(Debug)]
struct Machine {
    target: Vec<usize>,
    bottons: Vec<Vec<usize>>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(format!("Linha de dados incompleta ou inválida: {}", s));
        }

        let data_parts = &parts[1..];

        let target_str = data_parts.last().ok_or("Faltando alvo")?;
        let bottons_str = &data_parts[0..data_parts.len() - 1];

        let target = target_str
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|n| {
                n.parse::<usize>()
                    .map_err(|e| format!("Erro ao analisar target: {}", e))
            })
            .collect::<Result<Vec<usize>, String>>()?;

        let bottons = bottons_str
            .iter()
            .filter(|s| !s.starts_with('[') && !s.starts_with(']'))
            .map(|s| {
                s.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|n| {
                        n.parse::<usize>()
                            .map_err(|e| format!("Erro ao analisar botão: {}", e))
                    })
                    .collect::<Result<Vec<usize>, String>>()
            })
            .collect::<Result<Vec<Vec<usize>>, String>>()?;

        if target.is_empty() || bottons.is_empty() {
            return Err(format!("Botões ou alvo vazios após parsing: {}", s));
        }

        Ok(Machine { target, bottons })
    }
}

fn solve_machine_z3(m: &Machine) -> Result<usize, String> {
    let opt = Optimize::new();

    let num_counters = m.target.len();
    let num_buttons = m.bottons.len();

    let x_vars: Vec<Int> = (0..num_buttons)
        .map(|j| Int::new_const(format!("X_{}", j)))
        .collect();

    for x in x_vars.iter() {
        opt.assert(&x.ge(&Int::from_i64(0)));
    }

    for i in 0..num_counters {
        let mut lhs_components = Vec::new();

        for j in 0..num_buttons {
            let b_ij = m.bottons[j].iter().filter(|&&pos| pos == i).count() as i64;

            if b_ij > 0 {
                let b_ij_ast = Int::from_i64(b_ij);
                let term = b_ij_ast * x_vars[j].clone();
                lhs_components.push(term);
            }
        }

        let lhs = lhs_components
            .iter()
            .fold(Int::from_i64(0), |acc, x| acc + x);
        let t_i = m.target[i] as i64;
        let rhs = Int::from_i64(t_i);

        opt.assert(&lhs.eq(&rhs));
    }

    let cost_sum = x_vars.iter().fold(Int::from_i64(0), |acc, x| acc + x);

    opt.minimize(&cost_sum);

    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();

            let min_cost_result = model.eval(&cost_sum, true).unwrap();

            let cost = min_cost_result
                .as_i64()
                .ok_or("Erro ao converter custo para i64")? as usize;

            Ok(cost)
        }
        SatResult::Unsat => Err("O alvo é inalcançável (sistema insatisfazível).".to_string()),
        SatResult::Unknown => {
            Err("O Z3 não conseguiu determinar a satisfatibilidade (Unknown).".to_string())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "input.txt"
    };

    let contents = fs::read_to_string(filename).expect("Erro ao ler arquivo");
    let machines = contents
        .lines()
        .map(|line| line.parse::<Machine>().unwrap())
        .collect::<Vec<Machine>>();

    let total_machines = machines.len();
    println!("Total de máquinas a processar: {}", total_machines);

    let mut total_presses = 0;

    for (idx, m) in machines.iter().enumerate() {
        match solve_machine_z3(m) {
            Ok(cost) => {
                total_presses += cost;
                println!(
                    "\rMáquina {}/{} resolvida. Custo: {}",
                    idx + 1,
                    total_machines,
                    cost
                );
            }
            Err(e) => {
                eprintln!("\nErro ao resolver Máquina {}: {}", idx + 1, e);
            }
        }
    }

    println!("\n--- Processamento Concluído ---");
    println!(
        "Resultado Final (Total de Pressionamentos): {}",
        total_presses
    );
}

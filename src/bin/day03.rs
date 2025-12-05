use rayon::prelude::*;
use std::{env, fs};

use chrono::Utc;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let str = fs::read_to_string(filename).unwrap();

    let start_time = Utc::now();
    const LEN: usize = 12;
    let result = str
        .par_lines()
        .map(|line| {
            let mut digits_iter = line.bytes().map(|b| (b - b'0') as u64).rev();

            let mut greater = [0u64; LEN];
            for i in 0..LEN {
                if let Some(d) = digits_iter.next() {
                    greater[i] = d;
                }
            }

            greater.reverse();

            for digit in digits_iter {
                let mut carry = digit;
                for slot in greater.iter_mut() {
                    if carry >= *slot {
                        std::mem::swap(slot, &mut carry);
                    } else {
                        break;
                    }
                }
            }

            greater.iter().fold(0, |acc, &digit| acc * 10 + digit)
        })
        .sum::<u64>();

    let end_time = Utc::now();
    let duration = end_time.signed_duration_since(start_time);
    println!("Duration: {:?}", duration.num_nanoseconds());
    println!("Result: {}", result);
}

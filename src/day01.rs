use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// part1=73211
// part2=213958
fn part1(input: &mut dyn BufRead) -> String {
    let mut sum: u32 = 0;
    let mut max = 0;

    // ? use successors
    for line in get_string_from_input(input).lines() {
        if line == "" {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += line.trim().parse::<u32>().unwrap();
        }
    }

    max.to_string()
}

fn part2(input: &mut dyn BufRead) -> String {
    let mut sum: u32 = 0;
    let mut maxes: [u32; 3] = [0, 0, 0];

    // ? CSDR do it idiomatically functionally
    for line in get_string_from_input(input).lines() {
        if line == "" {
            let mut max_less_than_sum = None;

            for (idx, &item) in maxes.iter().enumerate() {
                match max_less_than_sum {
                    None if item < sum => {
                        max_less_than_sum = Some((item, idx));
                    }
                    Some((max_val, _)) if item > max_val && item < sum => {
                        max_less_than_sum = Some((item, idx));
                    }
                    _ => {}
                }
            }

            if let Some((_, max_index)) = max_less_than_sum {
                maxes[max_index] = sum;
            }
            sum = 0;
        } else {
            let x = line.trim().parse::<u32>().unwrap();
            sum += x;
        }
    }

    let total = maxes.iter().sum::<u32>();
    total.to_string()
}

pub const SOLVERS: &[Solver] = &[part1, part2];

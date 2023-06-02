use crate::Solver;
use std::{collections::HashSet, io::BufRead};

// part1 != 1357 too low (off by 1 error?) , != 1358 (forgot to return position of last char in chunk)
// part1 == 1361
// part2 == 3263

fn part1(input: &mut dyn BufRead) -> String {
    let n_unique_chars = 4;
    find_position_unique_marker(input, n_unique_chars).to_string()
}

fn part2(input: &mut dyn BufRead) -> String {
    let n_unique_chars = 14;
    find_position_unique_marker(input, n_unique_chars).to_string()
}

fn find_position_unique_marker(input: &mut dyn BufRead, n_unique_chars: usize) -> usize {
    let mut data = vec![];
    input.read_to_end(&mut data).unwrap();
    let data_str = String::from_utf8(data).expect("Data isnt valid UTF-8");
    for (i, chunk) in data_str
        .chars()
        .collect::<Vec<char>>()
        .windows(n_unique_chars)
        .enumerate()
    {
        let mut seen = HashSet::new();
        let mut mchunk = chunk.to_vec();
        mchunk.retain(|c| seen.insert(*c));
        if mchunk.len() == n_unique_chars {
            // println!("{} : {:?}", i + n_unique_chars, mchunk);
            return i + n_unique_chars;
        }
    }
    1
}

pub const SOLVERS: &[Solver] = &[part1, part2];

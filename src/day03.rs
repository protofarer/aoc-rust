use crate::{get_string_from_input, Solver};
use std::{collections::HashSet, io::BufRead};

// part1
// fail for answer 10439 using non_unique misplaced, too high
// 948 too low using unique misplaced
// 7793 identifying only unique misplaced items per rucksack
// part2 = 2499
fn part1(input: &mut dyn BufRead) -> String {
    let mut all_misplaced = Vec::new();

    // parse each line into a map where: letter => key, (0,0) values, 1 if found for corresponding compartment
    for line in get_string_from_input(input).lines() {
        let mut rucksack_misplaced = HashSet::new(); // misplaced items are uniquely identified per rucksack
        let mut compart = HashSet::new();
        let idx_split = line.len() / 2 - 1;

        for c in line.chars().take(idx_split + 1) {
            compart.insert(c);
        }

        for c in line.chars().skip(idx_split + 1) {
            if compart.contains(&c) {
                rucksack_misplaced.insert(c);
            }
        }
        all_misplaced.extend(rucksack_misplaced);
    }
    let sum_priorities_nonunique = all_misplaced
        .iter()
        .fold(0, |acc, curr| acc + priority(curr));
    sum_priorities_nonunique.to_string()
}

fn part2(input: &mut dyn BufRead) -> String {
    let mut i = 1;
    let mut sets: Vec<HashSet<char>> = Vec::new();
    let mut sum: u32 = 0;
    for line in get_string_from_input(input).lines() {
        sets.push(line.chars().collect::<HashSet<char>>());
        if i == 3 {
            for c in sets[0].iter() {
                if sets[1].contains(c) && sets[2].contains(c) {
                    sum += priority(c);
                }
            }
            sets.clear();
            i = 0;
        }
        i += 1;
    }
    sum.to_string()
}

fn priority(c: &char) -> u32 {
    if &c.to_lowercase().next().unwrap() == c {
        return *c as u32 - 96;
    }
    return *c as u32 - 38;
}

pub const SOLVERS: &[Solver] = &[part1, part2];

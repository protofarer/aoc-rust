use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// part1=13924
// part2=13448
fn part1(input: &mut dyn BufRead) -> String {
    let mut score: u32 = 0;
    for line in get_string_from_input(input).lines() {
        let mut line = line.split(" ");
        let opp = line.next().unwrap();
        let you = line.next().unwrap();

        match you {
            "X" => score += 1,
            "Y" => score += 2,
            "Z" => score += 3,
            _ => println!("Invalid input char",),
        }

        if opp == "A" && you == "X" || opp == "B" && you == "Y" || opp == "C" && you == "Z" {
            score += 3;
        } else if opp == "A" && you == "Y" || opp == "B" && you == "Z" || opp == "C" && you == "X" {
            score += 6;
        }
    }
    score.to_string()
}

fn part2(input: &mut dyn BufRead) -> String {
    let mut score: u32 = 0;
    for line in get_string_from_input(input).lines() {
        let mut line = line.split(" ");
        let opp = line.next().unwrap();
        let you = line.next().unwrap();

        if you == "Y" {
            score += 3;
            if opp == "A" {
                score += 1;
            } else if opp == "B" {
                score += 2;
            } else if opp == "C" {
                score += 3;
            }
        } else if you == "Z" {
            score += 6;
            if opp == "A" {
                score += 2;
            } else if opp == "B" {
                score += 3;
            } else if opp == "C" {
                score += 1;
            }
        } else if you == "X" {
            if opp == "A" {
                score += 3;
            } else if opp == "B" {
                score += 1;
            } else if opp == "C" {
                score += 2;
            }
        }
    }
    score.to_string()
}

pub const SOLVERS: &[Solver] = &[part1, part2];

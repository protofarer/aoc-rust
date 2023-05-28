use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

pub fn run() {
    if let Err(e) = day3::run_part2() {
        eprintln!("Application Error: {e}");
        std::process::exit(1);
    }
}

// https://adventofcode.com/2022/day/3
// part1
// fail for answer 10439 using non_unique misplaced, too high
// 948 too low using unique misplaced
// 7793 identifying only unique misplaced items per rucksack
// part2 = 2499
pub mod day3 {
    use std::{collections::HashSet, error::Error, fs};

    pub fn run_part1() -> Result<u32, Box<dyn Error>> {
        let contents = fs::read_to_string("/home/kenny/code-exercises/aoc/rust-aoc/data/3_input")?;

        // if fails solution, use HashSet
        let mut all_misplaced = Vec::new();
        // let mut misplaced_items = HashSet::new();

        // parse each line into a map where: letter => key, (0,0) values, 1 if found for corresponding compartment
        for line in contents.lines() {
            let mut rucksack_misplaced = HashSet::new(); // misplaced items are uniquely identified per rucksack
            let mut compart = HashSet::new();
            let idx_split = line.len() / 2 - 1;

            for c in line.chars().take(idx_split + 1) {
                compart.insert(c);
            }

            for c in line.chars().skip(idx_split + 1) {
                if compart.contains(&c) {
                    // misplaced_items.push(c);
                    rucksack_misplaced.insert(c);
                }
            }
            all_misplaced.extend(rucksack_misplaced);
            // let mut first_half = line.chars().take(idx_split + 1).collect::<Vec<char>>();
            // println!("first half: {:?}", first_half);
            // let mut second_half = line.chars().skip(idx_split + 1).collect::<Vec<char>>();
            // println!("second half: {:?}", second_half);
            // first_half.sort();
            // second_half.sort();
            // for i in 0..first_half.len() {
            //     println!("{} {}", first_half[i], second_half[i]);
            // }
        }
        println!("{:?}", all_misplaced);
        let sum_priorities_nonunique = all_misplaced
            .iter()
            .fold(0, |acc, curr| acc + priority(curr));
        println!("{sum_priorities_nonunique}");
        Ok(sum_priorities_nonunique)
    }

    pub fn run_part2() -> Result<u32, Box<dyn Error>> {
        let contents = fs::read_to_string("/home/kenny/code-exercises/aoc/rust-aoc/data/3_input")?;

        let mut i = 1;
        let mut sets: Vec<HashSet<char>> = Vec::new();
        let mut sum: u32 = 0;
        for line in contents.lines() {
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
        println!("{sum}");
        Ok(sum)
    }

    fn priority(c: &char) -> u32 {
        if &c.to_lowercase().next().unwrap() == c {
            return *c as u32 - 96;
        }
        return *c as u32 - 38;
    }
}
// part1: 13924
// part2: 13448
pub mod day2 {
    use std::{error::Error, fs};

    pub fn run() -> Result<u32, Box<dyn Error>> {
        let contents = fs::read_to_string("/home/kenny/code-exercises/aoc/rust-aoc/data/2_input")?;

        let mut score: u32 = 0;
        for line in contents.lines() {
            let mut line = line.split(" ");
            let opp = line.next().unwrap();
            let you = line.next().unwrap();

            // part 1
            // match you {
            //     "X" => score += 1,
            //     "Y" => score += 2,
            //     "Z" => score += 3,
            //     _ => return Err(From::from("invalid you input")),
            // }

            // if opp == "A" && you == "X" || opp == "B" && you == "Y" || opp == "C" && you == "Z" {
            //     println!("you == opp",);
            //     score += 3;
            // } else if opp == "A" && you == "Y"
            //     || opp == "B" && you == "Z"
            //     || opp == "C" && you == "X"
            // {
            //     score += 6;
            // }

            // part 2
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
        println!("{score}");
        Ok(score)
    }
}

pub mod day1 {
    use std::fs;

    // part 1 solution = 73211
    // part 2 solution = 213958
    pub fn run() -> Result<u32, std::io::Error> {
        let contents = fs::read_to_string("/home/kenny/code-exercises/aoc/rust-aoc/data/1_input")?;
        let mut sum: u32 = 0;
        let mut maxes: [u32; 3] = [0, 0, 0];

        // ? CSDR do it idiomatically functionally
        for line in contents.lines() {
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
        println!("Top 3 elves are carrying a sum of {} calories", total);
        Ok(total)
    }
}

fn _read_lines_from_file_buffered<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        lines.push(line);
    }
    Ok(lines)
}

fn _read_file_as_string(file_path: &str) -> Result<String, std::io::Error> {
    Ok(fs::read_to_string(file_path)?)
}

use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

pub fn run() {
    if let Err(e) = day3::run() {
        eprintln!("Application Error: {e}");
        std::process::exit(1);
    }
}

// https://adventofcode.com/2022/day/3
pub mod day3 {
    use std::error::Error;

    pub fn run() -> Result<(), Box<dyn Error>> {
        Ok(())
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

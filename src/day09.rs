#![allow(warnings)]

// part1 != 160 (too low :: typo in tail.react_move)
// != 3096 (too low :: off-by-1, need to count init pos initially since it may not cross back over in position)
// != 3097 (too low :: forced a +1, but input already covers init pos since tail crosses back over it)
// == 6290 wasn't reading double digit move mangitudes!!!!!!!!! use split instead of chars + next'ing

// ? CSDR refactor into a `Rope` struct and behavior

use crate::{get_string_from_input, Solver};
use anyhow::{anyhow, Result};
use std::{collections::HashSet, fmt};

fn part1(input: &mut dyn BufRead) -> String {
    let input_str = get_string_from_input(input);

    let mut rope: Rope = Rope::new(INIT_POS);

    for (i, line) in input_str.lines().enumerate() {
        // println!("{i}:{:?}", line);
        let movement = read_move_line(line).unwrap();
        rope.move_rope(movement);
        // println!("{}", rope);
    }

    let len = rope.get_distinct_path_len();
    println!("distinct_path.len: {}", len);
    len.to_string()
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position(isize, isize);

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
enum Direction {
    U,
    D,
    L,
    R,
}
use Direction::*;
struct Movement(Direction, usize);
const INIT_POS: Position = Position(0, 0);

#[derive(Debug)]
struct Rope {
    head: Position,
    tail: Position,
    distinct_path: HashSet<Position>,
}

impl Rope {
    fn new(init_pos: Position) -> Self {
        let mut set: HashSet<Position> = HashSet::new();
        set.insert(init_pos);
        Rope {
            head: init_pos,
            tail: init_pos,
            distinct_path: set,
        }
    }

    fn get_distinct_path_len(&self) -> usize {
        self.distinct_path.len()
    }

    fn move_rope(&mut self, movement: Movement) {
        for i in 0..movement.1 {
            match movement.0 {
                U => self.head.1 = self.head.1 + 1 as isize,
                D => self.head.1 = self.head.1 - 1 as isize,
                R => self.head.0 = self.head.0 + 1 as isize,
                L => self.head.0 = self.head.0 - 1 as isize,
            }
            self.move_tail();
        }
    }

    fn move_tail(&mut self) {
        let dx = self.head.0 as isize - self.tail.0 as isize;
        let dy = self.head.1 as isize - self.tail.1 as isize;
        if dx.abs() <= 1 && dy.abs() <= 1 {
            return;
        }
        self.tail.0 = match dx {
            dx if dx > 0 => self.tail.0 + 1,
            dx if dx < 0 => self.tail.0 - 1,
            dx if dx == 0 => self.tail.0,
            _ => unreachable!("value of dx produced an error state"),
        };
        self.tail.1 = match dy {
            dy if dy > 0 => self.tail.1 + 1,
            dy if dy < 0 => self.tail.1 - 1,
            dy if dy == 0 => self.tail.1,
            _ => unreachable!("value of dy produced an error state"),
        };
        self.distinct_path.insert(self.tail); // this is a copy, since HashSet is supposed to contain values not references
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "H({},{}) T({},{})",
            self.head.0, self.head.1, self.tail.0, self.tail.1
        )
    }
}

fn read_move_line(line: &str) -> anyhow::Result<Movement> {
    let mut words: Vec<&str> = line.split_whitespace().collect();
    let dir = match words[0] {
        "U" => U,
        "D" => D,
        "L" => L,
        "R" => R,
        _ => {
            eprintln!("Invalid movement char in input");
            return Err(anyhow!("bad input char for movement"));
        }
    };
    let n: usize = words[1].parse().unwrap();
    Ok(Movement(dir, n))
}

fn part2(input: &mut dyn BufRead) -> String {
    "ok".into()
}

use assert_fs::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[test]
fn some_moves() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n")?;
    let file = File::open(file.path())?;

    let mut reader = BufReader::new(file);

    assert_eq!(part1(&mut reader), "13");

    Ok(())
}
#[test]
fn more_moves() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("U 1\nD 1\nD 1\nD 1\nD 2\nU 1\nL 2\nD 3\n")?;
    let file = File::open(file.path())?;

    let mut reader = BufReader::new(file);

    assert_eq!(part1(&mut reader), "7");

    Ok(())
}
#[test]
fn tail_doesnt_move() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("U 1\nR 1\nD 1\nD 1\nL 1\nL 1\nU 2\nR 2\nL 2\nR 1\nR 1\nD 2\n")?;
    let file = File::open(file.path())?;

    let mut reader = BufReader::new(file);

    assert_eq!(part1(&mut reader), "1");

    Ok(())
}

#[test]
fn double_digit_move() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("R 10\n")?;
    let file = File::open(file.path())?;

    let mut reader = BufReader::new(file);

    assert_eq!(part1(&mut reader), "10");

    Ok(())
}

#[test]
fn triple_digit_move() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("R 100\n")?;
    let file = File::open(file.path())?;

    let mut reader = BufReader::new(file);

    assert_eq!(part1(&mut reader), "100");

    Ok(())
}

pub const SOLVERS: &[Solver] = &[part1, part2];

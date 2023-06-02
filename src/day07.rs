use std::{
    collections::HashMap,
    error::Error,
    io::BufRead,
    path::{Path, PathBuf},
};

use crate::Solver;

// part1 = 1428881
// issues encountered:
//  - collecting unnecessary data like subdirs or making a DirData struct when size is all that's needed
//  - iterating, permissions, borrowing on a HashMap
//  - using simple hashmap by path instead of a tree (only solve problem at hand, don't make it extensible or over-optimize)

// part2 = 10475598
// issues encountered:
//  - remembering what data is passed back from part1
//  - .sort() is in-place

const TOTAL_DISK_SPACE: u32 = 70000000;
const SPACE_FOR_UPDATE: u32 = 30000000;

// Find size of smallest file that upon deletion will free up enough space for the update
fn part2(input: &mut dyn BufRead) -> String {
    let map = sum_dir_sizes(input).unwrap();

    let space_used = map.get(&PathBuf::from("/").into_boxed_path()).unwrap();
    let space_free_without_update = TOTAL_DISK_SPACE - space_used;
    let space_to_free: i32 = SPACE_FOR_UPDATE as i32 - space_free_without_update as i32;

    if space_to_free <= 0 {
        return "0".to_string();
    }

    let mut candidate_sizes: Vec<_> = map
        .values()
        .filter(|&&s| s > space_to_free as u32)
        .collect();

    candidate_sizes.sort_unstable();
    candidate_sizes.first().unwrap().to_string()
}

// Sum the sizes of directories which are no larger than 100_000
fn part1(input: &mut dyn BufRead) -> String {
    // The two primary data structure are:
    //  - HashMap: key=path_to_dir (string), value=DirData struct: tuple
    //  containing total size and a vector of subdirectories' paths

    // Cautions: dir names non-unique => paths are unique

    // Assumptions:
    // - directories are only ls'ed once, can make a check to verify truth
    // - ls is not executed on the same dir more than once

    // Data inspection:
    // - cd to root happens once, at beginning of input
    // - cd cmds only move up/down one level
    let map = sum_dir_sizes(input).unwrap();

    map.values()
        .filter(|&&dir_total_size| dir_total_size <= 100000)
        .sum::<u32>()
        .to_string()
}

fn sum_dir_sizes(input: &mut dyn BufRead) -> Result<HashMap<Box<Path>, u32>, Box<dyn Error>> {
    // let contents = fs::read_to_string("/home/kenny/code-exercises/aoc/rust-aoc/data/7_input")?;
    let mut map: HashMap<Box<Path>, u32> = HashMap::new();
    let mut cwd = PathBuf::new();

    for line in input.lines() {
        match line.unwrap() {
            line_cmd if line_cmd.starts_with("$") => {
                parse_and_act_cmd(&mut cwd, &line_cmd, &mut map)
            }
            line_dir if line_dir.starts_with("dir") => {

                // let mut line_dir = line_dir.split(" ");
                // line_dir.next();
                // let dir = line_dir.next().unwrap().to_string();
            }
            line_file => {
                let size = line_file.split(" ").next().unwrap().parse::<u32>().unwrap();
                add_size_matching_paths(&cwd, size, &mut map);
            }
        }
    }
    Ok(map)
}

fn add_size_matching_paths(cwd: &PathBuf, size: u32, map: &mut HashMap<Box<Path>, u32>) {
    let paths: Vec<_> = map
        .keys()
        .cloned()
        .filter(|p| cwd.to_str().unwrap().starts_with(p.to_str().unwrap()))
        .collect();

    for path in paths {
        map.entry(path.clone())
            .and_modify(|dir_total_size| *dir_total_size += size);
    }
}

fn parse_and_act_cmd(cwd: &mut PathBuf, cmd_line: &str, map: &mut HashMap<Box<Path>, u32>) {
    let cmd_line: Vec<&str> = cmd_line.split(" ").collect();
    let cmd = cmd_line[1];

    if cmd == "ls" {
        // if this cmd is executed for a cwd more than once, FLAG IT or handle it (don't +size)
        map.entry(cwd.clone().into_boxed_path()).or_insert(0);
    } else if cmd == "cd" {
        if cmd_line[2] == ".." {
            cwd.pop();
        } else {
            cwd.push(cmd_line[2]);
        }
    }
}

pub const SOLVERS: &[Solver] = &[part1, part2];

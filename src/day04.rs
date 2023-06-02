use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// part1 = 518
// part2 != 811 (too low) :: was editing wrong run function
// part2 = 909

fn part1(input: &mut dyn BufRead) -> String {
    let mut n_fully_contained = 0;

    for line in get_string_from_input(input).lines() {
        let ranges = parse_ranges(line).unwrap();
        let (range_a, range_b) = (ranges[0], ranges[1]);
        if (range_a[0] >= range_b[0] && range_a[1] <= range_b[1])
            || (range_b[0] >= range_a[0] && range_b[1] <= range_a[1])
        {
            n_fully_contained += 1;
        }
    }
    n_fully_contained.to_string()
}

fn part2(input: &mut dyn BufRead) -> String {
    let mut n_overlaps = 0;

    for line in get_string_from_input(input).lines() {
        let ranges = parse_ranges(line).unwrap();
        let (range_a, range_b) = (ranges[0], ranges[1]);
        if range_a[0] <= range_b[1] && range_a[1] >= range_b[0] {
            n_overlaps += 1;
        }
    }
    n_overlaps.to_string()
}

fn parse_ranges(line: &str) -> Result<Vec<[u32; 2]>, std::num::ParseIntError> {
    line.split(",")
        .map(|range| {
            let mut nums = range.split("-").map(|num| num.parse::<u32>());
            let a = nums.next().unwrap()?;
            let b = nums.next().unwrap()?;
            Ok([a, b])
        })
        .collect()
}

pub const SOLVERS: &[Solver] = &[part1, part2];

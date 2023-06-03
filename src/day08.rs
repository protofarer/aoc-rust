use crate::Solver;
use std::io::BufRead;

// part1: != 1 (too low); != 1338 (too low) ; != 1730 (too high)
// part1 = 1693 (I had flipped the row/col when accessing grid!)
// part2 = 422059

// sketch:
// - trees under analysis are all interior trees, IOW not on edge of grid (simply add these)
// - iterate by tree, check against strip of trees about each cardinal direction
fn part1(input: &mut dyn BufRead) -> String {
    let grid = create_grid(input);
    let mut n_visible = 0;
    let width = grid.len();
    let height = grid[0].len();
    for row in 1..width - 1 {
        for col in 1..height - 1 {
            if is_visible(&grid, col, row) {
                n_visible += 1;
            }
        }
    }
    // add edge trees
    n_visible += (width * 2) + (height - 2) * 2;
    n_visible.to_string()
}

fn part2(input: &mut dyn BufRead) -> String {
    let grid = create_grid(input);
    let mut max_scenic_score = 0;
    let width = grid.len();
    let height = grid[0].len();
    // TODO dont check edge trees since their scores always 0, update scoring function too
    for row in 1..width - 1 {
        for col in 1..height - 1 {
            let x = get_scenic_score(&grid, col, row);
            max_scenic_score = if x > max_scenic_score {
                x
            } else {
                max_scenic_score
            };
        }
    }
    max_scenic_score.to_string()
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn is_visible(grid: &Vec<Vec<u8>>, col: usize, row: usize) -> bool {
    let h = grid[row][col];
    [
        h > get_strip_max(grid, row, col, Direction::North),
        h > get_strip_max(grid, row, col, Direction::South),
        h > get_strip_max(grid, row, col, Direction::East),
        h > get_strip_max(grid, row, col, Direction::West),
    ]
    .iter()
    .any(|&x| x)
}

fn get_strip_max(grid: &Vec<Vec<u8>>, row: usize, col: usize, dir: Direction) -> u8 {
    *get_strip(grid, row, col, dir).iter().max().unwrap()
}

fn get_strip(grid: &Vec<Vec<u8>>, row: usize, col: usize, dir: Direction) -> Vec<u8> {
    let mut strip = vec![];

    match dir {
        Direction::North => {
            for r in 0..row {
                strip.push(grid[r][col]);
            }
            strip.reverse();
        }
        Direction::South => {
            for r in row + 1..grid.len() {
                strip.push(grid[r][col]);
            }
        }
        Direction::East => {
            for c in col + 1..grid[0].len() {
                strip.push(grid[row][c]);
            }
        }
        Direction::West => {
            for c in 0..col {
                strip.push(grid[row][c]);
            }
            strip.reverse();
        }
    }
    strip
}

fn create_grid(input: &mut dyn BufRead) -> Vec<Vec<u8>> {
    let mut input_grid: Vec<Vec<u8>> = vec![]; // build this up as maxes are calculated

    for (row, line) in input.lines().enumerate() {
        input_grid.push(vec![]);

        for x in line.unwrap().chars() {
            let h = x.to_digit(10).unwrap() as u8;
            input_grid[row].push(h);
        }
    }

    input_grid
}

fn count_trees(strip: Vec<u8>, tree_height: u8) -> usize {
    let mut count = 0;
    for h in strip {
        if h < tree_height {
            count += 1;
        } else if h == tree_height {
            count += 1;
            break;
        }
    }
    count
}

fn get_scenic_score(grid: &Vec<Vec<u8>>, col: usize, row: usize) -> usize {
    let tree_height = grid[row][col];

    let north_score = {
        let strip = get_strip(grid, row, col, Direction::North);
        count_trees(strip, tree_height)
    };

    let south_score = {
        let strip = get_strip(grid, row, col, Direction::South);
        count_trees(strip, tree_height)
    };

    let east_score = {
        let strip = get_strip(grid, row, col, Direction::East);
        count_trees(strip, tree_height)
    };

    let west_score = {
        let strip = get_strip(grid, row, col, Direction::West);
        count_trees(strip, tree_height)
    };

    north_score * south_score * east_score * west_score
}

pub const SOLVERS: &[Solver] = &[part1, part2];

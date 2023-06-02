use crate::Solver;
use std::io::BufRead;

// part1 != TVMFBQTNS TVMFBQTDS
// part1 = FRDSQRRCD
// part2 = HRFTQVWNN

fn part1(input: &mut dyn BufRead) -> String {
    let mut data = vec![];
    input.read_to_end(&mut data).unwrap();
    let input_str = String::from_utf8(data).expect("Data inst UTF8");

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    let lines = input_str.lines();

    for line in lines.clone().take(8) {
        read_line_to_stack(&mut stacks, line);
    }
    // println!("{:?}", stacks[7]);

    let move_lines = lines.skip(10);
    for move_line in move_lines {
        move_by_pop(&mut stacks, move_line);
    }
    // println!("final stacks {:?}", stacks);
    let mut top_crates = String::new();
    for mut stack in stacks {
        top_crates.push(stack.pop().unwrap());
        // top_crates.push(stack.pop().unwrap());
    }
    top_crates
}

fn part2(input: &mut dyn BufRead) -> String {
    let mut data = vec![];
    input.read_to_end(&mut data).unwrap();
    let input_str = String::from_utf8(data).expect("Data inst UTF8");
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    let lines = input_str.lines();

    for line in lines.clone().take(8) {
        read_line_to_stack(&mut stacks, line);
    }
    // println!("{:?}", stacks[7]);

    for move_line in lines.skip(10) {
        move_by_group(&mut stacks, move_line);
    }
    // println!("final stacks {:?}", stacks);
    let mut top_crates = String::new();
    for mut stack in stacks {
        top_crates.push(stack.pop().unwrap());
        // top_crates.push(stack.pop().unwrap());
    }
    top_crates
}

fn move_by_group(stacks: &mut Vec<Vec<char>>, move_line: &str) {
    let words: Vec<&str> = move_line.split(" ").collect();
    let n_crates: usize = words[1].parse().unwrap();
    let from_stack: usize = words[3].parse::<usize>().unwrap() - 1;
    let to_stack: usize = words[5].parse::<usize>().unwrap() - 1;
    let len = stacks[from_stack].len();

    // println!("from stack {:?}", stacks[from_stack]);
    // println!("to stack {:?}", stacks[to_stack]);
    let mut crates_to_move = stacks[from_stack]
        .drain(len - n_crates..len)
        .collect::<Vec<char>>();
    stacks[to_stack].append(&mut crates_to_move);
    // println!("new mutated from stack {:?}", stacks[from_stack]);
    // println!("new mutated to stack {:?}", stacks[to_stack]);
}

fn move_by_pop(stacks: &mut Vec<Vec<char>>, move_line: &str) {
    let words: Vec<&str> = move_line.split(" ").collect();
    let n_crates: usize = words[1].parse().unwrap();
    let from_stack: usize = words[3].parse::<usize>().unwrap() - 1;
    let to_stack: usize = words[5].parse::<usize>().unwrap() - 1;

    // println!("from stack {:?}", stacks[from_stack]);
    // println!("to stack {:?}", stacks[to_stack]);
    for _i in 0..n_crates {
        let a_crate = stacks[from_stack].pop().unwrap();
        stacks[to_stack].push(a_crate);
    }
    // println!("new mutated from stack {:?}", stacks[from_stack]);
    // println!("new mutated to stack {:?}", stacks[to_stack]);
}

fn read_line_to_stack(stacks: &mut Vec<Vec<char>>, line: &str) {
    for (i, c) in line.chars().enumerate() {
        if (i as i32 - 1) % 4 == 0 && c != ' ' {
            stacks[(i - 1) / 4].insert(0, c);
        }
    }
}

pub const SOLVERS: &[Solver] = &[part1, part2];

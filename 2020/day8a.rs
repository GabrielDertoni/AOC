use std::io;
use std::io::prelude::*;
use std::collections::BTreeSet;

enum Op {
    Nop,
    Acc(i32),
    Jmp(i32),
}

fn parse_op(string: &str) -> Op {
    if string.starts_with("nop") {
        Op::Nop
    } else if string.starts_with("acc") {
        Op::Acc(string[4..].parse().unwrap())
    } else if string.starts_with("jmp") {
        Op::Jmp(string[4..].parse().unwrap())
    } else {
        unreachable!();
    }
}

fn main() {
    let input = io::stdin().lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    let mut visited: BTreeSet<usize> = BTreeSet::new();
    let mut curr_line: usize = 0;
    let mut acc = 0;
    
    loop {
        if let Some(_) = visited.get(&curr_line) {
            break;
        } else {
            visited.insert(curr_line);
        }

        let line = &input[curr_line];
        let op = parse_op(line);

        curr_line = match op {
            Op::Nop      => curr_line + 1,
            Op::Acc(num) => {
                acc += num;
                curr_line + 1
            },
            Op::Jmp(off) => {
                (curr_line as i32 + off) as usize
            },
        };

    }

    println!("Accumulator value is {}", acc);
}

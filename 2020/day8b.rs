use std::io;
use std::io::prelude::*;
use std::collections::BTreeSet;
use Op::{ Nop, Acc, Jmp };

enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_op(string: &str) -> Op {
    let num = string[4..].parse().unwrap();
    if string.starts_with("nop") {
        Op::Nop(num)
    } else if string.starts_with("acc") {
        Op::Acc(num)
    } else if string.starts_with("jmp") {
        Op::Jmp(num)
    } else {
        unreachable!();
    }
}

// Traverses through the instructions and returns Some(acc) if there is a path
// to the end of the instructions (allowing one fix), where `acc` is the
// accumulator value at that point of the execution. Returns None if there is
// no one-fix solution that removes all infinite loops withing the program.
fn traverse(prog_counter: i32, ops: &Vec<Op>, accumulator: i32,
            visited: &mut BTreeSet<i32>, is_changed: bool) -> Option<i32> {

    // If it is a repetition, there is no path to the end of the program.
    if let Some(_) = visited.get(&prog_counter) {
        return None;
    }

    visited.insert(prog_counter);
    let op = &ops[prog_counter as usize];
    let (next_counter, next_acc) = match op {
        Nop(_)   => (prog_counter + 1  , accumulator      ),
        Acc(val) => (prog_counter + 1  , accumulator + val),
        Jmp(off) => (prog_counter + off, accumulator      ),
    };

    if next_counter as usize >= ops.len() {
        Some(next_acc)
    } else {
        match traverse(next_counter, ops, next_acc, visited, is_changed) {
            Some(acc) => Some(acc),
            None      => {
                if is_changed {
                    // Cant make another change.
                    None
                } else {
                    let changed_next_counter = match op {
                        Acc(_)   => return None,         // No possible change in acc instruction.
                        Nop(off) => prog_counter + off,  // Interpret as jmp instruction.
                        Jmp(_)   => prog_counter + 1,    // Interpret as nop instruction.
                    };
                    traverse(changed_next_counter, ops, next_acc, visited, true)
                }
            }
        }
    }
}

fn main() {
    let ops = io::stdin().lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_op(&line))
        .collect::<Vec<Op>>();

    let mut visited: BTreeSet<i32> = BTreeSet::new();
    
    if let Some(acc) = traverse(0, &ops, 0, &mut visited, false) {
        println!("Accumulator value is {}", acc);
    } else {
        println!("There is no \"one fix\" solution");
    }

}

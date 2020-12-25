#![feature(str_split_once)]

use std::io;
use std::io::prelude::*;
use std::fmt;
use std::str::FromStr;
use Instruction::{ Mask, Mem };

struct State {
    mem: Vec<(usize, usize)>,
    mask_overwrite: usize,
    mask_replace: usize,
}

impl State {
    fn new() -> Self {
        State {
            // mem[i] is a tuple (address offset, value)
            mem: Vec::new(),
            mask_overwrite: 0, // All bits set to 0.
            mask_replace: 0,   // All bits set to 1.
        }
    }
}

enum Instruction {
    Mask((usize, usize)),
    Mem((usize, usize)),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, value) = s.split_once(" = ").ok_or("Expected '=' in the instruction")?;
        if op.starts_with("mask") {
            value.trim()
                .chars()
                .enumerate()
                .fold(Ok((0, 0)), |s, (i, c)| {
                    let bit = 1 << (35 - i);
                    match c {
                        // Overwrite with 0, replace is 1.
                        '0' => s.map(|(overwrite, replace)| (overwrite, replace | bit)),
                        // Overwrite with 1, replace is 1.
                        '1' => s.map(|(overwrite, replace)| (overwrite | bit, replace | bit)),
                        // Overwrite with 0, replace is 0.
                        'X' => s,
                        _   => Err("Unexpected char found in instruction"),
                    }
                })
                .map(|s| Mask(s))
        } else if op.starts_with("mem") {
            let idx = op.split(|c| c == '[' || c == ']')
                .nth(1)
                .unwrap()
                .parse()
                .map_err(|_| "Expected to parse a number as index")?;

            let value = value.trim()
                .parse()
                .map_err(|_| "Expected to parse a number as value")?;

            Ok(Mem((idx, value)))
        } else {
            Err("Instruction should start with 'mask' or 'mem'")
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mask = ")?;
        for i in 0..36 {
            match (self.mask_replace >> i) & 1 {
                0 => write!(f, "X")?,
                1 => write!(f, "{}", (self.mask_overwrite >> i) & 1)?,
                _ => unreachable!(),
            }
        }
        write!(f, "\n")?;
        for (idx, val) in self.mem.iter() {
            write!(f, "mem[{}] = {}\n", idx, val)?;
        }
        if self.mem.len() == 0 {
            write!(f, "Empty memory!\n")?;
        }
        write!(f, "----")?;
        Ok(())
    }
}

fn main() {
    let stdin = io::stdin();
    let state = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .fold(State::new(), |mut state, instruction| {
            let new_state = match instruction {
                Mask((overwrite, replace)) => {
                    state.mask_overwrite = overwrite;
                    state.mask_replace = replace;
                    state
                },
                Mem((idx, value)) => {
                    let write = state.mask_overwrite | (value & !state.mask_replace);
                    match state.mem.binary_search_by(|&(i, _)| i.cmp(&idx)) {
                        Ok(vidx)  => { state.mem[vidx].1 = write; },
                        Err(vidx) => state.mem.insert(vidx, (idx, write)),
                    }
                    state
                },
            };
            new_state
        });

    let final_sum = state.mem.iter().map(|&(_, v)| v).sum::<usize>();
    println!("The sum of all elements in memory is {}", final_sum);
}

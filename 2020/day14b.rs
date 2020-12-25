#![feature(str_split_once)]

use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;
use Instruction::{ Mask, Mem };

/**
 * This is where I gave up. That is simply a brute force approach.
 * NOTE: It works!!
 */
struct State {
    mem: HashMap<usize, usize>,
    mask_bits: usize,
    mask_xs: usize,
}

impl State {
    fn new() -> Self {
        State {
            mem: HashMap::new(),
            mask_bits: 0, // All bits set to 0.
            mask_xs: 0,   // All bits set to 1.
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
                        // Bits is 0, xs is 0.
                        '0' => s,
                        // Bits is 1, xs is 0.
                        '1' => s.map(|(b, xs)| (b | bit, xs)),
                        // Bits is 0, xs is 1.
                        'X' => s.map(|(b, xs)| (b, xs | bit)),
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

fn print_addr(addr: usize, xs: usize) {
    print!("addr = ");
    for i in 0..36 {
        if (xs >> (35 - i)) & 1 == 0 {
            print!("{}", (addr >> (35 - i)) & 1);
        } else {
            print!("X");
        }
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let state = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .fold(State::new(), |mut state, instruction| {
            match instruction {
                Mask((bits, xs)) => {
                    state.mask_bits = bits;
                    state.mask_xs = xs;
                    state
                },
                Mem((idx, value)) => {
                    let n_xs = state.mask_xs.count_ones();
                    for i in 0..2_usize.pow(n_xs) {
                        let mut addr = (idx | state.mask_bits) & !state.mask_xs;
                        let mut shift = 0;
                        for j in 0..36 {
                            if (state.mask_xs >> j) & 1 == 1 {
                                addr |= ((i >> shift) & 1) << j;
                                shift += 1;
                            }
                        }
                        state.mem.insert(addr, value);
                    }
                    state
                },
            }
        });

    let sum: usize = state.mem.iter().map(|(_, v)| v).sum();
    println!("The sum of all elements in memory is {}", sum);
}

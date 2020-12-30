#![feature(str_split_once)]

/**
 * Solution writeup.
 *
 * The chalange is that we need to simulate a 36-bit address space without actually having
 * enough memory to use all of those addresses. Each write will be done to a generic address,
 * that is, a memory address that may contain Xs in its bits meaning that bit could be a 0 or
 * a 1. So when inserting the value 3 at the generic address 0XX1, it would be as if addresses
 * 0001, 0011, 0101 and 0111 had been written to memory with value 3.
 *
 * In order to make the operation efficient, we can usa a datastructure (in this case a vector)
 * to store the writes in such a way to use this "generic address" notation to use less memory.
 * So, if the memory is empty and the value 3 needs to be written to 0XX1, we can just add that
 * entry (0XX1, 3) to the vector. If we want to write then the value 5 to address 10XX, we can
 * do that, no problem. However, a problem arises if a new value is to be written to a generic
 * address that collides with some other generic address inserted. Say we want to insert value
 * 8 to address 0X11, well we have already a "written" to address 0011 and 0111 wich all have
 * value 3 but now must change to have value 8. To do that, we need to split the collision
 * address into less generic ones that can describe what is now 8 and what is still 3.
 *
 * Let `addr` be the generic address to be inserted and `collision` the generic address that
 * collides with `addr`. Then `addr.xs` is a bit field where a 1 represents an X at that bit.
 * `addr.bits` are the bits of the generic address that are not Xs. One assumption used is that
 * `addr.bits & addr.xs == 0`. So, in order to detect a collision we can use get all xs in both
 * `all_xs = addr.xs | collision.xs` and then if
 * `addr.bits & !all_xs == collision & !all_xs`
 * is true, we have found a collision.
 *
 * Now, we remove `collision` from the vector as it will be changed. Every X bit that occurs in
 * both `addr` and `collision` is still going to be present in the replacements. Every X bit that
 * occurs only in `addr` is unchanged because its the places that will be overwritten. Every X
 * bit that occurs only in `collision` is special, because it is where there will be a split.
 * Let n be the number of Xs only present in `collision`, then `collision` will be replaced with
 * exactly n entries in the vector. For each one of n iterations a bit that was an X only in
 * collision is chosen as "considered". This bit is turned into the opposite of its annalogous bit
 * in `addr`. Then, all Xs left of the considered bit on the collision are left untouched and all
 * Xs on the right become their analogous bits in `addr`. The resulting generic address is inserted
 * in the vector. This new insertion is garanteed not to collide with `addr` because the considered
 * bit changed from an X, to the opposite of what is present in `addr`.
 *
 * Example:
 * only in collision
 * v  v
 * X0XX <-- collision
 * 10X0 <-- addr
 *
 * split into:
 * bit beeing considered is flipped from addr
 *    v
 * X0X1
 * now this is the one flipped
 * v
 * 00X0
 *    ^
 *  this is no longer flipped
 */

use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::cmp::{ PartialEq, Eq };
use std::fmt;
use Instruction::{ Mask, Mem };

#[derive(Clone, Copy)]
struct GenericAddr {
    bits: usize,
    xs: usize,
}

impl GenericAddr {
    fn new(bits: usize, xs: usize) -> Self {
        GenericAddr { bits, xs }
    }
}

impl PartialEq for GenericAddr {
    fn eq(&self, other: &Self) -> bool {
        let xs = self.xs | other.xs;
        self.bits & !xs == other.bits & !xs
    }
}

impl Eq for GenericAddr { }

impl fmt::Display for GenericAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..36).rev() {
            let x = (self.xs >> i) & 1;
            let bit = (self.bits >> i) & 1;
            let c = if x == 1 { 'X' } else if bit == 1 { '1' } else { '0' };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl FromStr for GenericAddr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim()
            .chars()
            .enumerate()
            .fold(Ok((0, 0)), |s, (i, c)| {
                let bit = 1 << (35 - i);
                match c {
                    '0' => s, // Bits is 0, xs is 0.
                    '1' => s.map(|(b, xs)| (b | bit, xs)), // Bits is 1, xs is 0.
                    'X' => s.map(|(b, xs)| (b, xs | bit)), // Bits is 0, xs is 1.
                    _   => Err("Unexpected char found in generic address."),
                }
            })
            .map(|(b, xs)| GenericAddr::new(b, xs))
    }
}

struct State {
    mem: Vec<(GenericAddr, usize)>,
    mask: GenericAddr,
}

impl State {
    fn new() -> Self {
        State {
            mem: Vec::new(),
            mask: GenericAddr::new(0, 0),
        }
    }
    fn write(&mut self, addr: GenericAddr, value: usize) {
        // Find all collisions with the new generic address.
        while let Some(i) = self.mem.iter().position(|(a, _)| *a == addr) {
            // Remove the collision and replace it with something that does not collide with addr.
            let (collision, val) = self.mem.swap_remove(i);
            let xs_in_both = collision.xs & self.mask.xs;
            let xs_only_in_collision = collision.xs ^ xs_in_both;
            for i in 0..xs_only_in_collision.count_ones() {
                let pos = (0..i+1).fold(0, |p, _| p + (xs_only_in_collision >> p).trailing_zeros() + 1);
                let x_considered = 1 << (pos - 1);
                let removed_xs = xs_only_in_collision & ((1 << pos) - 1);
                let replace_xs = removed_xs ^ xs_only_in_collision;
                let flipped_bit = x_considered & !addr.bits;
                let prev_removed_xs = x_considered ^ removed_xs;
                let replace = (addr.bits & prev_removed_xs) | flipped_bit;
                let replace_addr = GenericAddr::new(replace | collision.bits, replace_xs | xs_in_both);
                self.write(replace_addr, val);
            }
        }
        self.mem.push((addr, value));
    }
}

enum Instruction {
    Mask(GenericAddr),   // Mask instruction, only contains a generic address.
    Mem((usize, usize)), // Memory instruction, write the second to memory index first.
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, value) = s.split_once(" = ").ok_or("Expected '=' in the instruction")?;
        if op.starts_with("mask") {
            value.parse().map(|s| Mask(s))
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

fn main() {
    let stdin = io::stdin();
    let state = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .fold(State::new(), |mut state, instruction| {
            match instruction {
                Mask(mask) => {
                    state.mask = mask;
                    state
                },
                Mem((idx, value)) => {
                    let addr = GenericAddr::new(state.mask.bits | (idx & !state.mask.xs), state.mask.xs);
                    state.write(addr, value);
                    state
                },
            }
        });

    let sum: usize = state.mem.iter().map(|(addr, v)| 2_usize.pow(addr.xs.count_ones()) * v).sum();
    println!("The sum of all elements in memory is {}, used {} entries in memory vector", sum, state.mem.len());
}

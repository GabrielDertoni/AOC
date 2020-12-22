use std::io;
use std::io::prelude::*;

fn main() {
    let mut joltages: Vec<i32> = io::stdin().lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| l.parse())
        .map(Result::unwrap)
        .collect();

    joltages.sort();
    let mut joltage_skips = [0; 3];

    // Assert that can jump to firts adapter.
    assert!(joltages[0] - 3 <= 0);
    joltage_skips[(joltages[0] - 1) as usize] += 1;
    for i in 0..joltages.len() - 1 {
        let jump = joltages[i + 1] - joltages[i];
        joltage_skips[(jump - 1) as usize] += 1;
    }
    joltage_skips[2] += 1;
    println!("Answer is {}", joltage_skips[0] * joltage_skips[2]);
}

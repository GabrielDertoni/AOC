use std::io;
use std::io::prelude::*;

fn main() {
    let mut joltages: Vec<i32> = io::stdin().lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| l.parse())
        .map(Result::unwrap)
        .collect();

    joltages.push(0);
    joltages.sort();
    joltages.push(joltages[joltages.len() - 1] + 3);

    let mut nways_dp: Vec<usize> = vec![0; joltages.len()];

    for i in (0..joltages.len()).rev() {
        if i == joltages.len() - 1 {
            nways_dp[i] = 1;
        } else {
            let joltage = joltages[i];
            let mut jumping_to = i + 1;
            while jumping_to < joltages.len() && joltages[jumping_to] - joltage <= 3 {
                nways_dp[i] += nways_dp[jumping_to];
                jumping_to += 1;
            }
        }
    }
    println!("# of ways to combine adapters is {}", nways_dp[0]);
}

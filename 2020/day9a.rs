use std::io;
use std::io::prelude::*;

fn main() {
    let preemble_size = 25;
    if let Some(number) = io::stdin().lock()
        .lines()
        .map(Result::unwrap) 
        .map(|n| n.parse().unwrap())
        .collect::<Box<[i64]>>()
        .windows(preemble_size + 1)
        .map(|set| set.split_last().unwrap())
        .fold(None, |num, (&n, preemble)| {
            if let Some(_) = num {
                return num;
            }

            // Finding a pair of numbers that sum to n. O(n^2)
            if preemble.iter().fold(false, |found, &a|
                found || preemble.iter().find(|&&el| el == n - a).is_some())
            {
                None
            } else {
                Some(n)
            }
        }) {
        println!("The first number that does not fit the pattern is {}", number);
    } else {
        println!("All numbers fit the pattern");
    }
}

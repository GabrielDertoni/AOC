use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    // O(n) solution. Not exatcly suuuper fast. But hopefully good enough.
    let stdin = io::stdin();
    let starting: Vec<u32> = stdin.lock()
        .lines()
        .map(Result::unwrap)
        .nth(0)
        .expect("There is a first line")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut numbers: HashMap<u32, usize> = starting.iter()
        .map(|&n| n) // Copy from reference
        .zip(0..)
        .take(starting.len() - 1) // Take all except the last
        .collect();

    let end = 30000000;
    let mut last = starting[starting.len() - 1];
    for nth in starting.len().. {
        let next = match numbers.get(&last) {
            Some(i) => (nth - i - 1) as u32,
            None => 0,
        };
        if (nth + 1) % 1000 == 0 {
            // println!("Iteration {}, used {} spots in the table.", nth + 1, numbers.len());
            let prog = (nth + 1) as f64 / end as f64;
            print!( "\rProgress: [{}{}]  "
                  , std::iter::repeat('#').take((prog        * 30.).floor() as usize).collect::<String>()
                  , std::iter::repeat('.').take(((1. - prog) * 30.).ceil()  as usize).collect::<String>()
                  );
        }
        if nth + 1 == end {
            println!();
            println!("The {}th number spoken is {}.", nth + 1, next);
            println!("Used {} spots in the hash table.", numbers.len());
            break;
        }

        numbers.insert(last, nth - 1);
        last = next;
    }
}

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut numbers: Vec<u32> = stdin.lock()
        .lines()
        .map(Result::unwrap)
        .nth(0)
        .expect("There is a first line")
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    for nth in numbers.len().. {
        let last = numbers[numbers.len() - 1];
        let last_occurrence = numbers[..numbers.len() - 1]
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, &el)| el == last);

        let next = match last_occurrence {
            Some((i, _)) => (nth - i - 1) as u32,
            None => 0,
        };
        numbers.push(next);
        if nth + 1 == 2020 {
            println!("The {}th number spoken is {}", nth + 1, next);
            break;
        }
    }
}

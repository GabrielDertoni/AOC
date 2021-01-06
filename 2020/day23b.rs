use std::io;
use std::io::prelude::*;

fn main() {
    let n_iters = 10000000;
    let n_elements = 1000000;

    let stdin = io::stdin();
    let starting_config: Vec<usize> = stdin.lock()
        .lines()
        .map(Result::unwrap)
        .take(1)
        .map(|line| {
            line.chars()
                .map(|c| (c as u8 - b'0') as usize)
                .collect()
        })
        .nth(0)
        .unwrap();

    // `cup_config` is a linked list that cup_config[cup - 1] = next. Meaning
    // that any given cup numbered `cup` has on its direct clockwise direction
    // the cup numbered cup_config[cup - 1]. `cup_config` is a vector of nexts.
    let mut cup_config: Vec<usize> = (1..=n_elements)
        // Make a circular linking by default.
        .map(|i| if i == n_elements { 1 } else { i + 1 })
        .collect();

    let max_cup = n_elements;
    let min_cup = 1;

    // The head of the linked list. This is the index of the first cup's next
    // in cup_config.
    let head = starting_config[0] - 1;

    // Link list according to starting_config
    for i in 0..starting_config.len() {
        let cup = starting_config[i] - 1;
        cup_config[cup] = match starting_config.get(i + 1) {
            Some(&next) => next,
            None        => i + 2,
        };
    }
    let len = cup_config.len();
    // Make linked list cyclic.
    cup_config[len - 1] = head + 1;

    let mut curr_cup = head + 1;
    for i in 0..n_iters {
        // Remove three elements directly clockwise of `curr_cup` folowing links
        let mut removed = [0; 3];
        let mut ptr = curr_cup - 1;
        for r in removed.iter_mut() {
            *r = cup_config[ptr];
            ptr = *r - 1;
        }
        // Reconect. The next of `curr_cup` is now what comes after what was removed.
        cup_config[curr_cup - 1] = cup_config[ptr];

        // The cup we are looking for
        let mut destination_cup = if curr_cup <= min_cup { max_cup } else { curr_cup - 1 };

        // While the destination cup is a part of the cups we removed, change
        // the destination.
        while removed.iter().any(|&el| el == destination_cup) {
            if destination_cup <= min_cup {
                destination_cup = max_cup;
            } else {
                destination_cup -= 1;
            }
        }
        // Put all those removed cups after the destination cup.
        let old_next = cup_config[destination_cup - 1];
        cup_config[destination_cup - 1] = removed[0];
        cup_config[removed[2] - 1] = old_next;

        // Advence the curr_cup to its direct clockwise cup.
        curr_cup = cup_config[curr_cup - 1];

        // Print some progress bars :)
        if (i + 1) % 1000 == 0 {
            let prog = (i + 1) as f64 / n_iters as f64;
            print!( "\rProgress: [{}{}]  "
                  , std::iter::repeat('█').take((prog        * 30.).floor() as usize).collect::<String>()
                  , std::iter::repeat(' ').take(((1. - prog) * 30.).ceil()  as usize).collect::<String>()
                  );
        }
    }
    println!();

    let final_result = cup_config[0] * cup_config[cup_config[0] - 1];

    println!("The final result is {}", final_result);
}

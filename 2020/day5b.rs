use std::io;
use std::io::prelude::*;

fn main() {
    let mut max_id: usize = 0;
    let mut min_id: usize = 1024;
    let mut seats = vec![false; 1024];
    for line in io::stdin().lock().lines().map(Result::unwrap) {
        let id = line
            .chars()
            .fold(0usize, |n, c| {
                if c == 'B' || c == 'R' { (n << 1) | 1 }
                else { n << 1 }
            });

        if id > max_id { max_id = id };
        if id < min_id { min_id = id };
        seats[id] = true;
    }
    assert!(min_id < max_id);
    if let Some(missing) = seats[min_id..=max_id].iter().position(|&b| !b) {
        println!("Your seat is: {}", min_id + missing);
    } else {
        println!("There is no free seat!");
    }
}


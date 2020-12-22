use std::io;
use std::io::prelude::*;

fn main() {
    let mut max_id: u32 = 0;
    for line in io::stdin().lock().lines().map(Result::unwrap) {
        let id = line
            .chars()
            .fold(0u32, |n, c| {
                if c == 'B' || c == 'R' { (n << 1) | 1 }
                else { n << 1 }
            });

        if id > max_id { max_id = id };
    }
    println!("Max id was: {}", max_id);
}


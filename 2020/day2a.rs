#![feature(str_split_once)]

use std::io;
use std::io::prelude::*;

fn main() {
    let mut count = 0;
    for line in io::stdin().lock().lines() {
        let input = line.unwrap();
        let mut iter = input.split(' ');
        let allowed_occur = iter.next().unwrap();

        let (min_occur, max_occur): (u32, u32) = allowed_occur
            .split_once('-')
            .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
            .unwrap();

        let letter = iter.next().unwrap().chars().nth(0).unwrap();
        let password = iter.next().unwrap();

        let occur = password
            .chars()
            .fold(0, move |total, c| if c == letter { total + 1 } else { total });

        if min_occur <= occur && occur <= max_occur {
            count += 1;
        }
    }
    println!("# of valid passwords is {}", count);
}

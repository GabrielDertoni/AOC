#![feature(str_split_once)]

use std::io;
use std::io::prelude::*;

fn main() {
    let mut count = 0;
    for line in io::stdin().lock().lines() {
        let input = line.unwrap();
        let mut iter = input.split(' ');
        let allowed_occur = iter.next().unwrap();

        let (fst_occur, snd_occur): (usize, usize) = allowed_occur
            .split_once('-')
            .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
            .unwrap();

        let letter = iter.next().unwrap().chars().nth(0).unwrap();
        let password = iter.next().unwrap();
        let bytes = password.as_bytes();

        let is_fst = bytes[fst_occur - 1] as char == letter;
        let is_snd = bytes[snd_occur - 1] as char == letter;
        // XOR
        if (is_fst && !is_snd) || (!is_fst && is_snd) {
            count += 1;
        }
    }
    println!("# of valid passwords is {}", count);
}

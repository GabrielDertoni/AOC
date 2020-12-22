#![feature(str_split_once)]

use std::io;
use std::io::prelude::*;

const FIELDS: [&str; 8] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
    "cid",
];

fn parse_fields(line: &str) -> u8 {
    let mut fields = 0;
    for token in line.split(' ') {
        let (field, _) = token.split_once(':').unwrap();
        if let Some((_, idx)) = FIELDS.iter().zip(0..).find(move |(f, _)| **f == field) {
            fields |= 1 << (idx as u8);
        } else {
            unreachable!();
        }
    }
    return fields;
}

fn main() {
    let passports = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .fold(vec![0], |mut passports, line| {
            if line.len() == 0 {
                passports.push(0);
            } else {
                let len = passports.len();
                passports[len - 1] |= parse_fields(&line);
            }
            passports
        });

    let n_valid = passports.iter()
        .map(|passport| (0..7).map(|i| (passport >> i) & 1).all(|f| f == 1))
        .filter(|valid| *valid)
        .count();

    println!("# of valid passports is {}", n_valid);
}

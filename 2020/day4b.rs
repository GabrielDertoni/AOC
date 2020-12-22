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
        let (field, data) = token.split_once(':').unwrap();
        if let Some((_, idx)) = FIELDS.iter().zip(0..).find(move |(f, _)| **f == field) {
            if match field {
                "byr" => {
                    let byr = data.parse().unwrap();
                    1920 <= byr && byr <= 2002
                },
                "iyr" => {
                    let iyr = data.parse().unwrap();
                    2010 <= iyr && iyr <= 2020
                },
                "eyr" => {
                    let eyr = data.parse().unwrap();
                    2020 <= eyr && eyr <= 2030
                },
                "hgt" => {
                    if let Ok(hgt_num) = data[..data.len() - 2].parse() {
                        let hgt_unit = &data[data.len() - 2..];

                        match hgt_unit {
                            "cm" => 150 <= hgt_num && hgt_num <= 193,
                            "in" => 59 <= hgt_num && hgt_num <= 76,
                            _ => false,
                        }
                    } else {
                        false
                    }
                },
                "hcl" => {
                    data.len() == 7 &&
                    data.chars().nth(0).unwrap() == '#' &&
                    data.chars().skip(1).all(|c| c.is_digit(16))
                },
                "ecl" => {
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|opt| *opt == data)
                },
                "pid" => {
                    data.len() == 9 &&
                    data.chars().all(|c| c.is_digit(10))
                },
                "cid" => true,
                _ => false,
            } {
                fields |= 1 << (idx as u8);
            }
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

#![feature(str_split_once)]

use std::io;
use std::io::prelude::*;
use std::ops::RangeInclusive;

struct Rule {
    name: String,
    ranges: Box<[RangeInclusive<usize>]>,
}

impl std::str::FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once(":").ok_or("Expected ':' in rule.")?;
        let ranges: Box<[RangeInclusive<usize>]> = rest.split(" or ")
            .map(|range| {
                let (start, end) = range.trim().split_once("-").ok_or("Expected start-end")?;
                let start_n: usize = start.parse().map_err(|_| "Unable to parse range start")?;
                let end_n: usize = end.parse().map_err(|_| "Unable to parse range end")?;
                Ok(RangeInclusive::new(start_n, end_n))
            })
            .collect::<Result<_, &'static str>>()?;

        Ok(Rule { name: String::from(name), ranges })
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines().map(Result::unwrap);
    let rules: Box<[Rule]> = lines_iter.by_ref()
        .take_while(|l| l.len() > 0) // Take untill empty line.
        .map(|l| l.parse().unwrap())
        .collect();

    let invalid_sum: usize = lines_iter.by_ref()
        .skip_while(|line| !line.starts_with("nearby tickets"))
        .skip(1) // Skip the "nearby tickets" line itself.
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Box<[usize]>>()
        })
        .fold(0, |invalid_sum, values| {
            invalid_sum + values.iter()
                .filter(|value| {
                    rules.iter()
                        .flat_map(|rule| rule.ranges.iter())
                        .all(|range| !range.contains(value))
                })
                .sum::<usize>()
        });

    println!("The sum of all invalid values is {}", invalid_sum);
}

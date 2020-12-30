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

    let ticket: Box<[usize]> = lines_iter.by_ref()
        .skip_while(|line| !line.starts_with("your ticket"))
        .skip(1)
        .nth(0)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let nfields = rules.len();
    let mut matches: Vec<Vec<usize>> = lines_iter.by_ref()
        .skip_while(|line| !line.starts_with("nearby tickets"))
        .skip(1) // Skip the "nearby tickets" line itself.
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Box<[usize]>>()
        })
        .filter(|ticket| {
            // All values must be contained in some rage (valid ticket).
            ticket.iter()
                .all(|value| {
                    rules.iter()
                        .flat_map(|rule| rule.ranges.iter())
                        .any(|range| range.contains(value))
                })
        })
        .fold(vec![(0..nfields).collect(); nfields], |mut matches, ticket| {
            // For each value, search which fields it can fit in.
            for (i, value) in ticket.iter().enumerate() {
                rules.iter()
                    .enumerate()
                    .filter(|(_, rule)| {
                        // Filter rules to which `value` is invalid. (its not contained in any
                        // range)
                        !rule.ranges.iter()
                            .any(|range| range.contains(value))
                    })
                    .for_each(|(j, _)| {
                        // If the match is present, remove it.
                        match matches[j].binary_search(&i) {
                            Err(_) => (),
                            Ok(k)  => { matches[j].remove(k); },
                        }
                    });
            }
            matches
        });

    // There should be at least one rule with only one match. Then that is the first field figured
    // out. rule_indices[i] is the index of the ith rule.
    let mut rule_indices = vec![0_usize; rules.len()];
    for _ in 0..matches.len() {
        // Find the rule with only one match.
        let (i, rule_match) = matches.iter()
            .enumerate()
            .find(|&(_, r)| r.len() == 1)
            .expect("A rule with only one match");

        let rule_index = rule_match[0];
        rule_indices[i] = rule_index;
        // Since we know that rule_index is already solved, remove that possibility from all
        // other matches.
        matches.iter_mut()
            .filter_map(|el| {
                el.iter()
                    .position(|&e| e == rule_index)
                    .zip(Some(el)) // Gives Option<(index, element)>
            })
            .for_each(|(j, el)| { el.swap_remove(j); });
    }

    // matches[i] vector of all possible matches for the ith rule.
    for (i, j) in rule_indices.iter().enumerate() {
        println!("Rule \"{}\" is at position {}.", rules[i].name, j + 1);
    }
    println!();

    let final_ans = rules.iter()
        .enumerate()
        .filter(|(_, rule)| rule.name.starts_with("departure"))
        .map(|(i, _)| ticket[rule_indices[i]])
        .product::<usize>();

    println!("Multiplying all six ticket values starting with departure we get {}", final_ans);
}

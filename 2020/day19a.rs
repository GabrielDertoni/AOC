#![feature(iterator_fold_self)]
#![feature(str_split_once)]

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

enum Rule {
    Char(char),             // Match a single char
    And(Box<(Rule, Rule)>), // Match first rule and then second
    Or(Box<(Rule, Rule)>),  // Match first rule or second
    Ref(u32),               // A reference to another rule
}

fn parse_rule(s: &str) -> Rule {
    let parse = s.trim();
    if let Some('"') = parse.chars().nth(0) {
        // Not really the most elegant approach, but it requires less code :-)
        Rule::Char(parse.chars().nth(1).unwrap()).into()
    } else {
        parse.split('|')
            .map(|sequence| {
                sequence.trim().split(' ')
                    .map(|rule_ref| Rule::Ref(rule_ref.parse().unwrap()))
                    .rev() // Reverse so that first element of tuple is not nested
                    .fold_first(|next, rule_ref| Rule::And((rule_ref, next).into()))
                    .unwrap()
            })
            .rev() // Reverse so that first element of tuple is not nested
            .fold_first(|next, seq| Rule::Or((seq, next).into()))
            .unwrap()
    }
}

/**
 * Returns Some(str) if the test is successfull, then str is the unconsumed part. If the test
 * fails, returns None.
 */
fn test_rule<'a>(s: &'a str, r: &Rule, rules: &HashMap<u32, Rule>) -> Option<&'a str> {
    match r {
        Rule::Ref(idx) => test_rule(s, &rules[idx], rules),
        Rule::Or(b) => {
            let (x, xs) = b.as_ref();
            test_rule(s, x, rules)
                .or_else(|| test_rule(s, xs, rules))
        },
        Rule::And(b) => {
            let (x, xs) = b.as_ref();
            test_rule(s, x, rules)
                .and_then(|rest| test_rule(rest, xs, rules))
        },
        &Rule::Char(c) => {
            s.as_bytes()
                .split_first()
                .and_then(|(fst, _)| {
                    if *fst == c as u8 { Some(&s[1..]) }
                    else { None }
                })
        },
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock()
        .lines()
        .map(Result::unwrap);

    let rules: HashMap<u32, Rule> = lines_iter.by_ref()
        .take_while(|line| line.len() > 0)
        .map(|line| {
            let (rule_idx, rule) = line.split_once(":").unwrap();
            (rule_idx.parse().unwrap(), parse_rule(rule))
        })
        .collect();

    let result = lines_iter.by_ref()
        .filter(|line| {
            test_rule(&line, &rules[&0], &rules)
                .filter(|rest| rest.len() == 0)
                .is_some()
        })
        .count();

    println!("The total number of matching messages is {}", result);
}

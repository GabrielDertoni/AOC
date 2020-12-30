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

type MatchResults<'a> = Box<dyn Iterator<Item = (&'a str, &'a str)> + 'a>;

/**
 * Returns an iterator over all possible matches. This includes matches that do not match de
 * entire string, but only part of it.
 */
fn match_rule<'a>(s: &'a str, r: &'a Rule, rules: &'a HashMap<u32, Rule>) -> MatchResults<'a> {
    match r {
        Rule::Ref(idx) => match_rule(s, &rules[idx], rules),
        Rule::Or(b) => {
            let (x, xs) = b.as_ref();
            // An Or is just all possibilities for the first rule chained with all posibilities
            // for the second rule.
            Box::new({
                match_rule(s, x, rules)
                    .chain(match_rule(s, xs, rules))
            })
        },
        Rule::And(b) => {
            let (x, xs) = b.as_ref();
            Box::new({
                // An And is all matches to the first rule followed by the second. flat_map is
                // used in order to join the first rules's matches with matches also on the
                // second rule.
                match_rule(s, x, rules)
                    .flat_map(move |(pars, rest)| {
                        // For each match to the first rule, match the second one as well.
                        match_rule(rest, xs, rules)
                            .map(move |(p, a)| (&s[..pars.len() + p.len()], a))
                    })
            })
        },
        &Rule::Char(c) => {
            Box::new({
                // Will yield at most a single element.
                s.as_bytes()
                    .split_first()
                    .and_then(|(fst, _)| {
                        if *fst == c as u8 { Some((&s[..1], &s[1..])) }
                        else { None }
                    })
                    .into_iter() // Converts the Option into an iterator.
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
            match_rule(&line, &rules[&0], &rules)
                // We are only interested in matches tha consume the entire string.
                .filter(|(_, rest)| rest.len() == 0)
                .take(1) // Only one match is necessary
                .count() > 0
        })
        .count();

    println!("The total number of matching messages is {}", result);
}

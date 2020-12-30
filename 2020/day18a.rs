use std::io;
use std::io::prelude::*;

/**
 * Sure, there is a simpler more efficient way to solve this. But this is
 * nice and general.
 *
 * Fun note, I originally implemented the parsing functions to return an
 * Option<(T, &str)>, where T is what was parsed by the function and &str
 * would be the rest of the string. However, that worked with left to
 * right evaluation, since the "rest" is always whatever is unconsumed
 * to the left of the original string. In order to change this behaviour
 * to right to left evaluation, all that was needed was a change in the
 * parse_char(), eval_num() and eval_paren() functions, that now return
 * the "rest" as the unconsumed right side of the string.
 */

fn parse_char<'a>(expr: &'a str, c: char) -> Option<(char, &'a str)> {
    expr.chars()
        .last()
        .and_then(|p| {
            if p == c { Some((p, &expr[..expr.len() - 1])) }
            else { None }
        })
}

fn eval_num<'a>(expr: &'a str) -> Option<(i64, &'a str)> {
    let trim = expr.trim();
    let n_digits = trim.chars().rev().take_while(|c| c.is_digit(10)).count();
    trim[trim.len() - n_digits..].parse()
        .ok()
        .map(|p| (p, &trim[..trim.len() - n_digits]))
}

fn eval_paren<'a>(expr: &'a str) -> Option<(i64, &'a str)> {
    let trim = expr.trim();
    Some(trim)
        .and_then(|t| parse_char(t, ')')) // "First" is a ')' because we are reading left to right
        .and_then(|(_, rest)| eval_expr(rest))
        .and_then(|(v, rest)| {
            parse_char(rest, '(')
                .map(|(_, after)| (v, after))
        })
}

fn eval_sum<'a>(expr: &'a str) -> Option<(i64, &'a str)> {
    let trim = expr.trim();
    Some(trim)
        .and_then(|e| eval_paren(e).or_else(|| eval_num(e)))
        .and_then(|(v, rest)| {
            parse_char(rest.trim(), '+')
                .map(|(_, after)| (v, after))
        })
        .and_then(|(lhs, rest)| {
            eval_expr(rest)
                .map(|(rhs, after)| (lhs + rhs, after))
        })
}

fn eval_mult<'a>(expr: &'a str) -> Option<(i64, &'a str)> {
    let trim = expr.trim();
    Some(trim)
        .and_then(|e| eval_paren(e).or_else(|| eval_num(e)))
        .and_then(|(v, rest)| {
            parse_char(rest.trim(), '*')
                .map(|(_, after)| (v, after))
        })
        .and_then(|(lhs, rest)| {
            eval_expr(rest)
                .map(|(rhs, after)| (lhs * rhs, after))
        })
}

fn eval_expr<'a>(expr: &'a str) -> Option<(i64, &'a str)> {
    let parse = expr.trim();
    eval_sum(parse)
        .or_else(|| eval_mult(parse))
        .or_else(|| eval_paren(parse))
        .or_else(|| eval_num(parse))
}

fn main() {
    let stdin = io::stdin();
    let result: i64 = stdin.lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            match eval_expr(&line) {
                Some((result, _)) => result,
                None => panic!("Line \"{}\" had some parsing problems", line),
            }
        })
        .sum();

    println!("Sum of the result of all expressions is {}", result);

}

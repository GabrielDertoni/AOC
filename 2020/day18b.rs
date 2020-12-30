use std::io;
use std::io::prelude::*;

/**
 * This is a bit more elaborate then day18a.rs
 *
 * A Expr is the same as Mult
 * A Mult is Sum * Mult or a Sum
 * A Sum is Cell + Sum or a Cell
 * A Cell is a Paren or a Num
 * A Paren is (Expr)
 * A Num is simply a number.
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
    Some(expr)
        .and_then(|e| parse_char(e.trim(), ')')) // "First" is a ')' because we are reading left to right
        .and_then(|(_, rest)| eval_expr(rest))
        .and_then(|(v, rest)| {
            parse_char(rest.trim(), '(')
                .map(|(_, after)| (v, after))
        })
}

// Sum is Cell + Sum | Cell
fn eval_sum<'a>(expr: &'a str) -> Option<(i64, &'a str)> {
    Some(expr)
        .and_then(eval_cell)
        .and_then(|(v, rest)| {
            parse_char(rest.trim(), '+')
                .map(|(_, after)| (v, after))
        })
        .and_then(|(lhs, rest)| {
            eval_sum(rest)
                .map(|(rhs, after)| (lhs + rhs, after))
        })
        .or_else(|| eval_cell(expr))
}

// Mult is Sum * Mult | Sum
fn eval_mult<'a>(expr: &'a str) -> Option<(i64, &'a str)> {
    Some(expr)
        .and_then(eval_sum)
        .and_then(|(v, rest)| {
            parse_char(rest.trim(), '*')
                .map(|(_, after)| (v, after))
        })
        .and_then(|(lhs, rest)| {
            eval_mult(rest)
                .map(|(rhs, after)| (lhs * rhs, after))
        })
        .or_else(|| eval_sum(expr))
}

// Cell is (Expr) | Num
fn eval_cell<'a>(expr: &'a str) -> Option<(i64, &'a str)> {
    eval_paren(expr)
        .or_else(|| eval_num(expr))
}

// Expr is just a wrapper to Mult
fn eval_expr<'a>(expr: &'a str) -> Option<(i64, &'a str)> { eval_mult(expr) }

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

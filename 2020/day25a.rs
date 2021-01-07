/**
 * The algorithm described in the problem for "transforming" a number is the
 * same as subj_num^loop_size (mod 20201227). Doing a little research on the
 * number 20201227 we realize, it's prime!
 *
 * subj_num^2 (mod 20201227) = a
 *
 * (7^11) % 20201227
 */

use std::io;
use std::io::prelude::*;

const DIV_VAL: usize = 20201227;

// Calculates a^n (mod m).
// NOTE: m < sqrt(usize::MAX)
fn pow_mod(mut a: usize, n: usize, m: usize) -> usize {
    a %= m;
    if a == 0 { 0 }
    else if n == 1 { a % m }
    else if n % 2 == 0 {
        pow_mod((a * a) % m, n / 2, m)
    } else {
        (a * pow_mod(a, n - 1, m)) % m
    }
}

fn transform_value(subj_num: usize, loop_size: usize) -> usize {
    /*
     * This is the algorithm described by the problem. However, its just not
     * fast enough.
     *
     * let mut value = 1;
     * for _ in 0..loop_size {
     *     value *= subj_num;
     *     value %= DIV_VAL;
     * }
     * return value;
    */
    // This is way faster, computes in O(log n) rather then O(n)
    pow_mod(subj_num, loop_size, DIV_VAL)
}

fn find_loop_size(
    subj_num: usize,
    target: usize,
    range: impl Iterator<Item = usize>
) -> Option<usize>
{
    for i in range {
        if transform_value(subj_num, i) == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock()
        .lines()
        .map(Result::unwrap);

    let cards_public_key: usize = lines.next()
        .expect("Expected a first line")
        .parse()
        .expect("First line to be a number");

    let doors_public_key: usize = lines.next()
        .expect("Expected a second line")
        .parse()
        .expect("Second line to be a number");

    let cards_loop_sz = find_loop_size(7, cards_public_key, 1..)
        .expect("Card's loop size should be in range 1..");
    
    // Only a single key needs to be cracked, but this is the code to find the
    // door's secret loop size.
    // let doors_loop_sz = find_loop_size(7, doors_public_key, 1..)
    //     .expect("Door's loop size should be in range 1..");

    println!("Card's loop size is {}", cards_loop_sz);

    let encryption_key = transform_value(doors_public_key, cards_loop_sz);
    println!("Encryption key is {}", encryption_key);
}

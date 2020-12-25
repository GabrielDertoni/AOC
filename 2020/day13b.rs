use std::io;
use std::io::prelude::*;

fn extended_euclid(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        (1, 0)
    } else {
        let (pu, pv) = extended_euclid(b, a % b);
        (pv, pu - pv * (a / b))
    }
}


fn main() {
    let bus_indexed_ids: Box<[(usize, usize)]> = io::stdin().lock()
        .lines()
        .map(Result::unwrap)
        .skip(1)
        .nth(0)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, id)| Some(i).zip(id.parse().ok()))
        .collect();

    let n: i64 = bus_indexed_ids.iter()
        .map(|&(_, id)| id as i64)
        .product();

    let mut result: i64 = bus_indexed_ids.iter()
        .map(|&(i, id)| (i as i64, id as i64))
        .map(|(i, id)| {
            let b = n / id;
            let (inv_b, _) = extended_euclid(b, id);
            let a = (id - i) % id;
            a * b * inv_b
        })
        .sum::<i64>() % n;

    result = if result < 0 { n + result } else { result };
    println!("The earliest time for wich all busses leave in succession is {:?}", result);
}

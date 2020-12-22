use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let target = 2020;

    let mut set = HashSet::<u32>::new();
    for line in io::stdin().lock().lines() {
        let value = line.unwrap().parse().unwrap();
        if let Some(other) = set.get(&(target - value)) {
            println!("Found match! {} + {} = {}", value, other, target);
            println!("Result is: {} * {} = {}", value, other, value * other);
            break;
        }
        set.insert(value);
    }
}

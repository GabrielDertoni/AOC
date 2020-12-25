#![feature(iterator_fold_self)]

use std::io;
use std::io::prelude::*;

fn wait_time(timestamp: u32, bus_id: u32) -> u32 {
    (bus_id - (timestamp % bus_id)) % bus_id
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let t: u32 = lines.next().unwrap().parse().unwrap();
    let best_bus = lines.next().unwrap()
        .split(',')
        .filter_map(|id| id.parse().ok())
        .fold_first(|p: u32, id: u32| if wait_time(t, p) > wait_time(t, id) { id } else { p })
        .expect("There are no busses available!");

    println!("The earliest possible bus has id {} with a wait time of {} minutes.", best_bus, wait_time(t, best_bus));
    println!("Therefore ID * WaitTime = {}", best_bus * wait_time(t, best_bus));
}

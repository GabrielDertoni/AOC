#![feature(linked_list_cursors)]
#![feature(destructuring_assignment)]

use std::io;
use std::io::prelude::*;
use std::collections::LinkedList;
use std::collections::linked_list::CursorMut;

type Digit = u8;

// Receives a cursor into a linked list. Splits the next three elements from the
// list and reconstructs the starting list without those three elements. The
// list without the split list has as its last element the cursor current item.
// Two lists are returned: first the list with all elements except the next
// three, second a list containing only the three removed elements.
fn splice_next_three(cursor: &mut CursorMut<'_, Digit>) -> (LinkedList<Digit>, LinkedList<Digit>) {
    let mut splice_three = cursor.split_after();
    splice_three.append(&mut cursor.split_before());
    splice_three.append(&mut cursor.remove_current_as_list().unwrap());
    let mut cut_cursor = splice_three.cursor_front_mut();
    for _ in 0..2 {
        cut_cursor.move_next();
    }
    let rest = cut_cursor.split_after();
    (rest, splice_three)
}

fn main() {
    let n_iters = 100;

    let stdin = io::stdin();
    let mut cup_config: LinkedList<Digit> = stdin.lock()
        .lines()
        .map(Result::unwrap)
        .take(1)
        .map(|line| {
            line.chars()
                .map(|c| c as u8 - b'0')
                .collect()
        })
        .nth(0)
        .unwrap();

    let max_cup = *cup_config.iter().max().unwrap();
    let min_cup = *cup_config.iter().min().unwrap();

    // Initialy, the current cup is at the front of the list.
    let mut curr_cup = cup_config.cursor_front_mut();
    for _ in 0..n_iters {
        let removed;
        // Receives the new configuration of cups without the removed cups.
        // NOTE: Now the current cup is at the end of `cup_config`. See
        // `splice_next_three()`
        (cup_config, removed) = splice_next_three(&mut curr_cup);

        // The cup we are looking for
        let mut destination_cup = *cup_config.back().unwrap() - 1;

        // A cursor to find the destination cup. We know that the current cup is
        // at the back of `cup_config`, therefore we can start searching in the
        // front to go through every element until arriving back to current cup.
        let mut destination_cursor = cup_config.cursor_front_mut();

        // While we don't find the destination cup, keep moving forward (clockwise).
        // If we hit the end of the list, `destination_cup` is not in `cup_config`.
        // Start over with a new `destination_cup` according to game rules.
        while match destination_cursor.current() {
            Some(&mut v) => v != destination_cup,
            None     => {
                destination_cup = if destination_cup <= min_cup { max_cup } else { destination_cup - 1 };
                true
            },
        } {
            destination_cursor.move_next();
        }
        // Put all those removed cups after the destination cup.
        destination_cursor.splice_after(removed);
        // We know the current cup is at the back, so to get the next one, we
        // go to the front!
        curr_cup = cup_config.cursor_front_mut();
    }
    let final_config: String = cup_config.iter()
        .cycle()
        .skip_while(|&&cup| cup != 1)
        .skip(1) // Skip the 1 itself
        .map(|&cup| (cup + b'0') as char)
        .take(cup_config.len() - 1)
        .collect();

    println!("The final configuration is {}", final_config);
}

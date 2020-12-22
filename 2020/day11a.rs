#![feature(iter_map_while)]

use std::io;
use std::io::prelude::*;
use std::mem::swap;
use Place::{ Occupied, Free, Floor };


#[derive(PartialEq, Eq, Clone)]
enum Place {
    Occupied,
    Free,
    Floor,
}

fn parse_place(place: char) -> Place {
    match place {
        'L' => Free,
        '.' => Floor,
        '#' => Occupied,
        _   => unreachable!(),
    }
}

fn count_occupied_neighbors(x: usize, y: usize, layout: &Box<[Box<[Place]>]>) -> u8 {
    let mut count = 0;
    for i in 0..=2 {
        let dx: i32 = x as i32 + i as i32 - 1;
        if 0 <= dx && dx < layout.len() as i32 {
            for j in 0..=2 {
                let dy: i32 = y as i32 + j as i32 - 1;
                if i == 1 && j == 1 { continue };
                if 0 <= dy && dy < layout[dx as usize].len() as i32 {
                    count += if layout[dx as usize][dy as usize] == Occupied { 1 } else { 0 };
                }
            }
        }
    }
    count
}

// NOTE: Rick: That just sounds like Conway's Game of Life with fewer steps.
fn main() {
    let mut layout: Box<[Box<[Place]>]> = io::stdin().lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().map(parse_place).collect())
        .collect();

    let mut tmp_layout = layout.clone();
    let mut curr_layout = &mut layout;
    let mut other_layout = &mut tmp_layout;
    // let mut count = 0;
    // A bit of a functional-imperative mix. Courtesy of the Rust language :-).
    // NOTE: I now perfectly well that I could use scan... I just wont.
    let n_seated: usize = (0..).map_while(|_| {
        let mut changed = false;
        for i in 0..curr_layout.len() {
            for j in 0..curr_layout[i].len() {
                other_layout[i][j] = match curr_layout[i][j] {
                    Free => {
                        if count_occupied_neighbors(i, j, &curr_layout) == 0 {
                            changed = true;
                            Occupied
                        } else {
                            Free
                        }
                    },
                    Occupied => {
                        if count_occupied_neighbors(i, j, &curr_layout) >= 4 {
                            changed = true;
                            Free
                        } else {
                            Occupied
                        }
                    },
                    Floor => Floor,
                }
            }
        }
        let n_occupied: usize = other_layout.iter()
            .map(|line| line.iter().filter(|&p| *p == Occupied).count())
            .sum();

        swap(&mut curr_layout, &mut other_layout);

        if changed {
            Some(n_occupied)
        } else {
            None
        }
    })
    .last()
    .unwrap();

    println!("# seated people {}", n_seated);
}

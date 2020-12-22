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
        let dx = i as i32 - 1;

        for j in 0..=2 {
            let dy = j as i32 - 1;

            if dx == 0 && dy == 0 { continue };

            for mult in 1.. {
                let offx = x as i32 + dx * mult;
                let offy = y as i32 + dy * mult;

                if 0 <= offx && offx < layout.len() as i32 &&
                   0 <= offy && offy < layout[offx as usize].len() as i32 {
                    match layout[offx as usize][offy as usize] {
                        Occupied => {
                            count += 1;
                            break;
                        },
                        Free  => break,
                        Floor => continue,
                    }
                } else {
                    break;
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

    let mut count = 0;
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
                        if count_occupied_neighbors(i, j, &curr_layout) >= 5 {
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

        count += 1;

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

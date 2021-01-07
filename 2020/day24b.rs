/**
 * For more info, read day24a.rs
 *
 * This is basically Conway's Game of Life... Again...
 * But now, we don't have very many neighbors to check and a much more complex
 * coordinate sistem. It would be hard to allocate a matrix to store the world
 * as it could grow to any direction. However, there aren't as many neighbors
 * to check (only 6).
 *
 * This solution marks all black tiles' positions in a hash set. Then, it builds
 * a hash map of tiles to neighbor counts. Finally, it updates the set of black
 * tiles according to the rules.
 */

use std::io;
use std::io::prelude::*;
use std::iter;
use std::ops::{ Add, Sub, Mul };
use std::collections::{ HashSet, HashMap };

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector(i32, i32);

impl Vector {
    fn new(i: i32, j: i32) -> Self {
        Self(i, j)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl iter::Sum for Vector {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Vector>,
    {
        let mut acc: Vector = Vector::new(0, 0);
        while let Some(val) = iter.next() {
            acc = acc + val;
        }
        acc
    }
}

type ParserResult<'a, T> = Option<(T, &'a str)>;

trait Parser {
    type Output;

    fn parse<'a>(&self, s: &'a str) -> ParserResult<'a, Self::Output>;
}

impl<F, Out> Parser for F
where
    F: Fn(&'_ str) -> ParserResult<'_, Out>,
{
    type Output = Out;

    fn parse<'a>(&self, s: &'a str) -> ParserResult<'a, Out> {
        self(s)
    }
}

fn parse_vector(s: &'_ str) -> ParserResult<'_, Vector> {
    let mut chars = s.chars();
    chars.next()
        .and_then(|c| match c {
            'e' => Some((Vector::new( 1,  0), chars.as_str())),
            'w' => Some((Vector::new(-1,  0), chars.as_str())),
            's' => chars.next().and_then(|c2| match c2 {
                'e' => Some((Vector::new( 1, -1), chars.as_str())),
                'w' => Some((Vector::new( 0, -1), chars.as_str())),
                _   => None,
            }),
            'n' => chars.next().and_then(|c2| match c2 {
                'e' => Some((Vector::new( 0,  1), chars.as_str())),
                'w' => Some((Vector::new(-1,  1), chars.as_str())),
                _   => None,
            }),
            _   => None,
        })
}

fn parse_some<P: Parser>(s: &'_ str, parser: P) -> Vec<<P as Parser>::Output> {
    let mut acc = Vec::new();
    let mut p = s;
    while let Some((parsed, rest)) = parser.parse(p) {
        acc.push(parsed);
        p = rest;
    }
    acc
}

// All six directions in the hex grid.
const DIRS: &'static [Vector] = &[ Vector( 1,  0) // East
                                 , Vector(-1,  0) // West
                                 , Vector( 1, -1) // Southeast
                                 , Vector( 0, -1) // Southwest
                                 , Vector( 0,  1) // Northeast
                                 , Vector(-1,  1) // Northwest
                                 ];

// Returns a Hash Map of positions to the number of black tile neighbors. So if
// a given tile is black (it is contained in `black_set`) and its entry in the
// hash map is some `n`, then `n` is the number of black neighbors it has.
fn map_neighbors(black_set: &HashSet<Vector>) -> HashMap<Vector, u8> {
    let mut map = HashMap::new();

    // Iterate through all positions of black tiles.
    for position in black_set.iter() {
        let mut n_neighbors = 0;

        // Iterate through all neighbors of the current black tile.
        for dir in DIRS.iter() {
            let neighbor = *position + *dir;

            // If the neighbor is not a black tile, increase its neighbor count
            // in the map. If it is black, it will insert its own count in a
            // different iteration of the loop.
            if !black_set.contains(&neighbor) {
                if let Some(count) = map.get_mut(&neighbor) {
                    *count += 1;
                } else {
                    map.insert(neighbor, 1);
                }
            } else {
                // This tile has a neighbor!
                n_neighbors += 1;
            }
        }
        map.insert(*position, n_neighbors);
    }
    map
}

// Change the current state into the next.
fn advance_state(black_set: &mut HashSet<Vector>) {
    let map = map_neighbors(black_set);

    for (tile_pos, n_neighbors) in map {
        if black_set.contains(&tile_pos) {
            // The current tile is black, apply rules.
            if n_neighbors == 0 || n_neighbors > 2 {
                black_set.remove(&tile_pos);
            }
        } else {
            // The current tile is white, apply rules.
            if n_neighbors == 2 {
                black_set.insert(tile_pos);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut black_set: HashSet<Vector> = stdin.lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            parse_some(&line, parse_vector).into_iter().sum()
        })
        .fold(HashSet::new(), |mut set, vector| {
            if set.contains(&vector) {
                set.remove(&vector);
            } else {
                set.insert(vector);
            }
            set
        });

    for _ in 0..100 {
        advance_state(&mut black_set);
    }

    println!("There are {} black tiles at the end.", black_set.len());
}

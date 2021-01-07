/**
 * To facilitate positioning of the tiles, a different coordinates system can
 * be used. This system is chosen to work in such a way that integer values
 * may uniquely represent the position of a single hexagonal tile. To to that
 * we can consider that all tiles have radius 1 (the circumscribed circle).
 * Then, we define a vector base B = (i, j) where i and j are as follows:
 *
 * i = (sqrt(3), 0)
 * j = (sqrt(3)/2, 3/2)
 *
 * In this way, a point A = (1, 0) represents the tile east of the tile at (0, 0),
 * B = (0, 1) is the tile at northeast. The vectors pointing to all six directions
 * are:
 *
 * E  = ( 1,  0)
 * W  = (-1,  0)
 * NE = ( 0,  1)
 * SW = ( 0, -1)
 * SE = ( 1, -1)
 * NW = (-1,  1)
 *
 * From that, a tile at P = (5, 2) has a neighbor to Northwest at P + NW = (4, 3).
 */

use std::io;
use std::io::prelude::*;
use std::iter;
use std::ops::{ Add, Sub, Mul };
use std::collections::HashSet;

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

// Parses a single direction from a string, leaving an unconsumed tail and also
// returning it. May consume one or two characters.
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

// Takes any parser and applies it as many times as it can to the input.
fn parse_some<P: Parser>(s: &'_ str, parser: P) -> Vec<<P as Parser>::Output> {
    let mut acc = Vec::new();
    let mut p = s;
    while let Some((parsed, rest)) = parser.parse(p) {
        acc.push(parsed);
        p = rest;
    }
    acc
}

fn main() {
    let stdin = io::stdin();
    let coordinates: HashSet<Vector> = stdin.lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            // Sum all parsed vectors to get the position of the desired tile.
            parse_some(&line, parse_vector).into_iter().sum()
        })
        .fold(HashSet::new(), |mut set, vector| {
            // If a tile is already black, flip it to white. Otherwise, mark it
            // as black.
            if set.contains(&vector) {
                set.remove(&vector);
            } else {
                set.insert(vector);
            }
            set
        });

    println!("There are {} black tiles at the end.", coordinates.len());
}

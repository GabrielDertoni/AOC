use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

/**
 * All that matters are the borders of the tiles, the content inside of it is
 * irrelevant.
 *
 * Because we have the garantee that "tiles at the edge of the image also have
 * this border, but the outermost edges won't line up with any other tiles",
 * we can assume that if a border doesn't match any other border, it must be
 * an outer edge.
 *
 * Furthermore we only need to calculate the multiplyied corner id. Note that
 * there are many valid configuratiosn. If a given configuration of tiles is
 * valid, fliping or rotating that configuration also yields valid configurations.
 * However, a corner tile is always a corner tile, and because of the commutative
 * and associative properties of multiplication, the order of those corners
 * doesn't matter.
 *
 * A corner tile is just a tile that has no match for two of its borders.
 */

struct Tile {
    id: usize,
    top: String,
    right: String,
    bottom: String,
    left: String,
    neighbors: [bool; 4],
}

impl Tile {
    fn new(id: usize, tile: String) -> Self {
        let top = tile.lines().nth(0).unwrap().to_string();

        let right = tile.lines()
            .map(|l| l.chars().last().unwrap())
            .collect();

        let bottom = tile.lines()
            .last()
            .unwrap()
            .to_string();

        let left = tile.lines()
            .map(|l| l.chars().nth(0).unwrap())
            .collect();

        let neighbors = [false; 4];

        Self { id, top, right, bottom, left, neighbors }
    }

    fn borders(&self) -> [&str; 4] {
        [&self.top, &self.right, &self.bottom, &self.left]
    }

    fn n_neighbors(&self) -> u8 {
        self.neighbors.iter().filter(|&&x| x).count() as u8
    }
}

fn border_eq(a: &str, b: &str) -> bool {
    if a == b { true }
    else if a.len() != b.len() { false }
    else {
        a.chars()
            .zip(b.chars().rev())
            .all(|(x, y)| x == y)
    }
}

fn lookup_border<'a>(border: &str, table: &'a mut HashMap<String, Vec<usize>>) -> Option<&'a mut Vec<usize>> {
    if table.contains_key(border) {
        table.get_mut(border)
    } else {
        let flipped: String = border.chars().rev().collect();
        table.get_mut(flipped.as_ref() as &str)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock()
        .lines()
        .map(Result::unwrap);

    let mut tiles: HashMap<usize, Tile> = HashMap::new();
    let mut border_lookup: HashMap<String, Vec<usize>> = HashMap::new();
    while let Some(header) = lines_iter.by_ref().nth(0) {
        let id: usize = header
            .strip_prefix("Tile ").unwrap()
            .strip_suffix(":").unwrap()
            .parse().unwrap();

        let tile: String = lines_iter.by_ref()
            .take_while(|l| l.len() > 0)
            .collect::<Vec<String>>()
            .join("\n");

        let mut tile = Tile::new(id, tile);
        let borders: Box<[String]> = tile.borders().iter().map(|b| b.to_string()).collect();
        for (i, border) in borders.iter().enumerate() {
            match lookup_border(border, &mut border_lookup) {
                Some(v) => {
                    for other in v.iter() {
                        if *other == tile.id { continue };
                        let other_tile = tiles.get_mut(other).unwrap();
                        let j = other_tile.borders()
                            .iter()
                            .position(|&b| border_eq(b, border))
                            .unwrap();

                        other_tile.neighbors[j] = true;
                        tile.neighbors[i] = true;
                    }
                    v.push(id);
                },
                None    => { border_lookup.insert(border.to_string(), vec![id]); },
            }
        }
        tiles.insert(id, tile);
    }

    let prod: usize = tiles.iter()
        .filter(|(_, tile)| tile.n_neighbors() == 2)
        .map(|(id, _)| *id)
        .product();

    println!("The product of the four corners is {}", prod);
}

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;

/**
 * Assumptions:
 *  - There is no tile with id 0.
 *  - There are at most two matching borders in any orientation.
 *  - All tiles are square
 *  - The number of tiles is a perfect square
 *
 * Orientation of the borders:
 *    -->
 *  | ### |   index of the border Image1D increases according
 *  | ### |   to the arrows.
 *  v ### v  
 *    -->
 * 
 */

// Not super elegant, but it works, and you can still se the monster!
const MONSTER: &'static str = "                  # \n\
                               #    ##    ##    ###\n \
                                #  #  #  #  #  #   ";

type Pixel = bool;
type Image1D = Box<[Pixel]>;
type Image2D = Box<[Image1D]>;

struct Tile {
    id: usize,
    borders: [Image1D; 4],
    inner: Image2D,
    neighbors: [usize; 4],
}

impl Tile {
    fn new(id: usize, tile: Image2D) -> Self {
        let inner: Image2D = tile[1..tile.len()-1].iter()
            .map(|line| line[1..line.len()-1].as_ref().into())
            .collect();

        let top = tile[0].clone();
        let right = tile.iter().map(|l| l[l.len()-1]).collect();
        let bottom = tile[tile.len()-1].clone();
        let left = tile.iter().map(|l| l[0]).collect();

        let neighbors = [0; 4];
        let borders = [top, right, bottom, left];

        Self { id, borders, inner, neighbors }
    }

    fn transform_to_match(&mut self, border: &Image1D, conn_from: usize) {
        let conn_to = (conn_from + 2) % 4;
        let i = self.borders.iter()
            .position(|b| border_eq(&b, border))
            .unwrap();

        if conn_to > i { self.rotate_right(conn_to - i); }
        else if i > conn_to { self.rotate_left(i - conn_to); }

        // If they still aren't equal, conn_to border must be flipped.
        if self.borders[conn_to] != *border {
            // if conn_to border is top or bottom (conn_to % 2 = 0), flip Y, else flip X.
            if conn_to % 2 == 0 { self.flip_y(); }
            else { self.flip_x(); }
        }
    }

    fn flip_x(&mut self) {
        self.borders.swap(0, 2);
        self.neighbors.swap(0, 2);
        self.borders[1].reverse();
        self.borders[3].reverse();
        flip_image_x(&mut self.inner);
    }

    fn flip_y(&mut self) {
        self.borders.swap(1, 3);
        self.neighbors.swap(1, 3);
        self.borders[0].reverse();
        self.borders[2].reverse();
        flip_image_y(&mut self.inner);
    }

    fn rotate_right(&mut self, n: usize) {
        for _ in 0..n {
            rotate_image_right(&mut self.inner);
            self.borders.rotate_right(1);
            self.neighbors.rotate_right(1);
            self.borders[0].reverse();
            self.borders[2].reverse();
        }
    }

    fn rotate_left(&mut self, n: usize) {
        for _ in 0..n {
            rotate_image_left(&mut self.inner);
            self.borders.rotate_left(1);
            self.neighbors.rotate_left(1);
            self.borders[1].reverse();
            self.borders[3].reverse();
        }
    }

    fn top_neighbor(&self) -> Option<usize> {
        if self.neighbors[0] == 0 { None }
        else { Some(self.neighbors[0]) }
    }

    fn right_neighbor(&self) -> Option<usize> {
        if self.neighbors[1] == 0 { None }
        else { Some(self.neighbors[1]) }
    }

    fn bottom_neighbor(&self) -> Option<usize> {
        if self.neighbors[2] == 0 { None }
        else { Some(self.neighbors[2]) }
    }

    fn left_neighbor(&self) -> Option<usize> {
        if self.neighbors[3] == 0 { None }
        else { Some(self.neighbors[3]) }
    }

    fn n_neighbors(&self) -> u8 {
        self.neighbors.iter().filter(|&&x| x != 0).count() as u8
    }
}

fn border_eq(a: &Image1D, b: &Image1D) -> bool {
    if a.len() != b.len() { false } // This should never actually happen.
    else if *a == *b { true }
    else {
        a.iter()
            .zip(b.iter().rev())
            .all(|(x, y)| x == y)
    }
}

fn lookup_border<'a>(border: &Image1D, table: &'a mut HashMap<Image1D, Vec<usize>>) -> Option<&'a mut Vec<usize>> {
    if table.contains_key(border) {
        table.get_mut(border)
    } else {
        let mut flipped: Image1D = border.clone();
        flipped.reverse();
        table.get_mut(flipped.as_ref())
    }
}

fn generate_config_bfs(tiles: &mut HashMap<usize, Tile>) -> Vec<Vec<usize>> {
    // We know its suposed to be a square.
    let side_len = (tiles.len() as f32).sqrt() as usize;
    let mut tile_config: Vec<Vec<usize>> = vec![vec![0; side_len]; side_len];

    // Queue has elements of (x, y, connecting_border, connection_idx, tile_id)
    let mut queue: VecDeque<(usize, usize, Image1D, usize, usize)> = VecDeque::new();
    let (_, corner_tile) = tiles.iter_mut()
        .find(|(_, t)| t.n_neighbors() == 2)
        .unwrap();

    // Transforms the corner found to be the top left corner, because it will certanly have
    // position (0, 0). In order to do that, there must be no top nor left neighbors.
    if corner_tile.top_neighbor().is_some()  { corner_tile.flip_x(); }
    if corner_tile.left_neighbor().is_some() { corner_tile.flip_y(); }

    queue.push_back((0, 0, corner_tile.borders[0].clone(), 2, corner_tile.id));

    while !queue.is_empty() {
        let (x, y, connecting_border, conn_idx, curr_id) = queue.pop_back().unwrap();
        // If we have already visited, continue.
        if tile_config[y][x] != 0 { continue };
        let curr = tiles.get_mut(&curr_id).unwrap();
        tile_config[y][x] = curr.id;
        curr.transform_to_match(&connecting_border, conn_idx);

        if let Some(id) = curr.top_neighbor() {
            queue.push_back((x, y - 1, curr.borders[0].clone(), 0, id));
        }
        if let Some(id) = curr.right_neighbor() {
            queue.push_back((x + 1, y, curr.borders[1].clone(), 1, id));
        }
        if let Some(id) = curr.bottom_neighbor() {
            queue.push_back((x, y + 1, curr.borders[2].clone(), 2, id));
        }
        if let Some(id) = curr.left_neighbor() {
            queue.push_back((x - 1, y, curr.borders[3].clone(), 3, id));
        }
    }
    tile_config
}

fn construct_image(tile_config: &Vec<Vec<usize>>, tiles: &HashMap<usize, Tile>) -> Image2D {
    let mut image = Vec::new();
    // Assumes all tiles have same dimensions.
    let tile_height = tiles[&tile_config[0][0]].inner.len();
    for row in tile_config.iter() {
        row.iter()
            .map(|tile| tiles[tile].inner.iter()) // Gets line iterators for each tile in the row.
            // Creates a vector where the ith element is the concatenation of the ith line on each tile.
            .fold(vec![Vec::new(); tile_height], |mut lines, next| {
                // Joins together the accumulating lines with the lines on the next line iterator.
                lines.iter_mut()
                    .zip(next)
                    .for_each(|(line, s)| line.append(&mut s.to_vec()));
                lines
            })
            .into_iter()
            .for_each(|line| image.push(line));
    }
    image.into_iter()
        .map(|line| line.into_boxed_slice())
        .collect()
}

// Rotate right is the same as matrix transpose followed by flip y.
fn rotate_image_right(image: &mut Image2D) {
    transpose_image(image);
    flip_image_y(image);
}

fn rotate_image_left(image: &mut Image2D) {
    flip_image_y(image);
    transpose_image(image);
}

// NOTE: Assumes a square image
fn transpose_image(image: &mut Image2D) {
    let side = image.len();
    for i in 0..side {
        for j in i + 1..side {
            let (top, bot) = image.split_at_mut(i + 1);
            std::mem::swap(&mut top[i][j], &mut bot[j - i - 1][i]);
        }
    }
}

fn flip_image_y(image: &mut Image2D) {
    image.iter_mut()
        .for_each(|l| l.reverse())
}

fn flip_image_x(image: &mut Image2D) {
    image.reverse()
}

fn match_monster(image: &Image2D) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();

    let im_height = image.len();
    let im_width = image[0].len();

    let monster: Image2D = MONSTER.lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    let monster_height = monster.len();
    let monster_width = monster[0].len();

    for i in 0..im_height - monster_height {
        for j in 0..im_width - monster_width {
            let mut is_match = true;
            for mi in 0..monster_height {
                for mj in 0..monster_width {
                    if monster[mi][mj] && !image[i + mi][j + mj] {
                        is_match = false;
                        break;
                    }
                }
                if !is_match { break };
            }
            if is_match {
                matches.push((j, i));
            }
        }
    }
    matches
}

fn show_image(img: &Image2D, matches: &Vec<(usize, usize)>) {
    let monster: Image2D = MONSTER.lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    for (i, row) in img.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Some((x, y)) = matches.iter().find(|(x, y)| {
                let xoff = j as i32;
                let yoff = i as i32;
                (xoff-19..=xoff).contains(&(*x as i32)) && (yoff-2..=yoff).contains(&(*y as i32))
            }) {
                if monster[i - y][j - x] {
                    print!("\x1b[1m\x1b[33mO\x1b[0m");
                } else {
                    print!("{}", if *cell { '#' } else { '.' });
                }
            } else {
                print!("{}", if *cell { '#' } else { '.' });
            }
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock()
        .lines()
        .map(Result::unwrap);

    let mut tiles: HashMap<usize, Tile> = HashMap::new();
    let mut border_lookup: HashMap<Image1D, Vec<usize>> = HashMap::new();
    while let Some(header) = lines_iter.by_ref().nth(0) {
        let id: usize = header
            .strip_prefix("Tile ").unwrap()
            .strip_suffix(":").unwrap()
            .parse().unwrap();

        let tile: Image2D = lines_iter.by_ref()
            .take_while(|l| l.len() > 0)
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();

        let mut tile = Tile::new(id, tile);
        let borders: [Image1D; 4] = tile.borders.clone();
        for (i, border) in borders.iter().enumerate() {
            match lookup_border(border, &mut border_lookup) {
                Some(v) => {
                    for other in v.iter() {
                        if *other == tile.id { continue };
                        let other_tile = tiles.get_mut(other).unwrap();
                        let j = other_tile.borders.iter()
                            .position(|b| border_eq(b, border))
                            .unwrap();

                        other_tile.neighbors[j] = tile.id;
                        tile.neighbors[i] = *other;
                    }
                    v.push(id);
                },
                None    => { border_lookup.insert(border.clone(), vec![id]); },
            }
        }
        tiles.insert(id, tile);
    }

    let tile_config = generate_config_bfs(&mut tiles);
    println!("The moster is:\n{}", MONSTER);
    println!();
    let mut image = construct_image(&tile_config, &tiles);
    let mut max_found = 0;
    for _flip in 0..=1 {
        for _rot in 0..4 {
            let matches = match_monster(&image);
            if matches.len() > 0 { show_image(&image, &matches); }
            if matches.len() > max_found { max_found = matches.len(); }
            rotate_image_right(&mut image);
        }
        flip_image_y(&mut image);
    }

    let n_parts = MONSTER.chars().filter(|&c| c == '#').count();
    let total_count: usize = image.iter().map(|l| l.iter().filter(|&&x| x).count()).sum();
    println!("The number of # that are not part of any moster is {}", total_count - n_parts * max_found);
}

use std::io;
use std::io::prelude::*;

/**
 * We only need to simulate a few iterations. Therefore, there is only so far the farthest
 * active cube from the origin can be. In particular, it can expand at most n positions to
 * any direction, where n is the number of iterations.
 */

type Vec2D<T> = Vec<Vec<T>>;
type Vec3D<T> = Vec<Vec2D<T>>;

fn empty_plate(width: usize, height: usize) -> Vec2D<u8> {
    return vec![vec!['.' as u8; height]; width];
}

fn count_neighbors(x: i32, y: i32, z: i32, cube: &Vec3D<u8>) -> u8 {
    let mut count = 0;
    for k in 0..=2 {
        let dz = k as i32 - 1;
        let zoff = (z + dz) as usize;
        if !(0..cube.len()).contains(&zoff) { continue };
        for i in 0..=2 {
            let dx = i as i32 - 1;
            let xoff = (x + dx) as usize;
            if !(0..cube[zoff].len()).contains(&xoff) { continue };
            for j in 0..=2 {
                let dy = j as i32 - 1;
                let yoff = (y + dy) as usize;
                if !(0..cube[zoff][xoff].len()).contains(&yoff) { continue };
                if dx == dy && dx == dz && dx == 0 { continue };
                if cube[zoff][xoff][yoff] == '#' as u8 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let n = 6;

    let starting_plate: Vec2D<u8> = stdin.lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let prefix: Vec<u8> = std::iter::repeat('.' as u8).take(n).collect();
            let suffix: Vec<u8> = prefix.clone();
            vec![prefix, line.into_bytes(), suffix].concat()
        })
        .collect();

    let before = empty_plate(n, starting_plate[0].len());
    let after = before.clone();

    let base_plate = vec![before, starting_plate, after].concat();

    let width = base_plate.len();
    let height = base_plate[0].len();
    let above: Vec3D<u8> = std::iter::repeat(empty_plate(width, height)).take(n).collect();
    let below: Vec3D<u8> = above.clone();
    // Not really a cube, but you get the point. Also, that will be pretty big. But still, we'll be
    // fine.
    let mut cube = vec![above, vec![base_plate], below].concat();
    let mut copy = cube.clone();

    let mut curr = &mut cube;
    let mut tmp = &mut copy;

    for _iter in 0..n {
        for k in 0..curr.len() {
            for i in 0..curr[k].len() {
                for j in 0..curr[k][i].len() {
                    let n_neighbors = count_neighbors(i as i32, j as i32, k as i32, curr);
                    if curr[k][i][j] == '.' as u8 && n_neighbors == 3 {
                        tmp[k][i][j] = '#' as u8;
                    } else if curr[k][i][j] == '#' as u8 && (n_neighbors < 2 || n_neighbors > 3) {
                        tmp[k][i][j] = '.' as u8;
                    } else {
                        tmp[k][i][j] = curr[k][i][j];
                    }
                }
            }
        }
        std::mem::swap(&mut curr, &mut tmp);
    }

    let mut count = 0;
    for i in 0..cube.len() {
        for j in 0..cube[i].len() {
            for k in 0..cube[i][j].len() {
                if cube[i][j][k] == '#' as u8 {
                    count += 1;
                }
            }
        }
    }
    println!("At the end of {} iterations there were {} cubes active.", n, count);
}

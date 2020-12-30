use std::io;
use std::io::prelude::*;

/**
 * We only need to simulate a few iterations. Therefore, there is only so far the farthest
 * active cube from the origin can be. In particular, it can expand at most n positions to
 * any direction, where n is the number of iterations.
 */

type Vec2D<T> = Vec<Vec<T>>;
type Vec3D<T> = Vec<Vec2D<T>>;
type Vec4D<T> = Vec<Vec3D<T>>;

fn empty_line(width: usize) -> Vec<u8> {
    return vec!['.' as u8; width];
}

fn empty_plate(height: usize, width: usize) -> Vec2D<u8> {
    return vec![empty_line(width); height];
}

fn empty_cube(depth: usize, height: usize, width: usize) -> Vec3D<u8> {
    return vec![empty_plate(height, width); depth];
}

fn empty_hypercube(whatever_comes_after_depth_idk: usize, depth: usize, height: usize, width: usize) -> Vec4D<u8> {
    return vec![empty_cube(depth, height, width); whatever_comes_after_depth_idk];
}

fn count_neighbors(x: i32, y: i32, z: i32, w: i32, hypercube: &Vec4D<u8>) -> u8 {
    let mut count = 0;
    for q in 0..=2 {
        let dw = q as i32 - 1;
        let woff = (w + dw) as usize;
        if !(0..hypercube.len()).contains(&woff) { continue };
        for k in 0..=2 {
            let dz = k as i32 - 1;
            let zoff = (z + dz) as usize;
            if !(0..hypercube[woff].len()).contains(&zoff) { continue };
            for j in 0..=2 {
                let dy = j as i32 - 1;
                let yoff = (y + dy) as usize;
                if !(0..hypercube[woff][zoff].len()).contains(&yoff) { continue };
                for i in 0..=2 {
                    let dx = i as i32 - 1;
                    let xoff = (x + dx) as usize;
                    if !(0..hypercube[woff][zoff][yoff].len()).contains(&xoff) { continue };
                    if dx == dy && dx == dz && dx == dw && dx == 0 { continue };
                    if hypercube[woff][zoff][yoff][xoff] == '#' as u8 {
                        count += 1;
                    }
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
        .map(|line| vec![empty_line(n), line.into_bytes(), empty_line(n)].concat())
        .collect();

    let width = starting_plate[0].len();

    let base_plate = vec![ empty_plate(n, width)
                         , starting_plate
                         , empty_plate(n, width)
                         ].concat();

    let height = base_plate.len();
    let base_cube = vec![ empty_cube(n, height, width)
                        , vec![base_plate]
                        , empty_cube(n, height, width)
                        ].concat();

    let depth = base_cube.len();
    let mut hyper_cube = vec![ empty_hypercube(n, depth, height, width)
                             , vec![base_cube]
                             , empty_hypercube(n, depth, height, width)
                             ].concat();

    let mut copy = hyper_cube.clone();

    let mut curr = &mut hyper_cube;
    let mut tmp = &mut copy;

    for iter in 0..n {
        for q in 0..curr.len() {
            for k in 0..curr[q].len() {
                for j in 0..curr[q][k].len() {
                    for i in 0..curr[q][k][j].len() {
                        let n_neighbors = count_neighbors(i as i32, j as i32, k as i32, q as i32, curr);
                        if curr[q][k][j][i] == '.' as u8 && n_neighbors == 3 {
                            tmp[q][k][j][i] = '#' as u8;
                        } else if curr[q][k][j][i] == '#' as u8 && (n_neighbors < 2 || n_neighbors > 3) {
                            tmp[q][k][j][i] = '.' as u8;
                        } else {
                            tmp[q][k][j][i] = curr[q][k][j][i];
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut curr, &mut tmp);

        println!("Iteration {} finished", iter + 1);
    }

    let mut count = 0;
    for q in 0..curr.len() {
        for k in 0..curr[q].len() {
            for i in 0..curr[q][k].len() {
                for j in 0..curr[q][k][i].len() {
                    if curr[q][k][i][j] == '#' as u8 {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("At the end of {} iterations there were {} hypercubes active.", n, count);
}

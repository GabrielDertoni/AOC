use std::io;
use std::io::prelude::*;
use Instruction::{ North, East, West, South, Left, Right, Forward };

#[derive(Debug, Clone, Copy)]
enum Instruction {
    North(i32),
    East(i32),
    West(i32),
    South(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn rotate(&self, ang: i32) -> Self {
        assert!(ang % 90 == 0);
        let n = ang.abs() / 90;
        let mut new_dir = *self;
        for _ in 0..n {
            new_dir = if ang > 0 {
                // Left rotate.
                match new_dir {
                    Direction::North => Direction::West,
                    Direction::West  => Direction::South,
                    Direction::South => Direction::East,
                    Direction::East  => Direction::North,
                }
            } else {
                match new_dir {
                    Direction::West  => Direction::North,
                    Direction::South => Direction::West,
                    Direction::East  => Direction::South,
                    Direction::North => Direction::East,
                }
            };
        }
        new_dir
    }
}

fn main() {
    let (_, dx, dy) = io::stdin().lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let c = line.chars().nth(0).unwrap();
            let num = line[1..].parse().unwrap();
            match c {
                'N' => North(num),
                'S' => South(num),
                'E' => East(num),
                'W' => West(num),
                'L' => Left(num),
                'R' => Right(num),
                'F' => Forward(num),
                _   => unreachable!(),
            }
        }).fold((Direction::East, 0, 0), |(dir, dx, dy), instruction| {
            match instruction {
                North(num)   => (dir, dx, dy + num),
                South(num)   => (dir, dx, dy - num),
                East(num)    => (dir, dx + num, dy),
                West(num)    => (dir, dx - num, dy),
                Left(num)    => (dir.rotate(num), dx, dy),
                Right(num)   => (dir.rotate(-num), dx, dy),
                Forward(num) => match dir {
                    Direction::North => (dir, dx, dy + num),
                    Direction::South => (dir, dx, dy - num),
                    Direction::East  => (dir, dx + num, dy),
                    Direction::West  => (dir, dx - num, dy),
                }
            }
        });

    println!("The final manhattan distance is {}", dx.abs() + dy.abs());
}

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

fn sin(ang: i32) -> i32 {
    let x = (4 + (ang / 90) % 4) % 4;
    match x {
        0 | 2 =>  0,
        1 =>  1,
        3 => -1,
        _ => unreachable!(),
    }
}

fn cos(ang: i32) -> i32 {
    let x = (4 + (ang / 90) % 4) % 4;
    match x {
        0 =>  1,
        2 => -1,
        1 | 3 => 0,
        _ => unreachable!(),
    }
}

fn main() {
    let (x, y, _, _) = io::stdin().lock()
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
        }).fold((0, 0, 10, 1), |(x, y, dx, dy), instruction| {
            match instruction {
                North(num)   => (x, y, dx, dy + num),
                South(num)   => (x, y, dx, dy - num),
                East(num)    => (x, y, dx + num, dy),
                West(num)    => (x, y, dx - num, dy),
                Left(num)    => ( x
                                , y
                                , cos(num) * dx - sin(num) * dy
                                , cos(num) * dy + sin(num) * dx
                                ),

                Right(num)   => ( x
                                , y
                                , cos(-num) * dx - sin(-num) * dy
                                , cos(-num) * dy + sin(-num) * dx
                                ),

                Forward(num) => (x + num * dx, y + num * dy, dx, dy),
            }
        });

    println!("Final point is ({}, {})", x, y);
    println!("The final manhattan distance is {}", x.abs() + y.abs());
}

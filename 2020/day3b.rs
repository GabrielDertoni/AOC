use std::io;
use std::io::Read;

fn main() {
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Err(_) => unreachable!(),
        _      => (),
    }

    let mut mult = 1;
    for (dx, dy) in slopes.iter() {
        let mut count = 0;
        let mut pos_x = 0;
        for line in input.lines().step_by(*dy as usize) {
            if line.chars().nth(pos_x).unwrap() == '#' {
                count += 1;
            }
            
            pos_x = (pos_x + dx) % line.len();
        }
        mult *= count;
        println!("with dy/dx = {}/{}, # of trees in the way is {}", dy, dx, count);
    }
    println!("Final value is {}", mult);
}

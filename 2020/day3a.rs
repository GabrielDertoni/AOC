use std::io;
use std::io::Read;

fn main() {
    let dx = 3;
    let dy = 1;

    let mut input = String::new();
    match io::stdin().read_to_string(&mut input) {
        Err(_) => unreachable!(),
        _      => (),
    }

    let mut count = 0;
    let mut pos_x = 0;
    for line in input.lines().step_by(dy) {
        if line.chars().nth(pos_x).unwrap() == '#' {
            count += 1;
        }
        
        pos_x = (pos_x + dx) % line.len();
    }

    println!("# of trees in the way is {}", count);
}

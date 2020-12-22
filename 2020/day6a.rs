use std::io;
use std::io::prelude::*;

fn main() {
    let group_iter = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .fold(vec![[false; 26]], |mut groups: Vec<[bool; 26]>, answers| {
            if answers.len() > 0 {
                let len = groups.len();
                let group = &mut groups[len - 1];
                answers.chars().for_each(move |ans| group[ans as usize - 97] = true);
            } else {
                groups.push([false; 26]);
            }

            groups
        });

    let mut sum = 0;
    for group in group_iter {
        let count = group.iter().filter(|&el| *el).count();
        sum += count;
    }
    println!("Final sum is: {}", sum);
}

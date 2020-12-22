use std::io;
use std::io::prelude::*;

struct Group {
    nmemb: u32,
    answered: [u32; 26],
}

impl Group {
    fn empty() -> Self {
        Group {
            nmemb: 0,
            answered: [0; 26],
        }
    }
}

fn main() {
    let groups = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .fold(vec![Group::empty()], |mut groups: Vec<Group>, answers| {
            if answers.len() > 0 {
                let len = groups.len();
                let group = &mut groups[len - 1];
                group.nmemb += 1;
                answers.chars().for_each(move |ans| group.answered[ans as usize - 97] += 1);
            } else {
                groups.push(Group::empty());
            }

            groups
        });

    let mut sum = 0;
    for group in groups.iter() {
        let count = group.answered.iter().filter(|&el| *el == group.nmemb).count();
        sum += count;
    }
    println!("Final sum is: {}", sum);
}

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let target = 2020;

    let mut num_occur: HashMap<u32, u32> = HashMap::new();
    for line in io::stdin().lock().lines() {
        let value = line.unwrap().parse().unwrap();

        if let Some(current) = num_occur.get_mut(&value) {
            *current += 1;
        } else {
            num_occur.insert(value, 0);
        }
    }

    let mut found = false;
    for (a, ca) in num_occur.iter() {
        for (b, _) in num_occur.iter() {
            // If a == b but there is only one occurence of a, they must be the same.
            if (a == b && *ca == 1) || a + b > target { continue };

            if let Some(_) = num_occur.get(&(target - a - b)){
                let other = target - a - b;
                println!("Found match! {} + {} + {} = {}", a, b, other, target);
                println!("Result is: {} * {} * {} = {}", a, b, other, a * b * other);
                found = true;
                break;
            }
        }
        if found { break };
    }
}

use std::io;
use std::io::prelude::*;

fn main() {
    let preemble_size = 25;
    let list = io::stdin().lock()
        .lines()
        .map(Result::unwrap) 
        .map(|n| n.parse().unwrap())
        .collect::<Box<[i64]>>();

    if let Some(number) = list
        .windows(preemble_size + 1)
        .map(|set| set.split_last().unwrap())
        .fold(None, |num, (&n, preemble)| {
            if let Some(_) = num {
                return num;
            }

            // Finding a pair of numbers that sum to n. O(n^2)
            if preemble.iter().fold(false, |found, &a|
                found || preemble.iter().find(|&&el| el == n - a).is_some())
            {
                None
            } else {
                Some(n)
            }
        }) {
        // NOTE: Problems may occur if number == 0
        let mut sum = 0;
        let mut i = 0;
        let mut start = 0;
        while (i - start <= 1 || sum != number) && i < list.len() {
            sum += list[i];
            while start < i && sum > number {
                sum -= list[start];
                start += 1;
            }
            i += 1;
        }
        println!("The first number that does not fit the pattern is {}", number);
        if i == list.len() || i - start == 1 {
            println!("There are no contiguous slices of list larger than 1 that sum up to {}", number);
        } else {
            assert_eq!(list[start..i].iter().sum::<i64>(), number);
            let min = list[start..i].iter().min().unwrap();
            let max = list[start..i].iter().max().unwrap();
            println!("Min is {}, max is {} with min + max = {}", min, max, min + max);
        }
    } else {
        println!("All numbers fit the pattern");
    }
}

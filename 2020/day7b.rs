#![feature(str_split_once)]

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn parse_bags<'a>(line: &'a str) -> (&'a str, Vec<(String, u32)>) {
    let mut contains = Vec::new();

    let (mut bag, contains_bags) = line.split_once("contain").unwrap();
    bag = bag.split_once(" bags").unwrap().0;

    for contained_bag in contains_bags.split(", ") {
        let trim = contained_bag.trim();

        if trim.starts_with("no other bags") { continue };

        let bag_amount = trim
            .split(' ')
            .take(1)
            .nth(0)
            .unwrap()
            .parse()
            .unwrap();

        let bag_name = trim
            .split(' ')
            .skip(1)
            .take(2)
            .collect::<Vec<&str>>()
            .join(" ");

        contains.push((bag_name, bag_amount));
    }
    
    (bag, contains)
}

fn traverse(bag: &str, bags: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let mut count = 1;
    if let Some(contained) = bags.get(bag) {
        for (bag, amount) in contained.iter() {
            count += amount * traverse(bag, bags);
        }
    }
    return count;
}

fn main() {
    // bags[bag_name] is a vector of the bags that must be contained in bag_name.
    // Each element is a tuple: the contained bag name and its required ammount.
    let mut bags: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    for line in io::stdin().lock().lines().map(Result::unwrap) {
        let (bag, contains) = parse_bags(&line);
        bags.insert(String::from(bag), contains);
    }

    let count = traverse("shiny gold", &bags);

    // Dont count shiny gold bag.
    println!("# nested bags {}", count - 1);
}

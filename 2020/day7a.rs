#![feature(str_split_once)]

use std::io;
use std::io::prelude::*;
use std::collections::{ HashMap, HashSet };

fn parse_bags<'a>(line: &'a str) -> (&'a str, Vec<String>) {
    let mut contains = Vec::new();

    let (mut bag, contains_bags) = line.split_once("contain").unwrap();
    bag = bag.split_once(" bags").unwrap().0;

    for contained_bag in contains_bags.split(", ") {
        let bag_name = contained_bag
            .trim()
            .split(' ')
            .skip(1)
            .take(2)
            .collect::<Vec<&str>>()
            .join(" ");

        contains.push(bag_name);
    }
    
    (bag, contains)
}

fn traverse<'a>(bag: &'a str, bags: &'a HashMap<String, Vec<String>>, visited: HashSet<&'a str>) -> HashSet<&'a str> {
    let mut vis = visited;
    if let None = vis.get(bag) {
        vis.insert(bag);
        if let Some(containers) = bags.get(bag) {
            for container in containers.iter() {
                vis = traverse(container, bags, vis);
            }
        }
    }
    return vis;
}

fn main() {
    // bags[bag_name] is a vector of bags that must contain bag_name.
    let mut bags: HashMap<String, Vec<String>> = HashMap::new();
    for line in io::stdin().lock().lines().map(Result::unwrap) {
        let (bag, contains) = parse_bags(&line);

        for contained_bag in contains {
            match bags.get_mut(&contained_bag) {
                Some(vec) => { vec.push(String::from(bag)); },
                None      => { bags.insert(contained_bag, vec![String::from(bag)]); },
            }
        }
    }

    let mut visited: HashSet<&str> = HashSet::new();
    visited = traverse("shiny gold", &bags, visited);

    // Dont count the shiny gold bag.
    println!("# visited {}", visited.len() - 1);
}

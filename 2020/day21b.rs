/**
 * READ: more info. in day21a.rs
 *
 * This implements the same idea presented in day21a.rs, except its way more efficient.
 * In my machine its a 20x speedup.
 * Instead of building multiple hash sets and comparing every product with every other
 * product, this approach folds all products in a single hash map of allergens to
 * candidate ingredients. As it is expected that there are more ingredients than
 * allergens, we can get away with fewer entries in the hash map.
 *
 * For every product, if an allergen is not already in the hash map, it is added along
 * with all ingredients on that product. All these ingredients are cadidates for having
 * that allergen. If the allergen has already been inserted, then there must be at
 * least one ingredient in common between the current product and the candidates for
 * that allergen. Replace the candidates for the allergen with the common ingredients.
 *
 * After this hashmap is created, allergens may have still many ingredient candidates,
 * however it should be garanteed that at least one allergen will have only one
 * ingredient as candidate. That ingredient is called "solved" and is removed as
 * candidate from every other allergen entry. After that is done, there will be at
 * least one other solved ingredient and the process can be repeated.
 */

use std::io;
use std::io::prelude::*;
use std::collections::{ HashMap, HashSet };

type Ingredient = u32;
type Allergen = u32;

struct Product {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

fn parse_product( product: &str
                , ingredient_ids: &mut HashMap<String, Ingredient>
                , allergen_ids: &mut HashMap<String, Allergen>
                ) -> Product
{
    let mut words_iter = product.split(' ');
    let ingredients = words_iter.by_ref()
        .take_while(|word| !word.starts_with('('))
        .map(|word| {
            if let Some(id) = ingredient_ids.get(word) { *id }
            else {
                let len = ingredient_ids.len() as u32;
                ingredient_ids.insert(word.to_string(), len);
                len
            }
        })
        .collect();

    let separators: &[_] = &['(', ')', ',', ' '];
    let allergens = words_iter.by_ref()
        .map(|word| word.trim_matches(separators))
        .map(|word| {
            if let Some(id) = allergen_ids.get(word) { *id }
            else {
                let len = allergen_ids.len() as u32;
                allergen_ids.insert(word.to_string(), len);
                len
            }
        })
        .collect();

    Product { ingredients, allergens }
}

fn main() {
    let stdin = io::stdin();
    // Breaking the functional paradigm for a bit more efficiency
    let mut ingredient_name_to_id: HashMap<String, Ingredient> = HashMap::new();
    let mut allergen_name_to_id: HashMap<String, Allergen> = HashMap::new();
    let mut aller_to_ing: HashMap<Allergen, HashSet<Ingredient>> = stdin.lock().lines()
        .map(Result::unwrap)
        .map(|line| parse_product(line.as_str(), &mut ingredient_name_to_id, &mut allergen_name_to_id))
        .fold(HashMap::new(), |mut aller_to_ing, product| {
            let Product { ingredients, allergens } = product;
            for allergen in allergens {
                match aller_to_ing.get_mut(&allergen) {
                    Some(candidates) => {
                        // Calculates the intersection of both sets
                        let common = (&*candidates) & (&ingredients);
                        candidates.retain(|ing| common.contains(ing));
                    },
                    None             => {
                        // All ingredients in the product are viable candidates for this allergen.
                        aller_to_ing.insert(allergen, ingredients.clone());
                    },
                }
            }
            aller_to_ing
        });

    // A vector that represents the allergens that are not solved yet.
    let mut to_update: Vec<&mut HashSet<Ingredient>> = aller_to_ing.values_mut().collect();

    // Find a solved ingredient, remove it as candidate from the other allergens, repeat untill
    // all ingredients are solved.
    while to_update.len() > 0 {
        let i = to_update.iter()
            .position(|ingredients| ingredients.len() == 1)
            .unwrap();

        let solved = to_update.swap_remove(i);
        let solved_ing = solved.iter().nth(0).unwrap();
        // Remove solved from candidates.
        to_update.iter_mut()
            .for_each(|ingredients| {
                ingredients.remove(solved_ing);
            });
    }

    // Inverse hashmaps to get names from ids.
    let ingredient_id_to_name: HashMap<Ingredient, &str> = ingredient_name_to_id.iter()
        .map(|(name, id)| (*id, name.as_str()))
        .collect();

    let allergen_id_to_name: HashMap<Allergen, &str> = allergen_name_to_id.iter()
        .map(|(name, id)| (*id, name.as_str()))
        .collect();

    // Pairs (ingredient, allergen). Will only contain ingredients that do contain an allergen.
    let mut ingredient_allergen: Vec<(&str, &str)> = aller_to_ing.iter()
        .map(|(allergen, ingredients)| (allergen, ingredients.iter().nth(0).unwrap()))
        .map(|(allergen, ingredient)| (allergen_id_to_name[allergen], ingredient_id_to_name[ingredient]))
        .collect();

    ingredient_allergen.sort_unstable_by_key(|&(allergen, _)| allergen);

    let danger_list: String = ingredient_allergen.iter()
        .map(|(_, ingredient)| *ingredient)
        .collect::<Vec<&str>>()
        .join(",");

    println!("The DANGER list is: {}", danger_list);
}


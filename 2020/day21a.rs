/**
 *
 * Because we know that "each allergen is found in exactly one ingredient", if two
 * lists of ingredients have exactly one ingredient in common and one allergen in
 * common as well, it means that the ingredient that appears in both must have the
 * allergen found in both. For example, the list:
 *
 *   mxmxvkd sqjhc                 kfcds nhms (contains dairy, fish)
 *   mxmxvkd       fvjkl sbzzf trh            (contains dairy)
 *   ^^^^^^^                                            ^^^^^
 *   in common                                        in common
 *
 * implies that the ingredient "mxmxvkd" must contain diary.
 *
 * If there are two or more ingredients and allergens in common, we can't know which
 * of them have which allergen. However, we know that they must contain all of them
 * and therefore no other ingredient can contain those same allergens.
 * 
 *   mxmxvkd fvjkl sqjhc (contains diary, fish)
 *   mxmxvkd sbzzf sqjhc (contains diary, fish)
 *   ^^^^^^^       ^^^^^           ^^^^^  ^^^^
 *        in common                 in common
 *
 * we can't know if "mxmxvkd" contains diary and "sqjhc" has fish, or vice versa. But
 * in one case or the other, these ingredients must contain one of these allergens
 * and no other ingredient may contain them.
 *
 * If there is at least one allergen in common, but more ingredients in common, the
 * allergens in common can only be present in one of the common ingredients. As an
 * example take the pair of products
 *
 *   mxmxvkd kfcds sqjhc (contains fish, diary)
 *   mxmxvkd sbzzf sqjhc (contains fish)
 *   ^^^^^^^       ^^^^^           ^^^^
 *       in common               in common
 * 
 * both "mxmxvkd" can still contain fish or diary. However, one of them must contain
 * fish. Therefore no other ingredient can have fish.
 *
 * Also note that the number of matching allergens in any given pair of products must
 * be smaller or equal to the number of matching ingredients. For example, consider
 * the pair:
 *
 *   mxmxvkd fvjkl kfcds (contains diary, fish)
 *   mxmxvkd sbzzf sqjhc (contains diary, fish)
 *   ^^^^^^^                       ^^^^^  ^^^^
 *   in common                      in common
 *
 * if "mxmxvkd" contains diary, then "fvjkl" or "kfcds" contains fish. This implies
 * that "sbzzf" and "sqjhc" do not contain fish, but one of them must!
 *
 * In general, if there are common allergens between two products all other ingredients
 * that are not common between those two products can not contain those allergens.
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
    let products: Box<[Product]> = stdin.lock().lines()
        .map(Result::unwrap)
        .map(|line| parse_product(line.as_str(), &mut ingredient_name_to_id, &mut allergen_name_to_id))
        .collect();

    let mut ingredient_possible_allergens: HashMap<Ingredient, HashSet<Allergen>> = HashMap::new();
    for product in products.iter() {
        for ingredient in product.ingredients.iter() {
            match ingredient_possible_allergens.get_mut(ingredient) {
                Some(candidates) => 
                    product.allergens.iter()
                        .for_each(|allergen| { candidates.insert(*allergen); }),

                None             => {
                    ingredient_possible_allergens.insert(
                        *ingredient,
                        product.allergens.iter().map(|a| *a).collect()
                    );
                },
            }
        }
    }
    for product in products.iter() {
        for other in products.iter() {
            let common_ingredients: HashSet<Ingredient> = product.ingredients.intersection(&other.ingredients)
                .map(|ing| *ing)
                .collect();

            for common_allergen in product.allergens.intersection(&other.allergens) {
                // Remove from all ingredients that are not in common
                ingredient_possible_allergens.iter_mut()
                    .filter(|(ingredient, _)| !common_ingredients.contains(ingredient))
                    .for_each(|(_, candidates)| { candidates.remove(common_allergen); });
            }
        }
    }

    let safe_ingredients: HashSet<Ingredient> = ingredient_possible_allergens.iter()
        .filter(|(_, candidates)| candidates.len() == 0)
        .map(|(ingredient, _)| *ingredient)
        .collect();

    let n_occurr_safe_ingredients: usize = products.iter()
        .flat_map(|product| product.ingredients.iter())
        .filter(|ingredient| safe_ingredients.contains(ingredient))
        .count();

    println!("The number of occurrence of safe ingredients is {}", n_occurr_safe_ingredients);

    /*
    let ingredient_id_to_name: HashMap<Ingredient, &str> = ingredient_name_to_id.iter()
        .map(|(name, id)| (*id, name.as_str()))
        .collect();

    let allergen_id_to_name: HashMap<Allergen, &str> = allergen_name_to_id.iter()
        .map(|(name, id)| (*id, name.as_str()))
        .collect();

    for (i, a) in ingredient_possible_allergens.iter() {
        let name = ingredient_id_to_name[i];
        let poss: Vec<&str> = a.iter().map(|al| allergen_id_to_name[al]).collect();
        println!("Ingredient {} has possible allergens {:?}", name, poss);
    }
    */
}


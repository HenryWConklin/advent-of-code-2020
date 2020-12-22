use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

struct IngredientList {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn main() {
    let foods: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let split_ind = l.find('(').unwrap();

            let mut ingredients = HashSet::new();
            ingredients.extend(l[..split_ind - 1].split(' ').map(|s| s.to_string()));

            let mut allergens = HashSet::new();
            allergens.extend(
                l[split_ind + 2 + "contains".len()..l.len() - 1]
                    .split(", ")
                    .map(|s| s.to_string()),
            );
            IngredientList {
                ingredients,
                allergens,
            }
        })
        .collect();

    let mut all_ingredients = HashSet::new();
    all_ingredients.extend(foods.iter().flat_map(|f| f.ingredients.iter()));

    let mut all_allergens = HashSet::new();
    all_allergens.extend(foods.iter().flat_map(|f| f.allergens.iter()));

    let mut allergen_candidates = HashMap::new();
    allergen_candidates.extend(all_allergens.iter().map(|&a| (a, all_ingredients.clone())));

    // If a food contains an allergen, it must be present every time that allergen is listed
    // Eliminate any food that is not present any time a given allergen is listed
    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            let candidates = allergen_candidates.get_mut(allergen).unwrap();
            candidates.retain(|&f| food.ingredients.contains(f));
        }
    }

    // If any of the allergens has one candidate, it must be that ingredient and we can eliminate
    // it from the other allergens' candidates
    for _ in 0..allergen_candidates.len() {
        let mut single_candidates: HashSet<&String> = HashSet::new();
        for (_, candidates) in allergen_candidates.iter() {
            if candidates.len() == 1 {
                single_candidates.extend(candidates.iter())
            }
        }
        for (_, candidates) in allergen_candidates.iter_mut() {
            if candidates.len() != 1 {
                candidates.retain(|&i| !single_candidates.contains(i));
            }
        }
    }

    println!("{:?}", allergen_candidates);
    let mut safe_ingredients = all_ingredients.clone();
    for ingredients in allergen_candidates.values() {
        safe_ingredients.retain(|&f| !ingredients.contains(f));
    }

    let part1: usize = foods
        .iter()
        .map(|f| {
            f.ingredients
                .iter()
                .filter(|&i| safe_ingredients.contains(i))
                .count()
        })
        .sum();
    println!("{}", part1);

    let mut part2: Vec<_> = allergen_candidates.iter().collect();
    part2.sort_by(|x, y| x.0.cmp(y.0));
    let part2 = part2
        .iter()
        .map(|(_, allergen_set)| allergen_set.iter().next().unwrap())
        .join(",");
    println!("{}", part2);
}

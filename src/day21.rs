use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Ingredient = String;
type Allergen = String;
type Food = (Vec<Ingredient>, Vec<Allergen>);

#[aoc_generator(day21)]
fn aoc_generator(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(" (contains ");
            let ingredients = parts.next().unwrap().split(" ").map(|b| String::from(b));
            let allergens = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|b| String::from(b).replace(")", ""));

            (
                ingredients.collect::<Vec<Ingredient>>(),
                allergens.collect::<Vec<Allergen>>(),
            )
        })
        .collect::<Vec<Food>>()
}

#[aoc(day21, part1)]
fn part1(input: &[Food]) -> usize {
    solve(input)
        .iter()
        .filter(|(_, a)| a == "")
        .fold(0, |old, (ing, _)| {
            old + input.iter().filter(|(ings, _)| ings.contains(ing)).count()
        })
}

#[aoc(day21, part2)]
fn part2(input: &[Food]) -> String {
    let vec = solve(input);
    let mut ing = vec
        .iter()
        .filter(|(_, a)| a != &"")
        .map(|(i, _)| i.as_str())
        .collect::<Vec<&str>>();
    ing.sort_by(|a, b| {
        vec.iter()
            .filter(|(ing, _)| ing == a)
            .map(|(_, all)| all)
            .next()
            .unwrap()
            .partial_cmp(
                vec.iter()
                    .filter(|(ing, _)| ing == b)
                    .map(|(_, all)| all)
                    .next()
                    .unwrap(),
            )
            .unwrap()
    });
    ing.join(",")
}

fn solve(input: &[Food]) -> Vec<(Ingredient, Allergen)> {
    let mut possibilities: HashMap<Allergen, Vec<Ingredient>> = HashMap::new();

    for (ings, allergens) in input {
        for all in allergens {
            if possibilities.contains_key(all) {
                let ing = (*possibilities.get(all).unwrap()).clone();
                possibilities.insert(
                    all.to_string(),
                    ings.iter()
                        .filter(|i| ing.contains(i))
                        .map(|b| b.clone())
                        .collect::<Vec<Ingredient>>(),
                );
            } else {
                possibilities.insert(all.to_string(), ings.clone());
            }
        }
    }

    loop {
        let fixed = possibilities
            .iter()
            .filter(|(_, ings)| ings.len() == 1)
            .map(|(_, ings)| ings[0].clone())
            .collect::<Vec<Ingredient>>();
        let unknown = possibilities
            .iter()
            .filter(|(_, ings)| ings.len() > 1)
            .map(|(a, ings)| (a.clone(), ings.clone()))
            .collect::<Vec<(Allergen, Vec<Ingredient>)>>();

        if unknown.len() == 0 {
            break;
        }

        for u in unknown {
            if !u.1.iter().any(|ing| fixed.contains(ing)) {
                continue;
            }
            let mut new = u.1.clone();
            for newly_fixed in u.1.iter().filter(|ing| fixed.contains(ing)) {
                new.remove(new.iter().position(|ing| ing == newly_fixed).unwrap());
            }

            possibilities.insert(u.0.clone(), new);
        }
    }

    let mut all_ingres = input.iter().fold(vec![], |mut old, food| {
        old.append(&mut food.0.clone());
        old
    });
    all_ingres.sort();
    all_ingres.dedup();

    let b = all_ingres
        .iter()
        .map(|i| {
            (
                i.clone(),
                possibilities
                    .iter()
                    .filter(|(_, ings)| ings.contains(i))
                    .map(|(a, _)| a)
                    .next()
                    .unwrap_or(&String::new())
                    .clone(),
            )
        })
        .collect::<Vec<(Ingredient, Allergen)>>();

    b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(part1(&aoc_generator(input)), 5);
    }

    #[test]
    fn test_part2() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(
            part2(&aoc_generator(input)),
            String::from("mxmxvkd,sqjhc,fvjkl")
        );
    }
}

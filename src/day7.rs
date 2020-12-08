use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

const MY_BAG: &str = "shiny gold";

#[derive(Clone)]
struct Bag {
    color: String,
    children: HashMap<String, usize>,
    parents: Vec<String>,
}

#[aoc_generator(day7)]
fn generator_parents(input: &str) -> HashMap<String, HashMap<String, usize>> {
    let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();

    input.lines().for_each(|l| {
        let mut part = l.split(" contain ");
        let color = part.next().unwrap();
        let color = String::from(&color[..color.len() - " bags".len()]);

        if !bags.contains_key(&color) {
            bags.insert(color.clone(), HashMap::new());
        }

        part.next().unwrap().split(", ").for_each(|parent_desc| {
            let parent_desc = parent_desc.split(" bag").next().unwrap(); // removes bag or bags.
            let num_str = parent_desc.split(" ").next().unwrap();
            let num = num_str.parse().unwrap_or(0);
            let c_color = String::from(&parent_desc[num_str.len() + 1..]);

            if num == 0 {
                //println!{"Bag {} contains no other", color};
                return;
            }

            //println!{"Bag {} contains {} x {}", color, num, c_color};
            bags.get_mut(&color).unwrap().insert(c_color, num);
        });
    });

    bags
}

fn get_doubled_ancestors(bag: &str, bags: &HashMap<String, HashMap<String, usize>>) -> Vec<String> {
    bags.iter()
        .filter(|(_, h)| h.contains_key(bag))
        .fold(vec![], |mut vec, (c, _)| {
            vec.push(c.clone());
            vec.append(&mut get_doubled_ancestors(c, bags));

            vec
        })
}

#[aoc(day7, part1)]
fn part1(bags: &HashMap<String, HashMap<String, usize>>) -> usize {
    get_doubled_ancestors(MY_BAG, bags).iter().unique().count()
}

// returns bags inside +1 (cuz the initial bag is counted as well)
fn get_bags_inside(bag: &str, bags: &HashMap<String, HashMap<String, usize>>) -> usize {
    bags.get(bag)
        .unwrap()
        .iter()
        .fold(1, |old, (c, a)| old + a * get_bags_inside(c, bags))
}

#[aoc(day7, part2)]
fn part2(bags: &HashMap<String, HashMap<String, usize>>) -> usize {
    get_bags_inside(MY_BAG, bags) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";

        let bags = &generator_parents(input);

        println!("{}", bags.len());

        assert_eq!(part1(bags), 4);
    }

    #[test]
    fn test_part2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        assert_eq!(part2(&generator_parents(input)), 32);
        let input = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";
        assert_eq!(part2(&generator_parents(input)), 126);
    }
}

use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Range;

type Ticket = Vec<usize>;
type Rule = (String, Vec<Range<usize>>);

#[aoc_generator(day16)]
fn generator(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    input
        .split("\n\n")
        .fold((vec![], vec![], vec![]), |mut old, block| {
            let mut lines = block.lines();

            match lines.next().unwrap() {
                "your ticket:" => {
                    old.1 = lines
                        .next()
                        .unwrap()
                        .split(",")
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect()
                }
                "nearby tickets:" => {
                    old.2 = lines
                        .map(|l| l.split(",").map(|c| c.parse::<usize>().unwrap()).collect())
                        .collect()
                }
                _ => {
                    old.0 = block.lines().fold(old.0, |mut o, l| {
                        let mut split = l.split(": ");
                        let rule = (
                            String::from(split.next().unwrap()),
                            split
                                .next()
                                .unwrap()
                                .split(" or ")
                                .fold(vec![], |mut oo, r| {
                                    let s = r
                                        .split("-")
                                        .map(|r| r.parse::<usize>().unwrap())
                                        .collect::<Vec<usize>>();
                                    oo.push(s[0]..s[1] + 1); // inclusive
                                    oo
                                }),
                        );
                        o.push(rule);
                        o
                    })
                }
            }

            old
        })
}

#[aoc(day16, part1)]
fn part1((rules, _, nearby): &(Vec<Rule>, Ticket, Vec<Ticket>)) -> usize {
    let rules_ranges = rules.iter().fold(vec![], |mut old, new| {
        old.append(&mut new.1.clone());
        old
    });
    let mut b = 0;
    for ticket in nearby {
        // can't get iter to work...
        for val in ticket {
            if rules_ranges.iter().filter(|r| r.contains(val)).count() == 0 {
                b += val;
            }
        }
    }
    b
}

fn contains_invalid(ticket: &Ticket, rules: &Vec<Rule>) -> bool {
    ticket.iter().any(|num| {
        rules
            .iter()
            .all(|rule| rule.1.iter().all(|range| !range.contains(num)))
    })
}

#[aoc(day16, part2)]
fn part2((rules, my_ticket, nearby): &(Vec<Rule>, Ticket, Vec<Ticket>)) -> usize {
    let nearby = nearby
        .iter()
        .filter(|t| !contains_invalid(t, rules))
        .map(|t| t.clone())
        .collect::<Vec<Ticket>>(); // valid tickets

    let mut possible_fields: Vec<Vec<Rule>> = vec![vec![]; my_ticket.len()];

    for ticket in nearby {
        for (i, val) in ticket.iter().enumerate() {
            let mut matching_rules = rules
                .iter()
                .filter(|r| r.1.iter().any(|range| range.contains(val)))
                .map(|r| r.clone())
                .collect::<Vec<Rule>>();

            /*println!(
                "matching {}! {:?} {:?}",
                i,
                matching_rules,
                possible_fields.get(i).unwrap()
            );*/
            if possible_fields.get(i).unwrap().len() != 0 {
                matching_rules = matching_rules
                    .iter()
                    .filter(|r| {
                        possible_fields
                            .get(i)
                            .unwrap()
                            .iter()
                            .map(|r| r.0.clone())
                            .collect::<Vec<String>>()
                            .contains(&r.0)
                    })
                    .map(|r| r.clone())
                    .collect::<Vec<Rule>>();
                //println!("Common: {:?} ", matching_rules);
            }

            possible_fields[i] = matching_rules;
        }
    }

    let mut possible_fields = possible_fields.clone();
    //println!("Solve: {:?}", possible_fields);
    // possible_fields = Vec<Vec<Rule>>
    loop {
        if possible_fields.iter().all(|v| v.len() == 1) {
            break;
        }

        let determined =
            possible_fields
                .iter()
                .filter(|v| v.len() == 1)
                .fold(vec![], |mut o, r| {
                    o.push(r[0].clone());
                    o
                });

        //println!("determined {:?}", determined);

        for (i, field) in possible_fields.clone().iter().enumerate() {
            if field.len() == 1 {
                continue;
            }
            // remove elements in determined from field
            possible_fields[i] = field
                .iter()
                .filter(|r| !determined.contains(r))
                .map(|r| r.clone())
                .collect::<Vec<Rule>>();
        }
    }

    let mut my_ticket_data = vec![];

    for i in 0..my_ticket.len() {
        my_ticket_data.push((possible_fields[i][0].0.clone(), my_ticket[i]));
    }

    my_ticket_data
        .iter()
        .filter(|t| t.0.starts_with("departure"))
        .map(|t| t.1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";
        assert_eq!(part1(&generator(input)), 71);
    }

    #[test]
    fn test_part2() {
        let input = "departure_class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9"; // slighty modified
        assert_eq!(part2(&generator(input)), 12);
    }
}

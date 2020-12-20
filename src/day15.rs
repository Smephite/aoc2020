use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut round = 1;
    let mut last_spoken = 0;

    for num in input.split(",") {
        last_spoken = num.parse::<usize>().unwrap();
        map.insert(last_spoken, round);
        //println!(":{} {}", round, last_spoken);
        round += 1;
    }

    map.remove(&input.split(",").last().unwrap().parse::<usize>().unwrap());

    loop {
        let last_spoken_round = match map.contains_key(&last_spoken) {
            true => *map.get(&last_spoken).unwrap(),
            false => 0,
        };
        map.insert(last_spoken, round - 1);
        //println!("insert {} {}", round-1, last_spoken);

        if last_spoken_round == 0 {
            // never
            last_spoken = 0;
        } else {
            //println!("{} {}", round, last_spoken_round);
            last_spoken = round - last_spoken_round - 1;
        }

        //println!(":{} {}", round, last_spoken);

        if round == 2020 {
            // 2020
            break;
        }
        round += 1;
    }

    last_spoken
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut round = 1;
    let mut last_spoken = 0;

    for num in input.split(",") {
        last_spoken = num.parse::<usize>().unwrap();
        map.insert(last_spoken, round);
        //println!(":{} {}", round, last_spoken);
        round += 1;
    }

    map.remove(&input.split(",").last().unwrap().parse::<usize>().unwrap());

    loop {
        let last_spoken_round = match map.contains_key(&last_spoken) {
            true => *map.get(&last_spoken).unwrap(),
            false => 0,
        };
        map.insert(last_spoken, round - 1);
        //println!("insert {} {}", round-1, last_spoken);

        if last_spoken_round == 0 {
            // never
            last_spoken = 0;
        } else {
            //println!("{} {}", round, last_spoken_round);
            last_spoken = round - last_spoken_round - 1;
        }

        //println!(":{} {}", round, last_spoken);

        if round == 30000000 {
            // 2020
            break;
        }
        round += 1;
    }

    last_spoken
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0,3,6";
        assert_eq!(part1(input), 436);
    }

    //#[test]
    fn test_part2() {
        assert_eq!(part2("0,3,6"), 175594);
        assert_eq!(part2("1,3,2"), 2578);
        assert_eq!(part2("2,1,3"), 3544142);
        assert_eq!(part2("1,2,3"), 261214);
        assert_eq!(part2("2,3,1"), 6895259);
        assert_eq!(part2("3,2,1"), 18);
        assert_eq!(part2("3,1,2"), 362);
    }
}

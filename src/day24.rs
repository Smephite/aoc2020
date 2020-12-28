use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day24)]
fn aoc_generator(input: &str) -> HashMap<(isize, isize), bool> {
    let mut map = HashMap::new();

    for dir in input.lines() {
        toggle(&transform(dir), &mut map);
    }

    map
}

fn transform(dir: &str) -> (isize, isize) {
    let mut chars = dir.chars();
    let mut pos = (0, 0);
    loop {
        let c = chars.next();
        if c == None {
            break;
        }
        match c.unwrap() {
            'w' => pos.0 -= 1,
            'e' => pos.0 += 1,
            'n' => {
                let c2 = chars.next().unwrap();
                pos.1 += 1;
                match c2 {
                    'w' => pos.0 -= 1,
                    _ => {}
                }
            }
            's' => {
                let c2 = chars.next().unwrap();
                pos.1 -= 1;
                match c2 {
                    'e' => pos.0 += 1,
                    _ => {}
                }
            }
            _ => unimplemented!(),
        };
    }
    pos
}

fn toggle(coord: &(isize, isize), map: &mut HashMap<(isize, isize), bool>) {
    map.insert(*coord, !*map.get(coord).unwrap_or(&false));
}

#[aoc(day24, part1)]
fn part1(input: &HashMap<(isize, isize), bool>) -> usize {
    input.iter().filter(|&(_, b)| *b).count()
}
#[aoc(day24, part2)]
fn part2(input: &HashMap<(isize, isize), bool>) -> usize {
    let input = &mut input.clone();
    for _ in 0..100 {
        iterate_p2(input);
    }

    input.iter().filter(|&(_, b)| *b).count()
}

fn iterate_p2(input: &mut HashMap<(isize, isize), bool>) {
    // false -- white
    let org = input.clone();

    let mut to_update: Vec<(isize, isize)> = vec![];

    for active in org.iter().filter(|k| *k.1).map(|k| *k.0) {
        let active_adjc = adjacent(active)
            .iter()
            .filter(|p| *org.get(p).unwrap_or(&false))
            .count();
        if active_adjc == 0 || active_adjc > 2 {
            input.insert(active, false);
        }
        to_update.append(&mut adjacent(active));
    }

    to_update.sort();
    to_update.dedup();
    for white in to_update.iter().filter(|p| !org.get(p).unwrap_or(&false)) {
        if adjacent(*white)
            .iter()
            .filter(|p| *org.get(p).unwrap_or(&false))
            .count()
            == 2
        {
            input.insert(*white, true);
        }
    }
}

fn adjacent(coord: (isize, isize)) -> Vec<(isize, isize)> {
    let mut b = vec![];
    for x in -1..2 {
        for y in -1..2 {
            if (x == 0 && y == 0) || x == y {
                continue;
            }
            b.push((coord.0 + x, coord.1 + y));
        }
    }

    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(transform("nwwswee"), (0, 0));
        assert_eq!(transform("esew"), (1, -1));
        assert_eq!(transform("esenee"), (3, 0));
        let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew";
        assert_eq!(part1(&aoc_generator(input)), 10);
    }

    #[test]
    fn test_part2() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew";
        assert_eq!(part2(&aoc_generator(input)), 2208);
    }
}

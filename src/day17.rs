use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Location = (isize, isize, isize, isize);
type World = HashMap<Location, bool>;

#[aoc_generator(day17)]
fn generator(input: &str) -> World {
    let mut world = World::default();
    // x, y, z
    let mut loc: Location = (0, 0, 0, 0);

    for line in input.lines() {
        for c in line.chars() {
            if c == '#' {
                world.insert(loc.clone(), true);
            }
            loc.0 += 1;
        }

        loc.0 = 0;
        loc.1 += 1;
    }

    world
}

fn get_neighbours(loc: &Location, dim3: bool) -> Vec<Location> {
    let mut neighbours = vec![];
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                if dim3 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    neighbours.push((loc.0 + x, loc.1 + y, loc.2 + z, 0));
                } else {
                    for w in -1..2 {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }
                        neighbours.push((loc.0 + x, loc.1 + y, loc.2 + z, loc.3 + w));
                    }
                }
            }
        }
    }
    neighbours
}

fn get_active(world: &World, loc: &Location, dim3: bool) -> usize {
    get_neighbours(loc, dim3).iter().fold(0, |old, loc| {
        old + match world.get(loc).unwrap_or(&false) {
            true => 1,
            false => 0,
        }
    })
}

fn print_plane(world: &World, z: isize) {
    let min_x = world.keys().map(|pos| pos.0).min().unwrap();
    let min_y = world.keys().map(|pos| pos.0).min().unwrap();
    let max_x = world.keys().map(|pos| pos.1).max().unwrap();
    let max_y = world.keys().map(|pos| pos.1).max().unwrap();

    println!("z={}", z);
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            print!(
                "{}",
                match world.get(&(x, y, z, 0)).unwrap_or(&false) {
                    true => '#',
                    false => '.',
                }
            );
        }
        print!("\n");
    }
    print!("\n");
}

fn print_world3d(world: &World) {
    let min_z = world.keys().map(|loc| loc.2).min().unwrap();
    let max_z = world.keys().map(|loc| loc.2).max().unwrap();
    for z in min_z..max_z + 1 {
        print_plane(world, z)
    }
}

fn update(world_old: &World, dim3: bool) -> World {
    let mut world_new: HashMap<Location, bool> = HashMap::default();
    let mut to_update: Vec<Location> = vec![];
    for position in world_old
        .iter()
        .filter(|&(_, value)| *value)
        .map(|(key, _)| key)
    {
        let active = get_active(world_old, position, dim3);
        if active == 2 || active == 3 {
            world_new.insert(*position, true);
        } else {
            world_new.insert(*position, false);
        }
        to_update.append(
            &mut get_neighbours(position, dim3)
                .iter()
                .filter(|loc| !to_update.contains(loc) && !world_old.get(loc).unwrap_or(&false))
                .map(|loc| loc.to_owned())
                .collect::<Vec<Location>>(),
        )
    }

    let first_world_new = world_new.clone();

    for position in to_update
        .iter()
        .filter(|loc| !first_world_new.contains_key(loc) && !*world_old.get(loc).unwrap_or(&false))
    {
        let active = get_active(world_old, position, dim3);
        if active == 3 {
            world_new.insert(*position, true);
        } else {
            world_new.insert(*position, false);
        }
    }

    world_new
}

#[aoc(day17, part1)]
fn part1(world: &World) -> usize {
    let mut world = world.clone();

    for i in 0..6 {
        println!("After {} cycles: ", i);
        print_world3d(&world);
        world = update(&world, true);
    }

    world.iter().filter(|&(_, state)| *state).count()
}

#[aoc(day17, part2)]
fn part2(world: &World) -> usize {
    let mut world = world.clone();

    for i in 0..6 {
        world = update(&world, false);
    }

    world.iter().filter(|&(_, state)| *state).count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = ".#.\n..#\n###";

        assert_eq!(part1(&generator(input)), 112);
    }

    // #[test] slow
    fn test_part2() {
        let input = ".#.\n..#\n###";

        assert_eq!(part2(&generator(input)), 848);
    }
}

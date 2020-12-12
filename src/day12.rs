use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, PartialEq, Copy, Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

impl Direction {
    fn parse(input: &char) -> Direction {
        match *input {
            'N' => Self::NORTH,
            'S' => Self::SOUTH,
            'E' => Self::EAST,
            'W' => Self::WEST,
            'L' => Self::LEFT,
            'R' => Self::RIGHT,
            'F' => Self::FORWARD,
            _ => unimplemented!(),
        }
    }
}

type Instruction = (Direction, usize);

#[aoc_generator(day12)]
fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            (
                Direction::parse(&chars.next().unwrap()),
                chars.as_str().parse().unwrap(),
            )
        })
        .collect::<Vec<Instruction>>()
}

#[aoc(day12, part1)]
fn part1(input: &[Instruction]) -> usize {
    let target = input.iter().fold(
        (0, 0, Direction::EAST),
        |ship, instruction| match instruction.0 {
            Direction::LEFT => (ship.0, ship.1, rotate(&ship.2, instruction.1 as isize)),
            Direction::RIGHT => (ship.0, ship.1, rotate(&ship.2, -(instruction.1 as isize))),
            Direction::FORWARD => do_move(&ship, ship.2, instruction.1),
            _ => do_move(&ship, instruction.0, instruction.1),
        },
    );
    (target.0.abs() + target.1.abs()) as usize
}

fn rotate(old: &Direction, add: isize) -> Direction {
    let steps = add / 2;
    let dirs = [
        Direction::NORTH,
        Direction::WEST,
        Direction::SOUTH,
        Direction::EAST,
    ];

    let cur_index = dirs.iter().position(|r| r == old).unwrap();

    let mut new_index = cur_index as isize + steps;
    while new_index < 0 {
        new_index += 4;
    }

    while new_index as usize >= dirs.len() {
        new_index -= 4;
    }

    dirs[new_index as usize]
}

fn do_move(
    ship: &(isize, isize, Direction),
    dir: Direction,
    distance: usize,
) -> (isize, isize, Direction) {
    match dir {
        Direction::NORTH => (ship.0, ship.1 + distance as isize, ship.2),
        Direction::SOUTH => (ship.0, ship.1 - distance as isize, ship.2),
        Direction::EAST => (ship.0 + distance as isize, ship.1, ship.2),
        Direction::WEST => (ship.0 - distance as isize, ship.1, ship.2),
        _ => unreachable!(),
    }
}

fn do_stuff(
    (waypoint_x, waypoint_y, ship_x, ship_y): (isize, isize, isize, isize),
    direction: Direction,
    amount: isize,
) -> (isize, isize, isize, isize) {
    match direction {
        Direction::NORTH => (waypoint_x, waypoint_y + amount, ship_x, ship_y),
        Direction::SOUTH => (waypoint_x, waypoint_y - amount, ship_x, ship_y),
        Direction::WEST => (waypoint_x - amount, waypoint_y, ship_x, ship_y),
        Direction::EAST => (waypoint_x + amount, waypoint_y, ship_x, ship_y),
        Direction::LEFT => (-waypoint_y, waypoint_x, ship_x, ship_y),
        Direction::RIGHT => (waypoint_y, -waypoint_x, ship_x, ship_y),
        _ => (
            waypoint_x,
            waypoint_y,
            ship_x + waypoint_x * amount,
            ship_y + waypoint_y * amount,
        ), // forward
    }
}

#[aoc(day12, part2)]
fn part2(input: &[Instruction]) -> usize {
    let target = input.iter().fold(
        (10isize, 1isize, 0isize, 0isize),
        |position, &(direction, amount)| {
            let steps = match direction {
                Direction::LEFT | Direction::RIGHT => amount / 2,
                _ => 1,
            };

            let mut position = position;

            for _ in 0..steps {
                position = do_stuff(
                    position,
                    direction,
                    match direction {
                        Direction::LEFT | Direction::RIGHT => 1,
                        _ => (amount as isize),
                    },
                );
            }

            position
        },
    );
    (target.2.abs() + target.3.abs()) as usize
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_rotate() {
        let mut dir = Direction::NORTH;
        dir = rotate(&dir, 90);
        assert_eq!(dir, Direction::WEST);
        dir = rotate(&dir, 90);
        assert_eq!(dir, Direction::SOUTH);
        dir = rotate(&dir, 90);
        assert_eq!(dir, Direction::EAST);
        dir = rotate(&dir, 90);
        assert_eq!(dir, Direction::NORTH);
        dir = rotate(&dir, -90);
        assert_eq!(dir, Direction::EAST);
        dir = rotate(&dir, -90);
        assert_eq!(dir, Direction::SOUTH);
        dir = rotate(&dir, -90);
        assert_eq!(dir, Direction::WEST);
        dir = rotate(&dir, -90);
        assert_eq!(dir, Direction::NORTH);
        dir = rotate(&dir, 180);
        assert_eq!(dir, Direction::SOUTH);
        dir = rotate(&dir, 180);
        assert_eq!(dir, Direction::NORTH);
    }

    #[test]
    fn test_part1() {
        let input = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(part1(&generator(input)), 25);
    }

    #[test]
    fn test_part2() {
        let input = "F10\nN3\nF7\nR90\nF11";
        assert_eq!(part2(&generator(input)), 286);
    }
}

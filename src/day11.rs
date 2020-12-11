use aoc_runner_derive::{aoc, aoc_generator};
type WaitingArea = Vec<Vec<char>>;

#[aoc_generator(day11)]
fn generate(input: &str) -> WaitingArea {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn iterate_area(area: &mut WaitingArea, part: usize) {
    let options = match part {
        1 => (1, 4),
        2 => (-1, 5),
        _ => unimplemented!(),
    };

    let previous = &area.clone();
    for y in 0..previous.len() {
        for x in 0..previous.get(0).unwrap().len() {
            area[y][x] = match area[y][x] {
                'L' => match adjacent(previous, x, y, options.0) {
                    0 => '#',
                    _ => 'L',
                },
                '#' => match adjacent(previous, x, y, options.0) >= options.1 {
                    true => 'L',
                    _ => '#',
                },
                _ => area[y][x],
            }
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    DownLeft,
    UpRight,
    DownRight,
}

fn adjacent(area: &WaitingArea, x: usize, y: usize, max_steps: isize) -> usize {
    let x = x as isize;
    let y = y as isize;

    find_direction(area, (x, y), &Direction::Up, false, max_steps) as usize
        + find_direction(area, (x, y), &Direction::Down, false, max_steps) as usize
        + find_direction(area, (x, y), &Direction::Left, false, max_steps) as usize
        + find_direction(area, (x, y), &Direction::Right, false, max_steps) as usize
        + find_direction(area, (x, y), &Direction::UpLeft, false, max_steps) as usize
        + find_direction(area, (x, y), &Direction::UpRight, false, max_steps) as usize
        + find_direction(area, (x, y), &Direction::DownLeft, false, max_steps) as usize
        + find_direction(area, (x, y), &Direction::DownRight, false, max_steps) as usize
}

fn find_direction(
    area: &WaitingArea,
    coords: (isize, isize),
    direction: &Direction,
    recusive: bool,
    max_steps: isize,
) -> bool {
    if coords.0 < 0
        || coords.1 < 0
        || coords.0 >= area[0].len() as isize
        || coords.1 >= area.len() as isize
    {
        return false;
    }

    if area[coords.1 as usize][coords.0 as usize] == '#' && recusive {
        return true;
    } else if area[coords.1 as usize][coords.0 as usize] == 'L' && recusive {
        return false;
    }

    if max_steps == 0 {
        return false;
    }

    let dir: (isize, isize) = match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::UpLeft => (-1, -1),
        Direction::UpRight => (1, -1),
        Direction::DownLeft => (-1, 1),
        Direction::DownRight => (1, 1),
    };

    let new_coords: (isize, isize) = (dir.0 + coords.0 as isize, dir.1 + (coords.1 as isize));

    let found = find_direction(area, new_coords, direction, true, max_steps - 1);

    found
}

#[allow(dead_code)]
fn print_area(area: &WaitingArea) {
    area.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{}", c));
        println!("");
    });
    println!();
}

#[aoc(day11, part1)]
fn part1(input: &WaitingArea) -> usize {
    let mut mut_area = input.clone();
    let mut old_seated = 0;
    loop {
        iterate_area(&mut mut_area, 1);
        //print_area(&mut_area);

        let seated = mut_area
            .iter()
            .map(|l| l.iter().filter(|&&c| c == '#').count())
            .sum();

        if seated == old_seated {
            return seated;
        }
        old_seated = seated;
    }
}

#[aoc(day11, part2)]
fn part2(input: &WaitingArea) -> usize {
    let mut mut_area = input.clone();
    let mut old_seated = 0;
    loop {
        iterate_area(&mut mut_area, 2);
        //print_area(&mut_area);

        let seated = mut_area
            .iter()
            .map(|l| l.iter().filter(|&&c| c == '#').count())
            .sum();

        if seated == old_seated {
            return seated;
        }
        old_seated = seated;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";

        assert_eq!(part1(&generate(input)), 37);
    }

    #[test]
    fn test_part2_adjacent() {
        let input_adj1 = ".......#.\n...#.....\n.#.......\n.........\n..#L....#\n....#....\n.........\n#........\n...#.....";
        let input_adj2 = ".............\n.L.L.#.#.#.#.\n.............";
        let input_adj3 = ".##.##.\n#.#.#.#\n##...##\n...L...\n##...##\n#.#.#.#\n.##.##.";
        assert_eq!(adjacent(&generate(input_adj1), 3, 4, -1), 8);
        assert_eq!(adjacent(&generate(input_adj2), 1, 1, -1), 0);
        assert_eq!(adjacent(&generate(input_adj3), 3, 3, -1), 0);
    }
    #[test]
    fn test_part2() {
        let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        assert_eq!(part2(&generate(input)), 26);
    }
}

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
fn aoc_generator(input: &str) -> (usize, usize) {
    let mut b = input.lines().map(|l| l.parse::<usize>().unwrap());
    (b.next().unwrap(), b.next().unwrap())
}

fn transform(subject_nr: usize, loop_size: usize) -> usize {
    let mut val = 1;
    for _ in 0..loop_size {
        val *= subject_nr;
        val %= 20201227;
    }

    val
}

fn find_loop_size(pub_key: usize) -> usize {
    let mut val = 1;
    let mut loop_size = 0;
    let subject_nr = 7;
    loop {
        if val == pub_key {
            break;
        }

        val *= subject_nr;
        val %= 20201227;

        loop_size += 1;
    }

    loop_size
}

#[aoc(day25, part1)]
fn part1(input: &(usize, usize)) -> usize {
    let loop_card = find_loop_size(input.0);

    transform(input.1, loop_card)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);

        let input = "5764801\n17807724";
        assert_eq!(part1(&aoc_generator(input)), 14897079);
    }
}

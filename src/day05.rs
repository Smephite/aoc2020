use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::FromIterator;

#[aoc_generator(day5, part1)]
#[aoc_generator(day5, part2)]
fn parse_passes(input: &str) -> Vec<usize> {
    input.lines().map(|l| pass_id(l)).collect()
}

#[aoc_generator(day5, part2, fast_gen)]
fn parse_passes2(input: &str) -> Vec<usize> {
    input.lines().map(|l| pass_id_2(l)).collect()
}

fn convert(code: &str) -> usize {
    code.bytes().rev().enumerate().fold(0, |acc, (i, b)| {
        (((b == b'B' || b == b'R') as usize) << i) + acc
    })
}

fn parse_pass(pass: &str) -> (usize, usize) {
    (
        usize::from_str_radix(
            &String::from_iter(pass[0..7].chars().map(|c| match c {
                'B' => '1',
                _ => '0',
            })),
            2,
        )
        .unwrap(),
        usize::from_str_radix(
            &String::from_iter(pass[7..].chars().map(|c| match c {
                'R' => '1',
                _ => '0',
            })),
            2,
        )
        .unwrap(),
    )
}

fn pass_id_2(pass: &str) -> usize {
    pass.lines()
        .map(|l| l.split_at(7))
        .map(|(row, col)| convert(row) * 8 + convert(col))
        .next()
        .unwrap()
}

fn pass_id(pass: &str) -> usize {
    let pass = parse_pass(pass);
    pass.0 * 8 + pass.1
}

#[aoc(day5, part1)]
fn part1(input: &[usize]) -> Option<usize> {
    input.into_iter().map(|l| l.to_owned()).max()
}

#[aoc(day5, part2)]
fn part2(input: &[usize]) -> Option<usize> {
    (*input.into_iter().min().expect("err min")..*input.into_iter().max().expect("err max"))
        .step_by(1)
        .filter(|s| !input.contains(s))
        .next()
}

#[aoc(day5, part2, fast_gen)]
fn part2_2(input: &[usize]) -> usize {
    input
        .into_iter()
        .filter(|i| !input.contains(&(*i + 1)))
        .next()
        .expect("not found")
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_test() {
        assert_eq!(
            parse_passes("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL"),
            parse_passes2("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL")
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1(&parse_passes("BFFFBBFRRR")), Some(567));
        assert_eq!(part1(&parse_passes("FFFBBBFRRR")), Some(119));
        assert_eq!(part1(&parse_passes("BBFFBBFRLL")), Some(820));
        assert_eq!(
            part1(&parse_passes("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL")),
            Some(820)
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&parse_passes("FFFFFFFLLL\nFFFFFFFLRL")), Some(1));
        assert_eq!(part2_2(&parse_passes("FFFFFFFLLL\nFFFFFFFLRL")), 1);
    }
}

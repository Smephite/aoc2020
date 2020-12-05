use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::FromIterator;

#[aoc_generator(day5)]
fn parse_passes(input: &str) -> Vec<usize> {
    input.lines().map(|l| pass_id(l)).collect()
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

#[cfg(test)]
mod tests {
    use super::*;
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
    }
}

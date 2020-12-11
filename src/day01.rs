use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(expenses: &[i32]) -> i32 {
    for expense in expenses {
        let diff = 2020 - expense;
        if expenses.contains(&diff) {
            return expense * diff;
        }
    }
    -1
}
#[aoc(day1, part1, array)]
fn part1_arr(expenses: &[i32]) -> i32 {
    let mut array: [bool; 2020] = [false; 2020];

    for expense in expenses {
        if array[*expense as usize] {
            return expense * (2020 - expense);
        }
        array[(2020 - expense) as usize] = true;
    }

    -1
}

#[aoc(day1, part2)]
fn part2(expenses: &[i32]) -> i32 {
    for (i, expense1) in expenses.into_iter().enumerate() {
        let diff1 = 2020 - expense1;
        for expense2 in &expenses[i + 1..].to_vec() {
            let diff2 = diff1 - expense2;
            if expenses.contains(&diff2) {
                return expense1 * expense2 * diff2;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }
    #[test]
    fn part1_example_array() {
        assert_eq!(part1_arr(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}

use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn part1(expenses: &[i32]) -> i32 {
    for expense in expenses {
        if expenses.contains(&(2020 - expense)) {
            return expense*(2020-expense);
        }
    }
    -1
}

#[aoc(day1, part2)]
fn part2(expenses: &[i32]) -> i32 {
    for expense1 in expenses {
        for expense2 in expenses {
            if expenses.contains(&(2020 - expense1 - expense2)) {
                return expense1*expense2*(2020 - expense1 - expense2);
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
    fn part2_example() {
        assert_eq!(part2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}
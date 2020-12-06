use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .bytes()
                .filter(|c| c.is_ascii_alphabetic())
                .fold([false; 26], |mut arr, c| {
                    arr[(c - b'a') as usize] = true;
                    arr
                })
                .iter()
                .filter(|b| **b)
                .count()
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .bytes()
                .filter(|c| c.is_ascii_alphabetic())
                .fold([0_usize; 26], |mut arr, c| {
                    arr[(c - b'a') as usize] += 1;
                    arr
                })
                .iter()
                .filter(|b| **b == group.lines().count())
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("abcx\nabcy\nabcz"), 6);
        assert_eq!(part1("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"), 11);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"), 6);
    }
}

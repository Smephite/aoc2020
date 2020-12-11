use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn aoc_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

#[aoc(day9, part1)]
fn run_part1(input: &[usize]) -> usize {
    part1(input, 0, 25)
}

fn part1(input: &[usize], index: usize, size: usize) -> usize {
    let slice = input[index..index + size].to_vec();

    let boss = input[index + size];

    let matches = slice
        .iter()
        .filter(|&&i| boss > i && slice.contains(&(boss - i)) && boss - i != i)
        .count();

    if matches == 0 {
        input[index + size]
    } else {
        part1(input, index + 1, size)
    }
}

#[aoc(day9, part2)]
fn run_part2(input: &[usize]) -> usize {
    part2(input, 2, run_part1(input))
}

fn part2(input: &[usize], search: usize, invalid_num: usize) -> usize {
    let invalid_index = input.iter().position(|&i| i == invalid_num).unwrap();

    if search >= invalid_index - 1 {
        panic!("No possible solultion!");
    }
    //println!("Starting search with {} #s", search);

    let mut lower_bound = 0;
    loop {
        let upper_bound = lower_bound + search; // not inclusive

        //println!("Searching {}-{}", lower_bound, upper_bound);

        if upper_bound - 1 == invalid_index {
            return part2(input, search + 1, invalid_num);
        }

        if input[lower_bound..upper_bound].iter().sum::<usize>() == invalid_num {
            return input[lower_bound..upper_bound].iter().min().unwrap()
                + input[lower_bound..upper_bound].iter().max().unwrap();
        }

        lower_bound += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";

        assert_eq!(part1(&aoc_generator(input), 0, 5), 127);
    }

    #[test]
    fn test_part2() {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        let gen = &aoc_generator(input);
        assert_eq!(part2(gen, 2, part1(gen, 0, 5)), 62);
    }
}

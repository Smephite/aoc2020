use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<usize> {
    let mut out = input
        .lines()
        .map(|l| usize::from_str_radix(l, 10).unwrap())
        .collect::<Vec<usize>>();
    out.push(0);
    out.push(out.iter().max().unwrap() + 3); // built-in joltage
    out.sort();
    out
}

#[aoc(day10, part1)]
fn part1(input: &[usize]) -> usize {
    let (one, three, _) = input
        .iter()
        .fold((0, 0, 0), |(one, three, old), &new| match new - old {
            1 => (one + 1, three, new),
            3 => (one, three + 1, new),
            _ => (one, three, new),
        });
    one * three
}

#[aoc(day10, part2)]
fn part2(input: &[usize]) -> usize {
    println!("part2 {:?}", input.to_vec());
    count_posibilities(input, &0)
}

fn count_posibilities(input: &[usize], start: &usize) -> usize {
    if start >= &(input.len() - 1) {
        // last element
        return 1;
    }

    if input[start + 1] - input[*start] > 3 {
        // first diff is greater 3 --> all following can only be equal or larger --> not valid
        return 0;
    }

    let mut ret = 0;

    for i in (1..4)
        .into_iter()
        .filter(|&i| start + i < input.len() && input[start + i] - input[*start] <= 3)
    {
        if *start == 0 {
            println!("i {}", i)
        }
        let diff = input[start + i] - input[*start];
        if diff > 3 {
            // if first diff is > 3 no can follow, scnd diff > 3 -> no third possible
            break; // thus no need to check
        }

        ret += count_posibilities(input, &(start + i));
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";

        assert_eq!(part1(&generator(input)), 22 * 10);
    }

    #[test]
    fn test_part2() {
        let input1 = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        let input2 = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";

        assert_eq!(part2(&generator(input1)), 8);
        assert_eq!(part2(&generator(input2)), 19208);
    }
}

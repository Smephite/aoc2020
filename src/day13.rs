use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let time = lines.next().unwrap().parse::<usize>().unwrap();

    let mut busses = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|&x| x != "x")
        .map(|id| {
            let id = id.parse::<usize>().unwrap();
            (id - time % id, id)
        })
        .collect::<Vec<(usize, usize)>>();

    busses.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    busses[0].0 * busses[0].1
}

#[aoc(day13, part2)] // This takes only 85min!
fn part2_slow(input: &str) -> usize {
    let timetable = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .enumerate()
        .map(|x| {
            (
                x.0,
                match x.1 {
                    "x" => 0,
                    _ => x.1.parse::<usize>().unwrap(),
                },
            )
        })
        .filter(|x| x.1 != 0)
        .collect::<Vec<(usize, usize)>>();

    println!("{:?}", timetable);

    let max = timetable.iter().max_by_key(|x| x.1).unwrap();

    let mut num = max.1;

    loop {
        let mut success = true;

        for i in 0..timetable.len() {
            if timetable[i].0 == max.0 || timetable[i].1 == 0 {
                continue;
            }

            if (timetable[i].0 as isize - max.0 as isize + num as isize) % timetable[i].1 as isize
                == 0
            {
                continue;
            }

            success = false;
            break;
        }

        if success {
            break;
        }

        //println!("{}", num);
        num += max.1;
    }

    num - max.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "939\n7,13,x,x,59,x,31,19";

        assert_eq!(part1(input), 295);
    }

    #[test]
    fn test_part2() {
        let input = "939\n7,13,x,x,59,x,31,19";
        assert_eq!(part2_slow(input), 1068781);
        let input = "939\n17,x,13,19";
        assert_eq!(part2_slow(input), 3417);
        let input = "939\n67,7,59,61";
        assert_eq!(part2_slow(input), 754018);
        let input = "939\n67,x,7,59,61";
        assert_eq!(part2_slow(input), 779210);
        let input = "939\n67,7,x,59,61";
        assert_eq!(part2_slow(input), 1261476);
        let input = "939\n1789,37,47,1889";
        assert_eq!(part2_slow(input), 1202161486);
    }
}

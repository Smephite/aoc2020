use crate::intcode::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<(Operation, Vec<isize>)> {
    input
        .lines()
        .map(|l| {
            let mut args = l.split(" ");
            let op = Operation::find(args.next().unwrap());
            let op_args = args.map(|d| d.parse::<isize>().ok().unwrap()).collect();
            (op, op_args)
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(instructions: &Vec<(Operation, Vec<isize>)>) -> isize {
    let mut p = Processor::default();
    p.load(instructions);
    p.run_no_duplicate().get_accumulator()
}

#[aoc(day8, part2)]
fn part2(instructions: &Vec<(Operation, Vec<isize>)>) -> isize {
    for (index, ins) in instructions.iter().enumerate() {
        if !(ins.0 == Operation::JMP || ins.0 == Operation::NOP) {
            continue;
        }

        let mut cloned_ins = instructions.clone();

        cloned_ins[index].0 = match ins.0 {
            Operation::JMP => Operation::NOP,
            _ => Operation::JMP,
        };

        let mut cpu = Processor::default();
        cpu.load(&cloned_ins);
        cpu.run_no_duplicate();

        if cpu.get_exit_code() == 0 {
            println!("Error in line {}", index + 1);
            return cpu.get_accumulator();
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

        assert_eq!(part1(&generator(input)), 5);
    }

    #[test]
    fn test_part2() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

        assert_eq!(part2(&generator(input)), 8);
    }
}

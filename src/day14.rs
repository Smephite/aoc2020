use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::iter::FromIterator;

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .fold(
            (String::from_iter(['X'; 36].iter()), HashMap::new()),
            |(mut mask, mut memory), instruction| {
                match instruction.split(&[' ', '['][..]).next().unwrap() {
                    "mask" => {
                        mask = String::from(instruction.split(" = ").nth(1).unwrap());
                    }
                    "mem" => {
                        let mut data = instruction.split(&['[', ']', ' '][..]);
                        let address = data.nth(1).unwrap().parse::<usize>().unwrap();
                        let number = data.nth(2).unwrap().parse::<usize>().unwrap();

                        memory.insert(
                            address,
                            mask.chars().rev().enumerate().fold(
                                number,
                                |number, (index, c)| match c {
                                    '1' => number | (1 << index),
                                    '0' => number & !(1 << index),
                                    _ => number,
                                },
                            ),
                        );
                    }
                    _ => unimplemented!(),
                };
                (mask, memory)
            },
        )
        .1
        .values()
        .sum()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .fold(
            (String::from_iter(['X'; 36].iter()), &mut HashMap::new()),
            |(mut mask, memory), instruction| {
                match instruction.split(&[' ', '['][..]).next().unwrap() {
                    "mask" => {
                        mask = String::from(instruction.split(" = ").nth(1).unwrap());
                    }
                    "mem" => {
                        let mut data = instruction.split(&['[', ']', ' '][..]);
                        let address = data.nth(1).unwrap().parse::<usize>().unwrap();
                        let number = data.nth(2).unwrap().parse::<usize>().unwrap();

                        write_to_bitmasked_address(mask.clone(), address, number, memory, 0);

                        fn write_to_bitmasked_address(
                            mut mask: String,
                            address: usize,
                            value: usize,
                            memory: &mut HashMap<usize, usize>,
                            bit_index: usize,
                        ) {
                            if mask.len() == 0 {
                                memory.insert(address, value);
                                return;
                            }

                            let bit = mask.pop().unwrap();
                            match bit {
                                '0' => write_to_bitmasked_address(
                                    mask,
                                    address,
                                    value,
                                    memory,
                                    bit_index + 1,
                                ),
                                '1' => write_to_bitmasked_address(
                                    mask,
                                    address | (1 << bit_index),
                                    value,
                                    memory,
                                    bit_index + 1,
                                ),
                                _ => {
                                    write_to_bitmasked_address(
                                        mask.clone(),
                                        address | (1 << bit_index),
                                        value,
                                        memory,
                                        bit_index + 1,
                                    );
                                    write_to_bitmasked_address(
                                        mask.clone(),
                                        address & !(1 << bit_index),
                                        value,
                                        memory,
                                        bit_index + 1,
                                    );
                                }
                            }
                        }
                    }
                    _ => unimplemented!(),
                };
                (mask, memory)
            },
        )
        .1
        .values()
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        assert_eq!(part1(input), 165);
    }

    #[test]
    fn test_part2() {
        let input = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        assert_eq!(part2(input), 208);
    }
}

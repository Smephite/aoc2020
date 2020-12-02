use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day2)]
fn aoc_generator(input: &str) -> Vec<PasswordPolicy> {
    input.lines().map(|l| PasswordPolicy::parse(l)).collect()
}



struct PasswordPolicy {
    range: (i32, i32),
    character: char,
    password: Vec<char>
}

impl PasswordPolicy {
    fn parse(string: &str) -> PasswordPolicy
    {
        let mut policy = PasswordPolicy::empty();
        let space_split = string.split_whitespace().collect::<Vec<&str>>();
        let number_values = space_split[0].split("-").map(|l| l.parse().unwrap()).collect::<Vec<i32>>();

        policy.range = (number_values[0], number_values[1]);
        policy.character = space_split[1][0..1].chars().next().unwrap();
        policy.password = space_split[2].chars().collect();

        policy
    }
    fn empty() -> PasswordPolicy {
        PasswordPolicy{
            range: (0, 0),
            character: ' ',
            password: "".chars().collect()
        }
    }
}

#[aoc(day2, part1)]
fn part1(input: &[PasswordPolicy]) -> i32 {
    let mut valid = 0;
    for policy in input {
        let mut occ = 0;
        for character in &policy.password {
            if *character == policy.character {
                occ += 1
            }
        }
        if policy.range.0 <= occ && policy.range.1 >= occ {
            valid += 1
        }
    }

    valid
}

// quick and dirty
//#[aoc(day2, part1, dirty)]
#[allow(dead_code)]
fn part1_dirty(input: &str) -> i32 {
        
    let split = input.split("\n");
    let mut val = 0;

    for s in split
    {
        let spaces = s.split_whitespace().collect::<Vec<&str>>();

        let num = spaces[0];
        let int = num.split("-").collect::<Vec<&str>>();

        let a = int[0].parse().expect("not a number");
        let b = int[1].parse().expect("not a number");

        let c : char = spaces[1][0..1].chars().collect::<Vec<char>>()[0];

        let first_part : Vec<&str>= s.split(":").collect();

        let pwd : &str = &first_part[1][1..];
        let mut occ = 0;
        //println!("{}-{} {}: {}", a, b, c, pwd);
        for cc in pwd.chars().collect::<Vec<char>>()
        {
            //print!("{}",c);
            if c == cc{
                occ+=1;
            }
        }

        //print!("\n{} found {} times: ", c, occ);
        if occ >= a && occ <= b {
            val+=1;
            //print!("pass!");
        }

        //print!("\n");

    }



    val
}

#[aoc(day2, part2)]
fn part2(input: &[PasswordPolicy]) -> i32 
{   
    let mut valid = 0;
    for policy in input {

        if (policy.password[(policy.range.0 - 1)as usize] == policy.character) ^ (policy.password[(policy.range.1 - 1) as usize] == policy.character) {
            valid += 1
        }
    }
    valid
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_dirty_test() {
        assert_eq!(part1_dirty("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 2);
        assert_eq!(part1_dirty("1-1 a: abcde\n1-1 b: acadefg\n2-9 c: ccccccccc"), 2);
    }
    #[test]
    fn part1_test() {
        assert_eq!(part1(&aoc_generator("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc")), 2);
        
        assert_eq!(part1(&aoc_generator("1-1 a: abcde\n1-1 b: acadefg\n2-9 c: ccccccccc")), 2);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(&aoc_generator("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc")), 1)
    }
}
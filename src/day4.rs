use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeMap;

#[aoc_generator(day4, part1, def)]
#[aoc_generator(day4, part2, def)]
fn generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|p_str| {
            let mut pass: Passport = Passport::default();
            for z in p_str.split_whitespace() {
                let entry = z.split(":").collect::<Vec<&str>>();

                let key: &str = &entry[0];

                match key {
                    "byr" => pass.byr = String::from(entry[1]),
                    "iyr" => pass.iyr = String::from(entry[1]),
                    "eyr" => pass.eyr = String::from(entry[1]),
                    "hgt" => pass.hgt = String::from(entry[1]),
                    "hcl" => pass.hcl = String::from(entry[1]),
                    "ecl" => pass.ecl = String::from(entry[1]),
                    "pid" => pass.pid = String::from(entry[1]),
                    "cid" => pass.cid = String::from(entry[1]),
                    _ => print!("Unhandled!"),
                };
            }
            pass
        })
        .collect()
}

// inspiration from https://github.com/pedantic79/advent-of-code-2020/commit/725b8eca3fbcad264deed4fa33f311ccaaaa31b5

#[aoc_generator(day4, part1, fast)]
#[aoc_generator(day4, part2, fast)]
fn generator_2(input: &str) -> Option<Vec<FastPassport>> {
    input
        .split("\n\n")
        .map(|pass_str| {
            pass_str
                .split_whitespace()
                .map(|field| {
                    let split = field
                        .split(":")
                        .map(|s| String::from(s))
                        .collect::<Vec<String>>();
                    Some((split[0].to_owned(), split[1].to_owned()))
                })
                .collect::<Option<_>>()
                .map(FastPassport)
        })
        .collect()
}

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Default for Passport {
    fn default() -> Passport {
        Passport {
            byr: String::new(),
            iyr: String::new(),
            eyr: String::new(),
            hgt: String::new(),
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: String::new(),
        }
    }
}

impl Passport {
    fn is_valid_1(&self) -> bool {
        self.byr != ""
            && self.iyr != ""
            && self.eyr != ""
            && self.hgt != ""
            && self.hcl != ""
            && self.ecl != ""
            && self.pid != ""
    }

    fn is_valid_2(&self) -> bool {
        matches!(scan_fmt!(&self.byr, "{/\\d{4}$/}", usize), Ok(x) if x >= 1920 && x <= 2002)
            && matches!(scan_fmt!(&self.iyr, "{/\\d{4}$/}", usize), Ok(x) if x >= 2010 && x <= 2020)
            && matches!(scan_fmt!(&self.eyr, "{/\\d{4}$/}", usize), Ok(x) if x >= 2020 && x <= 2030)
            && matches!(scan_fmt!(&self.hgt, "{d}{}", usize, String), Ok((h, u)) if (u == "cm" && h >= 150 && h <= 193) || (u == "in" && h >= 59 && h <= 76))
            && matches!(
                scan_fmt!(&self.hcl, "#{/[a-f0-9]{6}$/}", [hex usize]),
                Ok(_)
            )
            && matches!(
                &*self.ecl,
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            )
            && matches!(scan_fmt!(&self.pid, "{/\\d{9}$/}", usize), Ok(_))
    }
}

#[derive(Debug)]
struct FastPassport(BTreeMap<String, String>);

impl FastPassport {
    fn is_valid_1(&self) -> bool {
        self.0
            .keys()
            .filter(|&key| key != "cid")
            .eq(["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"].iter())
    }

    fn is_valid_2(&self) -> bool {
        self.is_valid_1()
            && self.0.iter().all(|(k, v)| match &k[..] {
                "byr" => {
                    v.chars().all(|c| c.is_ascii_digit()) && {
                        let y = v.parse::<u16>().unwrap_or(0);
                        y >= 1920 && y <= 2002
                    }
                }
                "iyr" => {
                    v.chars().all(|c| c.is_ascii_digit()) && {
                        let y = v.parse::<u16>().unwrap_or(0);
                        y >= 2010 && y <= 2020
                    }
                }
                "eyr" => {
                    v.chars().all(|c| c.is_ascii_digit()) && {
                        let y = v.parse::<u16>().unwrap_or(0);
                        y >= 2020 && y <= 2030
                    }
                }
                "hgt" => {
                    let h = v[..v.len() - 2].parse::<u16>().unwrap_or(0);
                    match &v[v.len() - 2..] {
                        "cm" => h >= 150 && h <= 193,
                        "in" => h >= 59 && h <= 76,
                        _ => false,
                    }
                }
                "hcl" => {
                    let chars = v.chars().collect::<Vec<char>>();
                    chars[0] == '#'
                        && chars[1..].iter().all(|c| c.is_ascii_hexdigit())
                        && v.len() == 7
                }
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                    .iter()
                    .any(|e| e == v),
                "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
                "cid" => true, // ignore
                _ => false,
            })
    }
}

#[aoc(day4, part1, def)]
fn part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid_1()).count()
}

#[aoc(day4, part2, def)]
fn part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid_2()).count()
}

#[aoc(day4, part1, fast)]
fn part1_2(passports: &[FastPassport]) -> usize {
    passports.iter().filter(|p| p.is_valid_1()).count()
}

#[aoc(day4, part2, fast)]
fn part2_2(passports: &[FastPassport]) -> usize {
    passports.iter().filter(|p| p.is_valid_2()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let g =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\n\
    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\n\
    hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\n\
    hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";

        let data = generator(g);
        assert_eq!(part1(&data), 2);
        assert_eq!(part1_2(&generator_2(g).unwrap()), 2);
    }

    #[test]
    fn part2_test() {
        let invalid =
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\n\
        iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946\n\n\
        hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\n\
        hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007";
        let valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f\n\n\
        eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\n\
        hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022\n\n
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        println!("Invalid:");
        assert_eq!(part2(&generator(invalid)), 0);
        assert_eq!(part2_2(&generator_2(invalid).unwrap()), 0);
        println!("Valid:");
        assert_eq!(part2(&generator(valid)), 4);
        assert_eq!(part2_2(&generator_2(valid).unwrap()), 4);
    }
}

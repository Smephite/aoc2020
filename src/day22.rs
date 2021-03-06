use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day22)]
fn aoc_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut data = input.split("\n\n").map(|block| {
        let mut lines = block.lines();
        lines.next();
        lines.map(|l| l.parse::<usize>().unwrap()).rev().collect()
    });

    (data.next().unwrap(), data.next().unwrap())
}

#[aoc(day22, part1)]
fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut cards = input.clone();

    loop {
        if cards.0.len() == 0 || cards.1.len() == 0 {
            break;
        }

        let p1 = cards.0.pop().unwrap();
        let p2 = cards.1.pop().unwrap();

        if p1 > p2 {
            let mut v = vec![p2, p1];
            v.append(&mut cards.0);
            cards.0 = v;
        } else {
            let mut v = vec![p1, p2];
            v.append(&mut cards.1);
            cards.1 = v;
        }
    }

    if cards.0.len() == 0 {
        cards
            .1
            .iter()
            .fold((0, 1), |(old_val, mult), card| {
                (old_val + mult * card, mult + 1)
            })
            .0
    } else {
        cards
            .0
            .iter()
            .fold((0, 1), |(old_val, mult), card| {
                (old_val + mult * card, mult + 1)
            })
            .0
    }
}

#[aoc(day22, part2)]
fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut cards = input.clone();

    if play_p2(&mut cards) {
        cards
            .0
            .iter()
            .fold((0, 1), |(old_val, mult), card| {
                (old_val + mult * card, mult + 1)
            })
            .0
    } else {
        cards
            .1
            .iter()
            .fold((0, 1), |(old_val, mult), card| {
                (old_val + mult * card, mult + 1)
            })
            .0
    }
}

fn hash(cards: &Vec<usize>) -> usize {
    cards
        .iter()
        .fold((0, 1), |(old_val, mult), card| {
            (old_val + mult * card, mult * 100)
        })
        .0
}

// true p1 wins
fn play_p2(cards: &mut (Vec<usize>, Vec<usize>)) -> bool {
    let mut saved_hashes: Vec<(usize, usize)> = vec![];
    loop {
        if cards.0.len() == 0 || cards.1.len() == 0 {
            break;
        }

        let hash = (hash(&cards.0), hash(&cards.1));
        // break condition
        if saved_hashes.contains(&hash) {
            return true;
        }

        saved_hashes.push(hash);

        let p1 = cards.0.pop().unwrap();
        let p2 = cards.1.pop().unwrap();

        let winner;

        if p1 <= cards.0.len() && p2 <= cards.1.len() {
            // recusive
            winner = play_p2(&mut (
                cards.0[cards.0.len() - p1..].to_vec(),
                cards.1[cards.1.len() - p2..].to_vec(),
            ));
        } else {
            winner = p1 > p2;
        }

        if winner {
            let mut v = vec![p2, p1];
            v.append(&mut cards.0);
            cards.0 = v;
        } else {
            let mut v = vec![p1, p2];
            v.append(&mut cards.1);
            cards.1 = v;
        }
    }

    cards.0.len() != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";

        assert_eq!(part1(&aoc_generator(input)), 306);
    }

    #[test]
    fn test_part2() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";

        assert_eq!(part2(&aoc_generator(input)), 291);
    }
}

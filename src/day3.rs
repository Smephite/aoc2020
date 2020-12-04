use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn aoc_generator(input : &str) -> Map{
    let data = input.lines().collect::<Vec<&str>>();    
    let mut map = Map::create(&data.len(), &data[0].len());

    let mut index = 0;

    for line in data {
        map.set_line(&index, line);
        index+=1;
    }

    map

}
// this one is fancier but slower
//#[aoc_generator(day3)]
#[allow(dead_code)]
fn aoc_generator_2(input : &str) -> Map {
    let data : Vec<Vec<bool>> = input.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect();
    let mut map = Map::create(&data.len(), &data[0].len());
    map.data = data;

    map
}

#[derive(Debug)]
struct Map {
    height: usize,
    width: usize,
    data: Vec<Vec<bool>>
}

impl Map {
    fn create(height: &usize, width: &usize) -> Map
    {
        Map {
            height: (*height),
            width: (*width),
            data: vec![vec![false; *width]; *height]
        }
    }

    fn set_line(&mut self, line: &usize, input: &str)
    {
        assert_eq!(*line < self.height, true);
        assert_eq!(input.len(), self.width);

        for (i, c) in input.char_indices(){
            self.data[*line][i] = c == '#';
        }
    }

    fn get_position(&self, x: &usize, y: &usize) -> bool
    {
        assert_eq!(*y < self.height, true);
        self.data[*y][x % self.width]
    }

    fn count_trees(&self, x_inc: usize, y_inc: usize) -> usize {
        let mut x = 0;

        (0..self.height).step_by(y_inc)
        .filter(|&r| {
            let x_org = x;
            x += x_inc;
            if x >= self.width {
                x -= self.width;
            }
            self.data[r][x_org]
        }).count()
    }
}


#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut trees = 0;
    
    while y < map.height {
        if map.get_position(&x, &y) {
            trees += 1
        }
        x += 3;
        y += 1;
    }
    trees
}

#[aoc(day3, part1, no_mod)]
fn part1_no_mod(map: &Map) -> usize {
    map.count_trees(3, 1)
}

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
    let mut response = 1;
    let slopes = [(1, 1), (3,1), (5,1), (7,1), (1,2)];

    for slope in slopes.iter() {
        let mut y = 0;
        let mut x = 0;

        let mut trees = 0;
        
        while y < map.height {
            if map.get_position(&x, &y) {
                trees += 1;
            }
            x += slope.0;
            y += slope.1;
        }
        
        response *= trees;
    }

    response
}

#[aoc(day3, part2, no_mod)]
fn part2_no_mod(map: &Map) -> usize {
    let slopes = [(1, 1), (3,1), (5,1), (7,1), (1,2)];

    slopes.iter().map(|(x, y)| map.count_trees(*x, *y)).product()
}






#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let test_data = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(part1(&aoc_generator(test_data)), 7);
        assert_eq!(part1_no_mod(&aoc_generator(test_data)), 7);
    }

    #[test]
    fn part2_test() {
        let test_data = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
        assert_eq!(part2(&aoc_generator(test_data)), 336);
        assert_eq!(part2_no_mod(&aoc_generator(test_data)), 336);
    }

}
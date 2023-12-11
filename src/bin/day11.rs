use std::collections::HashSet;

use adventofcode2023::read_input;

struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn distance(
        &self,
        other: &Galaxy,
        verticals: &HashSet<usize>,
        horizontals: &HashSet<usize>,
        empty_bonus: usize,
    ) -> usize {
        let mut result = self.x.abs_diff(other.x) + self.y.abs_diff(other.y);
        for i in self.x.min(other.x) + 1..self.x.max(other.x) {
            if !verticals.contains(&i) {
                result += empty_bonus;
            }
        }
        for i in self.y.min(other.y) + 1..self.y.max(other.y) {
            if !horizontals.contains(&i) {
                result += empty_bonus;
            }
        }
        result
    }
}

fn parse(input: &str) -> (Vec<Galaxy>, HashSet<usize>, HashSet<usize>) {
    let mut verticals = HashSet::new();
    let mut horizontals = HashSet::new();
    let mut galaxies = vec![];
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                verticals.insert(j);
                horizontals.insert(i);
                galaxies.push(Galaxy { x: j, y: i });
            }
        }
    }
    (galaxies, verticals, horizontals)
}

fn process(input: &str, empty_bonus: usize) -> usize {
    let mut result = 0;
    let (galaxies, verticals, horizontals) = parse(input);
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            result += galaxies[i].distance(&galaxies[j], &verticals, &horizontals, empty_bonus);
        }
    }
    result
}

fn part1(input: &str) -> usize {
    process(input, 1)
}

fn part2(input: &str) -> usize {
    process(input, 1000000 - 1)
}

fn main() {
    let input = read_input(11);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    ";

    #[test]
    fn test_day11_part1() {
        assert_eq!(part1(INPUT.trim()), 374);
    }

    #[test]
    fn test_day11_part2() {
        assert_eq!(process(INPUT.trim(), 9), 1030);
        assert_eq!(process(INPUT.trim(), 99), 8410);
    }
}

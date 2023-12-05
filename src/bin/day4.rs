use adventofcode2023::read_input;

use std::collections::HashSet;

struct Card {
    winning: HashSet<i32>,
    numbers: HashSet<i32>,
}

impl Card {
    fn count(&self) -> usize {
        let mut result = 0;
        for w in &self.winning {
            if self.numbers.contains(w) {
                result += 1;
            }
        }
        result
    }

    fn points(&self) -> i32 {
        let count = self.count();
        match count {
            0 => 0,
            x => 2i32.pow(x as u32 - 1),
        }
    }
}

fn parse(input: &str) -> Vec<Card> {
    let mut result = vec![];
    for line in input.lines() {
        let line = line.trim();
        let parts = line.split(": ").collect::<Vec<_>>()[1]
            .split(" | ")
            .collect::<Vec<_>>();
        result.push(Card {
            winning: parts[0]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            numbers: parts[1]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        });
    }
    result
}

fn part1(input: &str) -> i32 {
    let cards = parse(input);
    cards.iter().map(|x| x.points()).sum()
}

fn part2(input: &str) -> i32 {
    let cards = parse(input);
    let mut multiplier = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        for j in 0..card.count() {
            multiplier[i + j + 1] += multiplier[i];
        }
    }
    multiplier.iter().sum()
}

fn main() {
    let input = read_input(4);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part1() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_day4_part2() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        assert_eq!(part2(input), 30);
    }
}

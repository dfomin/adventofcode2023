use std::{cmp::Ordering, collections::HashMap};

use adventofcode2023::read_input;

struct HandsManager {
    cards_order: &'static str,
    cards_map: HashMap<char, usize>,
}

impl HandsManager {
    fn new(cards_order: &'static str) -> Self {
        let mut cards_map = HashMap::new();
        for (i, card) in cards_order.chars().enumerate() {
            cards_map.insert(card, i);
        }
        Self {
            cards_order,
            cards_map,
        }
    }

    fn stats(&self, hand: &str) -> [i32; 5] {
        let mut cards = vec![0; self.cards_map.len()];
        for card in hand.chars() {
            cards[self.cards_map[&card]] += 1;
        }
        let mut stats = [0; 5];
        let mut jokers = 0;
        if self.cards_order.chars().next().unwrap() == 'J' {
            jokers = cards[0];
            if jokers == 5 {
                stats[4] = 1;
                return stats;
            }
            cards[0] = 0;
        }
        for card in cards {
            if card > 0 {
                stats[card - 1] += 1;
            }
        }
        for i in (0..stats.len()).rev() {
            if stats[i] > 0 {
                stats[i] -= 1;
                stats[i + jokers] += 1;
                break;
            }
        }
        stats
    }
}

fn compare(hands_manager: &HandsManager, first: &str, second: &str) -> Ordering {
    let stats1 = hands_manager.stats(first);
    let stats2 = hands_manager.stats(second);
    for (v1, v2) in stats1.iter().rev().zip(stats2.iter().rev()) {
        if v1 < v2 {
            return Ordering::Less;
        } else if v1 > v2 {
            return Ordering::Greater;
        }
    }
    for (ch1, ch2) in first.chars().zip(second.chars()) {
        let i1 = hands_manager.cards_order.find(ch1).unwrap();
        let i2 = hands_manager.cards_order.find(ch2).unwrap();
        if i1 < i2 {
            return Ordering::Less;
        } else if i1 > i2 {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

fn parse(input: &str) -> Vec<(String, i32)> {
    let mut bids = vec![];
    for line in input.lines() {
        let line = line.trim();
        let parts = line.split_whitespace().collect::<Vec<_>>();
        bids.push((String::from(parts[0]), parts[1].parse().unwrap()));
    }
    bids
}

fn process(input: &str, cards_order: &'static str) -> i32 {
    let mut bids = parse(input);
    let hands_manager = HandsManager::new(cards_order);
    let mut result = 0;
    bids.sort_by(|a, b| compare(&hands_manager, &a.0, &b.0));
    for (i, bid) in bids.iter().enumerate() {
        result += (i as i32 + 1) * bid.1;
    }
    result
}

fn part1(input: &str) -> i32 {
    let cards_order = "23456789TJQKA";
    process(input, cards_order)
}

fn part2(input: &str) -> i32 {
    let cards_order = "J23456789TQKA";
    process(input, cards_order)
}

fn main() {
    let input = read_input(7);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        let input = "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "
        .trim();

        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn test_day7_part2() {
        let input = "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "
        .trim();

        assert_eq!(part2(input), 5905);
    }
}

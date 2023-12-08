use std::collections::HashMap;

use adventofcode2023::read_input;

fn parse(input: &str) -> (String, HashMap<String, Vec<String>>) {
    let mut mapping = HashMap::new();
    let mut iter = input.lines();
    let instructions = iter.next().unwrap().to_string();
    iter.next();
    for line in iter {
        let line = line.trim();
        let parts = line.split(" = ").collect::<Vec<_>>();
        let targets = parts[1].split(", ").collect::<Vec<_>>();
        mapping.insert(
            parts[0].to_string(),
            vec![
                targets[0].replace("(", "").to_string(),
                targets[1].replace(")", "").to_string(),
            ],
        );
    }
    (instructions, mapping)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        let t = a;
        a = b;
        b = t;
    }

    loop {
        a = a % b;
        if a == 0 {
            return b;
        }
        let t = a;
        a = b;
        b = t;
    }
}

fn lcm(numbers: Vec<usize>) -> usize {
    let mut current_gcd = numbers[0];
    let mut result = numbers[0];
    for i in 1..numbers.len() {
        current_gcd = gcd(current_gcd, numbers[i]);
        result *= numbers[i];
        result /= current_gcd;
    }
    result
}

fn part1(instructions: &str, mapping: &HashMap<String, Vec<String>>) -> usize {
    let mut position = "AAA";
    let mut steps = 1;
    loop {
        for ch in instructions.chars() {
            position = &mapping[position][if ch == 'L' { 0 } else { 1 }];
            if position == "ZZZ" {
                return steps;
            }

            steps += 1;
        }
    }
}

fn part2(instructions: &str, mapping: &HashMap<String, Vec<String>>) -> usize {
    let mut positions = vec![];
    for position in mapping.keys() {
        if position.chars().last().unwrap() == 'A' {
            positions.push(position);
        }
    }

    let mut steps = 1;
    let mut first_values = vec![0; positions.len()];
    let mut values_set = 0;
    loop {
        for ch in instructions.chars() {
            for i in 0..positions.len() {
                let position = &mapping[positions[i]][if ch == 'L' { 0 } else { 1 }];
                positions[i] = position;
                if position.chars().last().unwrap() == 'Z' {
                    if first_values[i] == 0 {
                        first_values[i] = steps;
                        values_set += 1;
                        if values_set == positions.len() {
                            return lcm(first_values);
                        }
                    }
                }
            }

            steps += 1;
        }
    }
}

fn main() {
    let input = read_input(8);
    let (instructions, mapping) = parse(&input);

    println!("{}", part1(&instructions, &mapping));
    println!("{}", part2(&instructions, &mapping));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1() {
        let input = "
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "
        .trim();

        let (instructions, mapping) = parse(&input);
        assert_eq!(part1(&instructions, &mapping), 2);

        let input = "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "
        .trim();

        let (instructions, mapping) = parse(&input);
        assert_eq!(part1(&instructions, &mapping), 6);
    }

    #[test]
    fn test_day8_part2() {
        let input = "
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "
        .trim();

        let (instructions, mapping) = parse(&input);
        assert_eq!(part2(&instructions, &mapping), 6);
    }
}

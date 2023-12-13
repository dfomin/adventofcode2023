use core::panic;

use adventofcode2023::read_input;

struct Pattern {
    vertical: Vec<usize>,
    horizontal: Vec<usize>,
}

impl Pattern {
    fn from(pattern: &Vec<Vec<char>>) -> Self {
        let mut vertical = vec![0; pattern[0].len()];
        let mut horizontal = vec![0; pattern.len()];
        for (i, line) in pattern.iter().enumerate() {
            for (j, ch) in line.iter().enumerate() {
                if *ch == '#' {
                    vertical[j] |= 1 << i;
                    horizontal[i] |= 1 << j;
                }
            }
        }

        Self {
            vertical,
            horizontal,
        }
    }

    fn check(vertical: &Vec<usize>, horizontal: &Vec<usize>, skip: usize) -> Option<usize> {
        for (coeff, array) in [vertical, horizontal].iter().enumerate() {
            for i in 1..array.len() {
                let mut left = i - 1;
                let mut right = i;
                let mut equal = true;
                loop {
                    if array[left] != array[right] {
                        equal = false;
                        break;
                    }

                    if right == array.len() - 1 || left == 0 {
                        break;
                    }

                    left -= 1;
                    right += 1;
                }

                if equal {
                    let result = i * if coeff == 1 { 100 } else { 1 };
                    if result != skip {
                        return Some(result);
                    }
                }
            }
        }
        None
    }

    fn value(&self, smudge: bool) -> usize {
        let original = Pattern::check(&self.vertical, &self.horizontal, 0).unwrap();
        if smudge {
            for i in 0..self.horizontal.len() {
                for j in 0..self.vertical.len() {
                    let mut horizontal = self.horizontal.clone();
                    let mut vertical = self.vertical.clone();
                    horizontal[i] ^= 1 << j;
                    vertical[j] ^= 1 << i;
                    if let Some(value) = Pattern::check(&vertical, &horizontal, original) {
                        return value;
                    }
                }
            }
            panic!("Unexpected");
        } else {
            original
        }
    }
}

fn process(input: &str, smudge: bool) -> usize {
    let mut result = 0;
    let mut processed_lines = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            result += Pattern::from(&processed_lines).value(smudge);
            processed_lines = vec![];
        } else {
            let processed_line = line.chars().collect::<Vec<_>>();
            processed_lines.push(processed_line);
        }
    }
    result += Pattern::from(&processed_lines).value(smudge);
    result
}

fn part1(input: &str) -> usize {
    process(input, false)
}

fn part2(input: &str) -> usize {
    process(input, true)
}

fn main() {
    let input = read_input(13);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    ";

    #[test]
    fn test_day13_part1() {
        assert_eq!(part1(INPUT.trim()), 405);
    }

    #[test]
    fn test_day13_part2() {
        assert_eq!(part2(INPUT.trim()), 400);
    }
}

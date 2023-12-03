use std::collections::HashSet;

use adventofcode2023::read_input;

struct Number {
    number: u32,
    x_min: usize,
    x_max: usize,
    y: usize,
}

impl Number {
    fn check(&self, x: usize, y: usize) -> bool {
        let x_min = if self.x_min > 0 {
            self.x_min - 1
        } else {
            self.x_min
        };
        let x_max = self.x_max + 1;
        let y_min = if self.y > 0 { self.y - 1 } else { self.y };
        let y_max = self.y + 1;
        x_min <= x && x_max >= x && y_min <= y && y_max >= y
    }
}

fn parse(input: &str, only_stars: bool) -> (Vec<Number>, HashSet<(usize, usize)>) {
    let mut numbers = vec![];
    let mut symbols = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut current_number = 0;
        let mut number_count = 0;
        for (j, ch) in line.trim().chars().enumerate() {
            if let Some(value) = ch.to_digit(10) {
                number_count += 1;
                current_number *= 10;
                current_number += value;
            } else {
                if number_count > 0 {
                    let number = Number {
                        number: current_number,
                        x_min: j - number_count,
                        x_max: j - 1,
                        y: i,
                    };

                    numbers.push(number);

                    current_number = 0;
                    number_count = 0;
                }

                if ch != '.' && !only_stars || ch == '*' {
                    symbols.insert((j, i));
                }
            }
        }

        if number_count > 0 {
            let number = Number {
                number: current_number,
                x_min: line.len() - number_count - 1,
                x_max: line.len() - 1,
                y: i,
            };

            numbers.push(number);
        }
    }

    (numbers, symbols)
}

fn part1(input: &str) -> u32 {
    let mut result = 0;
    let (numbers, symbols) = parse(input, false);
    for number in numbers {
        for (x, y) in &symbols {
            if number.check(*x, *y) {
                result += number.number;
                break;
            }
        }
    }
    result
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    let (numbers, symbols) = parse(input, true);
    for (x, y) in symbols {
        let mut ratio = 1;
        let mut counter = 0;
        for number in &numbers {
            if number.check(x, y) {
                ratio *= number.number;
                counter += 1;
            }

            if counter > 2 {
                break;
            }
        }
        if counter == 2 {
            result += ratio;
        }
    }
    result
}

fn main() {
    let input = read_input(3);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "
        .trim();

        assert_eq!(part1(&input), 4361);

        let input = "
            12.......*..
            +.........34
            .......-12..
            ..78........
            ..*....60...
            78..........
            .......23...
            ....90*12...
            ............
            2.2......12.
            .*.........*
            1.1.......56
        "
        .trim();

        assert_eq!(part1(&input), 413);
    }

    #[test]
    fn test_day3_part2() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "
        .trim();

        assert_eq!(part2(&input), 467835);

        let input = "
            12.......*..
            +.........34
            .......-12..
            ..78........
            ..*....60...
            78..........
            .......23...
            ....90*12...
            ............
            2.2......12.
            .*.........*
            1.1.......56
        "
        .trim();

        assert_eq!(part2(&input), 6756);
    }
}

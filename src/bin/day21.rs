use core::panic;
use std::collections::{HashMap, HashSet};

use adventofcode2023::read_input;

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let field: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|x| x.trim().chars().collect())
        .collect();
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == 'S' {
                return (field, (j, i));
            }
        }
    }
    panic!("Unexpected");
}

fn check_position(
    positions: &mut HashSet<(usize, usize)>,
    field: &Vec<Vec<char>>,
    x: usize,
    y: usize,
) {
    if field[y][x] == '#' {
        return;
    }

    positions.insert((x, y));
}

fn next_positions(field: &Vec<Vec<char>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut new_positions = HashSet::new();
    if x > 0 {
        check_position(&mut new_positions, field, x - 1, y);
    }
    if y > 0 {
        check_position(&mut new_positions, field, x, y - 1);
    }
    if x < field[0].len() - 1 {
        check_position(&mut new_positions, field, x + 1, y);
    }
    if y < field.len() - 1 {
        check_position(&mut new_positions, field, x, y + 1);
    }
    new_positions
}

fn part1(input: &str, steps: usize) -> usize {
    let (field, (x, y)) = parse(input);
    let mut positions = HashSet::new();
    positions.insert((x, y));
    for _ in 0..steps {
        let mut new_positions = HashSet::new();
        for (x, y) in positions {
            new_positions.extend(next_positions(&field, x, y));
        }
        positions = new_positions;
    }
    positions.len()
}

fn part2(input: &str, steps: usize) -> usize {
    let (field, (x, y)) = parse(input);
    let mut steps_count = HashMap::new();
    let mut positions = HashSet::new();
    positions.insert((x, y));
    let mut step = 0;
    let mut has_new = true;
    while has_new {
        has_new = false;
        let mut new_positions = HashSet::new();
        for (x, y) in positions {
            if !steps_count.contains_key(&(x, y)) {
                steps_count.insert((x, y), step);
                has_new = true;
            }
            new_positions.extend(next_positions(&field, x, y));
        }
        positions = new_positions;
        step += 1;
    }
    let odd = steps_count.values().filter(|x| *x % 2 == 1).count();
    let even = steps_count.values().filter(|x| *x % 2 == 0).count();
    let odd_corners = steps_count
        .values()
        .filter(|x| *x % 2 == 1 && **x > field.len() as i32 / 2)
        .count();
    let even_corners = steps_count
        .values()
        .filter(|x| *x % 2 == 0 && **x > field.len() as i32 / 2)
        .count();
    let n = (steps - (field.len() / 2)) / field.len();
    (n + 1) * (n + 1) * odd + n * n * even - (n + 1) * odd_corners + n * even_corners
}

fn main() {
    let input = read_input(21);

    println!("{}", part1(&input, 64));
    println!("{}", part2(&input, 26501365));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    ";

    #[test]
    fn test_day21_part1() {
        assert_eq!(part1(INPUT, 6), 16);
    }

    #[test]
    fn test_day21_part2() {
        /*assert_eq!(part2(INPUT, 6), 16);
        assert_eq!(part2(INPUT, 10), 50);
        assert_eq!(part2(INPUT, 50), 1594);
        assert_eq!(part2(INPUT, 100), 6536);
        assert_eq!(part2(INPUT, 500), 167004);
        assert_eq!(part2(INPUT, 1000), 668697);
        assert_eq!(part2(INPUT, 5000), 16733044);*/
    }
}

use std::collections::HashMap;

use adventofcode2023::read_input;

fn load(data: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for x in 0..data[0].len() {
        let mut current = data.len();
        for y in 0..data.len() {
            match data[y][x] {
                'O' => {
                    result += current;
                    current -= 1;
                }
                '#' => {
                    current = data.len() - y - 1;
                }
                _ => (),
            }
        }
    }
    result
}

fn calculate(data: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] == 'O' {
                result += data.len() - y;
            }
        }
    }
    result
}

fn part1(input: &str) -> usize {
    let data = input
        .lines()
        .map(|x| x.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    load(&data)
}

fn next_stone(
    x: usize,
    y: usize,
    data: &Vec<Vec<char>>,
    shift: (i32, i32),
    make_step: bool,
) -> (usize, usize) {
    let mut x = x as i32 + if make_step { shift.0 } else { 0 };
    let mut y = y as i32 + if make_step { shift.1 } else { 0 };
    while x >= 0 && x < data[0].len() as i32 && y >= 0 && y < data.len() as i32 {
        if data[y as usize][x as usize] != '#' {
            break;
        }

        x += shift.0;
        y += shift.1;
    }

    (x.max(0) as usize, y.max(0) as usize)
}

fn iterate(data: &mut Vec<Vec<char>>) {
    for x in 0..data[0].len() {
        let mut current = next_stone(x, 0, &data, (0, 1), false).1;
        for y in 0..data.len() {
            match data[y][x] {
                'O' => {
                    let t = data[current][x];
                    data[current][x] = 'O';
                    data[y][x] = t;
                    current = next_stone(x, current, &data, (0, 1), true).1;
                }
                '#' => {
                    current = next_stone(x, y, &data, (0, 1), true).1;
                }
                _ => (),
            }
        }
    }

    for y in 0..data.len() {
        let mut current = next_stone(0, y, &data, (1, 0), false).0;
        for x in 0..data[0].len() {
            match data[y][x] {
                'O' => {
                    let t = data[y][current];
                    data[y][current] = 'O';
                    data[y][x] = t;
                    current = next_stone(current, y, &data, (1, 0), true).0;
                }
                '#' => {
                    current = next_stone(x, y, &data, (1, 0), true).0;
                }
                _ => (),
            }
        }
    }

    for x in 0..data[0].len() {
        let mut current = next_stone(x, data.len() - 1, &data, (0, -1), false).1;
        for y in (0..data.len()).rev() {
            match data[y][x] {
                'O' => {
                    let t = data[current][x];
                    data[current][x] = 'O';
                    data[y][x] = t;
                    current = next_stone(x, current, &data, (0, -1), true).1;
                }
                '#' => {
                    current = next_stone(x, y, &data, (0, -1), true).1;
                }
                _ => (),
            }
        }
    }

    for y in 0..data.len() {
        let mut current = next_stone(data[0].len() - 1, y, &data, (-1, 0), false).0;
        for x in (0..data[0].len()).rev() {
            match data[y][x] {
                'O' => {
                    let t = data[y][current];
                    data[y][current] = 'O';
                    data[y][x] = t;
                    current = next_stone(current, y, &data, (-1, 0), true).0;
                }
                '#' => {
                    current = next_stone(x, y, &data, (-1, 0), true).0;
                }
                _ => (),
            }
        }
    }
}

fn part2(input: &str) -> usize {
    let mut data = input
        .lines()
        .map(|x| x.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut positions: HashMap<String, usize> = HashMap::new();
    for step in 1..=1000000000 {
        iterate(&mut data);
        let mut key = String::new();
        for i in 0..data.len() {
            for j in 0..data[0].len() {
                if data[i][j] == 'O' {
                    key += &format!("{i}_{j}-")[..];
                }
            }
        }
        if let Some(old_steps) = positions.get(&key) {
            let left_steps = (1000000000 - old_steps) % (step - old_steps);
            for _ in 0..left_steps {
                iterate(&mut data);
            }
            return calculate(&data);
        } else {
            positions.insert(key, step);
        }
    }
    calculate(&data)
}

fn main() {
    let input = read_input(14);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    ";

    #[test]
    fn test_day14_part1() {
        assert_eq!(part1(INPUT.trim()), 136);
    }

    #[test]
    fn test_day14_part2() {
        assert_eq!(part2(INPUT.trim()), 64);
    }
}

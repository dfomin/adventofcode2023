use std::collections::HashMap;

use adventofcode2023::read_input;

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
    steps: usize,
    direction: (i32, i32),
    direction_steps: usize,
}

impl Position {
    fn next_positions(
        &self,
        history: &mut HashMap<(usize, usize, i32, i32), Position>,
        field: &Vec<Vec<usize>>,
        record: &mut usize,
        min: usize,
        max: usize,
    ) -> Vec<Position> {
        let x = self.x as i32;
        let y = self.y as i32;
        let mut result = vec![];
        let shifts = [(0, -1), (-1, 0), (0, 1), (1, 0)];
        for shift in shifts {
            if self.direction.0 == -shift.0 && self.direction.1 == -shift.1 {
                continue;
            }
            let multiplier = if shift != self.direction {
                min as i32
            } else {
                1
            };
            let new_x = x + shift.0 * multiplier;
            let new_y = y + shift.1 * multiplier;
            if 0 <= new_x
                && new_x < field[0].len() as i32
                && 0 <= new_y
                && new_y < field.len() as i32
            {
                let mut i = 0;
                let mut new_steps = self.steps;
                while i < multiplier {
                    new_steps +=
                        field[(y + shift.1 * (i + 1)) as usize][(x + shift.0 * (i + 1)) as usize];
                    i += 1;
                }
                let new_position = Position {
                    x: new_x as usize,
                    y: new_y as usize,
                    steps: new_steps,
                    direction: shift,
                    direction_steps: if shift != self.direction {
                        min
                    } else {
                        self.direction_steps + 1
                    },
                };
                if new_position.direction_steps > max {
                    continue;
                }
                let mut add = false;
                if let Some(prev_position) =
                    history.get(&(new_x as usize, new_y as usize, shift.0, shift.1))
                {
                    if prev_position.direction_steps > new_position.direction_steps
                        || prev_position.steps > new_position.steps
                    {
                        add = true;
                    }
                } else {
                    add = true;
                }
                if add {
                    if new_position.x == field[0].len() - 1 && new_position.y == field.len() - 1 {
                        *record = (*record).min(new_position.steps);
                    }
                    history.insert(
                        (new_position.x, new_position.y, shift.0, shift.1),
                        new_position.clone(),
                    );
                    result.push(new_position);
                }
            }
        }
        result
    }
}

fn bfs(
    mut positions: Vec<Position>,
    history: &mut HashMap<(usize, usize, i32, i32), Position>,
    field: &Vec<Vec<usize>>,
    min: usize,
    max: usize,
) -> usize {
    let mut result = usize::MAX;
    let mut index = 0;
    while index < positions.len() {
        let position = positions[index].clone();
        positions.extend(position.next_positions(history, &field, &mut result, min, max));
        index += 1;
    }
    result
}

fn dfs(
    mut positions: Vec<Position>,
    history: &mut HashMap<(usize, usize, i32, i32), Position>,
    field: &Vec<Vec<usize>>,
    min: usize,
    max: usize,
) -> usize {
    let mut result = usize::MAX;
    while !positions.is_empty() {
        let position = positions.pop().unwrap();
        positions.extend(position.next_positions(history, &field, &mut result, min, max));
    }
    result
}

fn process(input: &str, min: usize, max: usize) -> usize {
    let field: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|x| {
            x.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let positions = vec![
        Position {
            x: 0,
            y: 0,
            steps: 0,
            direction: (0, 1),
            direction_steps: 1,
        },
        Position {
            x: 0,
            y: 0,
            steps: 0,
            direction: (1, 0),
            direction_steps: 1,
        },
    ];
    let mut history = HashMap::new();
    for position in &positions {
        history.insert(
            (
                position.x,
                position.y,
                position.direction.0,
                position.direction.1,
            ),
            position.clone(),
        );
    }
    bfs(positions, &mut history, &field, min, max)
    // dfs(positions, &mut history, &field, min, max)
}

fn part1(input: &str) -> usize {
    process(input, 1, 3)
}

fn part2(input: &str) -> usize {
    process(input, 4, 10)
}

fn main() {
    let input = read_input(17);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    ";

    const ANOTHER_INPUT: &str = "
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991
    ";

    #[test]
    fn test_day17_part1() {
        assert_eq!(part1(INPUT), 102);
    }

    #[test]
    fn test_day17_part2() {
        assert_eq!(part2(INPUT), 94);
        assert_eq!(part2(ANOTHER_INPUT), 71);
    }
}

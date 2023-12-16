use std::collections::HashSet;

use adventofcode2023::read_input;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Beam {
    x: usize,
    y: usize,
    direction: (i32, i32),
}

impl Beam {
    fn update(&self, cave: &Vec<Vec<char>>) -> Vec<Beam> {
        let mut result: Vec<Beam> = vec![];
        let width = cave[0].len();
        let height = cave.len();
        match cave[self.y][self.x] {
            '.' => {
                result.push(self.clone());
            }
            '|' => {
                if self.direction.1 != 0 {
                    result.push(self.clone());
                } else {
                    let mut up_beam = self.clone();
                    up_beam.direction = (0, -1);
                    result.push(up_beam);
                    let mut down_beam = self.clone();
                    down_beam.direction = (0, 1);
                    result.push(down_beam);
                }
            }
            '-' => {
                if self.direction.0 != 0 {
                    result.push(self.clone());
                } else {
                    let mut left_beam = self.clone();
                    left_beam.direction = (-1, 0);
                    result.push(left_beam);
                    let mut right_beam = self.clone();
                    right_beam.direction = (1, 0);
                    result.push(right_beam);
                }
            }
            '/' => {
                let mut new_beam = self.clone();
                new_beam.direction = (-self.direction.1, -self.direction.0);
                result.push(new_beam);
            }
            '\\' => {
                let mut new_beam = self.clone();
                new_beam.direction = (self.direction.1, self.direction.0);
                result.push(new_beam);
            }
            _ => panic!("Unexpected"),
        }
        result
            .iter()
            .map(|x| x.step(width, height))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }

    fn step(&self, width: usize, height: usize) -> Option<Beam> {
        let new_x = self.x as i32 + self.direction.0;
        let new_y = self.y as i32 + self.direction.1;
        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            Some(Beam {
                x: new_x as usize,
                y: new_y as usize,
                direction: self.direction,
            })
        } else {
            None
        }
    }
}

fn process(cave: &Vec<Vec<char>>, beam: Beam) -> usize {
    let mut beams = vec![beam];
    let mut history: HashSet<Beam> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let mut has_new_beam = false;
        let mut next_beams = vec![];
        for beam in &beams {
            visited.insert((beam.x, beam.y));
            if !history.contains(beam) {
                has_new_beam = true;
                history.insert(beam.clone());
                next_beams.extend(beam.update(cave));
            }
        }
        if !has_new_beam {
            break;
        }
        beams = next_beams;
    }
    visited.len()
}

fn part1(input: &str) -> usize {
    let cave = input
        .trim()
        .lines()
        .map(|x| x.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    process(
        &cave,
        Beam {
            x: 0,
            y: 0,
            direction: (1, 0),
        },
    )
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    let cave = input
        .trim()
        .lines()
        .map(|x| x.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..cave.len() {
        result = result.max(process(
            &cave,
            Beam {
                x: 0,
                y: i,
                direction: (1, 0),
            },
        ));
        result = result.max(process(
            &cave,
            Beam {
                x: cave[0].len() - 1,
                y: i,
                direction: (-1, 0),
            },
        ));
    }
    for i in 0..cave[0].len() {
        result = result.max(process(
            &cave,
            Beam {
                x: i,
                y: 0,
                direction: (0, 1),
            },
        ));
        result = result.max(process(
            &cave,
            Beam {
                x: i,
                y: cave.len() - 1,
                direction: (0, -1),
            },
        ));
    }

    result
}

fn main() {
    let input = read_input(16);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    ";

    #[test]
    fn test_day16_part1() {
        assert_eq!(part1(INPUT), 46);
    }

    #[test]
    fn test_day16_part2() {
        assert_eq!(part2(INPUT), 51);
    }
}

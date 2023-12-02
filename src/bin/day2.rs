use core::panic;

use adventofcode2023::read_input;

struct GameResult {
    id: i32,
    cubes: Vec<Vec<i32>>,
    minimum: Vec<i32>,
}

impl GameResult {
    fn is_possible(&self, real_cubes: &[i32]) -> bool {
        for cubes in &self.cubes {
            for i in 0..cubes.len() {
                if cubes[i] > real_cubes[i] {
                    return false;
                }
            }
        }

        true
    }

    fn add(&mut self, cubes: Vec<i32>) {
        for i in 0..self.minimum.len() {
            self.minimum[i] = self.minimum[i].max(cubes[i]);
        }
        self.cubes.push(cubes);
    }

    fn factor(&self) -> i32 {
        self.minimum.iter().product::<i32>()
    }
}

fn parse(input: &str) -> Vec<GameResult> {
    let mut result = vec![];
    for line in input.lines() {
        let parts = line.trim().split(": ").collect::<Vec<_>>();
        let id = parts[0].split(' ').collect::<Vec<_>>()[1]
            .parse::<i32>()
            .unwrap();
        let mut game_result = GameResult {
            id,
            cubes: vec![],
            minimum: vec![0; 3],
        };
        for game in parts[1].split("; ") {
            let mut colors = vec![0; 3];
            for cubes in game.split(", ") {
                let values = cubes.split(' ').collect::<Vec<_>>();
                let index = match values[1] {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("Unexpected color"),
                };

                colors[index] = values[0].parse::<i32>().unwrap();
            }

            game_result.add(colors);
        }

        result.push(game_result);
    }

    result
}

fn part1(input: &str, real_cubes: Vec<i32>) -> i32 {
    let mut result = 0;
    for game in parse(input) {
        if game.is_possible(&real_cubes) {
            result += game.id;
        }
    }

    result
}

fn part2(input: &str) -> i32 {
    let mut result = 0;
    for game in parse(input) {
        result += game.factor();
    }
    result
}

fn main() {
    let input = read_input(2);

    println!("{}", part1(&input, vec![12, 13, 14]));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "
        .trim();

        assert_eq!(part1(input, vec![12, 13, 14]), 8);
    }

    #[test]
    fn test_day2_part2() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "
        .trim();

        assert_eq!(part2(input), 2286);
    }
}

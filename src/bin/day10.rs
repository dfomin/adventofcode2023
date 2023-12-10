use adventofcode2023::read_input;

#[derive(PartialEq)]
enum PipeIntersection {
    FromUp,
    FromDown,
    Empty,
}

struct Position {
    x: usize,
    y: usize,
    prev_x: usize,
    prev_y: usize,
}

impl Position {
    fn apply(&mut self, map: &Vec<Vec<char>>) {
        match map[self.y][self.x] {
            '|' => {
                if self.prev_y < self.y {
                    self.y += 1;
                    self.prev_y += 1;
                } else {
                    self.y -= 1;
                    self.prev_y -= 1;
                }
            }
            '-' => {
                if self.prev_x < self.x {
                    self.x += 1;
                    self.prev_x += 1;
                } else {
                    self.x -= 1;
                    self.prev_x -= 1;
                }
            }
            'L' => {
                if self.prev_y < self.y {
                    self.x += 1;
                    self.prev_y += 1;
                } else {
                    self.y -= 1;
                    self.prev_x -= 1;
                }
            }
            'J' => {
                if self.prev_y < self.y {
                    self.x -= 1;
                    self.prev_y += 1;
                } else {
                    self.y -= 1;
                    self.prev_x += 1;
                }
            }
            '7' => {
                if self.prev_x < self.x {
                    self.y += 1;
                    self.prev_x += 1;
                } else {
                    self.x -= 1;
                    self.prev_y -= 1;
                }
            }
            'F' => {
                if self.prev_x > self.x {
                    self.y += 1;
                    self.prev_x -= 1;
                } else {
                    self.x += 1;
                    self.prev_y -= 1;
                }
            }
            _ => {
                panic!("Unexpected pipe configuration")
            }
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, usize, usize) {
    let mut map = vec![];
    let mut start_x = 0;
    let mut start_y = 0;
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        let mut row = vec![];
        for (j, ch) in line.chars().enumerate() {
            row.push(ch);
            if ch == 'S' {
                start_x = j;
                start_y = i;
            }
        }
        map.push(row);
    }
    (map, start_x, start_y)
}

fn get_starting_positions(map: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<Position> {
    let mut positions = vec![];
    if x > 0 && String::from("-FL").contains(map[y][x - 1]) {
        positions.push(Position {
            x: x - 1,
            y,
            prev_x: x,
            prev_y: y,
        });
    }
    if x < map[0].len() - 1 && String::from("-J7").contains(map[y][x + 1]) {
        positions.push(Position {
            x: x + 1,
            y,
            prev_x: x,
            prev_y: y,
        });
    }
    if y > 0 && String::from("|F7").contains(map[y - 1][x]) {
        positions.push(Position {
            x,
            y: y - 1,
            prev_x: x,
            prev_y: y,
        });
    }
    if y < map.len() - 1 && String::from("|LJ").contains(map[y + 1][x]) {
        positions.push(Position {
            x,
            y: y + 1,
            prev_x: x,
            prev_y: y,
        });
    }
    positions
}

fn dfs(pipe: &mut Vec<Vec<usize>>, x: usize, y: usize) {
    if pipe[y][x] != 0 {
        return;
    }
    pipe[y][x] = 2;
    if x > 0 {
        dfs(pipe, x - 1, y);
    }
    if x < pipe[0].len() - 1 {
        dfs(pipe, x + 1, y);
    }
    if y > 0 {
        dfs(pipe, x, y - 1);
    }
    if y < pipe.len() - 1 {
        dfs(pipe, x, y + 1);
    }
}

fn part1(input: &str) -> i32 {
    let mut steps = 1;
    let (map, x, y) = parse(input);
    let mut positions = get_starting_positions(&map, x, y);
    loop {
        for i in 0..positions.len() {
            positions[i].apply(&map);
        }
        steps += 1;
        if positions[0].x == positions[1].x && positions[0].y == positions[1].y {
            return steps;
        }
    }
}

fn part2(input: &str) -> i32 {
    let (map, x, y) = parse(input);
    let mut positions = get_starting_positions(&map, x, y);
    let mut pipe = vec![vec![0; map[0].len()]; map.len()];
    pipe[y][x] = 1;
    for i in 0..positions.len() {
        pipe[positions[i].y][positions[i].x] = 1;
    }
    loop {
        for i in 0..positions.len() {
            positions[i].apply(&map);
            pipe[positions[i].y][positions[i].x] = 1;
        }
        if positions[0].x == positions[1].x && positions[0].y == positions[1].y {
            break;
        }
    }
    for i in 0..map.len() {
        dfs(&mut pipe, 0, i);
        dfs(&mut pipe, map[0].len() - 1, i);
    }
    for i in 0..map[0].len() {
        dfs(&mut pipe, i, 0);
        dfs(&mut pipe, i, map.len() - 1);
    }
    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if pipe[i][j] == 0 {
                let mut count_pipes = 0;
                let mut state = PipeIntersection::Empty;
                for k in 0..j {
                    if String::from("|LJ7F").contains(map[i][k]) && pipe[i][k] == 1 {
                        if state == PipeIntersection::Empty {
                            if map[i][k] == '|' {
                                count_pipes += 1;
                            } else if map[i][k] == 'L' {
                                state = PipeIntersection::FromUp;
                            } else if map[i][k] == 'F' {
                                state = PipeIntersection::FromDown;
                            }
                        } else if state == PipeIntersection::FromUp {
                            if String::from("7F").contains(map[i][k]) {
                                count_pipes += 1;
                                state = PipeIntersection::Empty;
                            } else if String::from("LJ").contains(map[i][k]) {
                                state = PipeIntersection::Empty;
                            }
                        } else if state == PipeIntersection::FromDown {
                            if String::from("LJ").contains(map[i][k]) {
                                count_pipes += 1;
                                state = PipeIntersection::Empty;
                            } else if String::from("7F").contains(map[i][k]) {
                                state = PipeIntersection::Empty;
                            }
                        }
                    }
                }
                if count_pipes % 2 == 1 {
                    result += 1;
                }
            }
        }
    }
    result
}

fn main() {
    let input = read_input(10);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_part1() {
        let mut input = "
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
        "
        .trim();

        assert_eq!(part1(input), 4);

        input = "
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
        "
        .trim();

        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_day10_part2() {
        let mut input = "
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "
        .trim();

        assert_eq!(part2(input), 4);

        input = "
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "
        .trim();

        assert_eq!(part2(input), 8);

        input = "
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "
        .trim();

        assert_eq!(part2(input), 10);
    }
}

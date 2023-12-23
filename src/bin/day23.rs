use core::panic;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use adventofcode2023::read_input;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Position {
    fn neighbors(&self, field: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>) -> Vec<Position> {
        let shifts = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut result = vec![];
        for shift in shifts {
            let new_x = self.x as i32 + shift.0;
            let new_y = self.y as i32 + shift.1;
            if new_x >= 0
                && new_x < field[0].len() as i32
                && new_y >= 0
                && new_y < field.len() as i32
            {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if field[new_y][new_x] != '#' && !visited[new_y][new_x] {
                    result.push(Position { x: new_x, y: new_y });
                }
            }
        }
        result
    }

    fn slope(&self, slope: char) -> Position {
        match slope {
            '>' => Position {
                x: self.x + 1,
                y: self.y,
            },
            '<' => Position {
                x: self.x - 1,
                y: self.y,
            },
            'v' => Position {
                x: self.x,
                y: self.y + 1,
            },
            '^' => Position {
                x: self.x,
                y: self.y - 1,
            },
            _ => panic!("Unexpected"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Vertex {
    position: Position,
    targets: Vec<(Position, usize)>,
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|x| x.trim().chars().collect())
        .collect()
}

fn dfs(
    field: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    position: &Position,
    current_result: usize,
    ignore_slops: bool,
) -> Option<usize> {
    if position.x == field[0].len() - 2 && position.y == field.len() - 1 {
        return Some(current_result);
    }
    visited[position.y][position.x] = true;
    let mut result: Option<usize> = None;
    if "<>^v".contains(field[position.y][position.x]) && !ignore_slops {
        let next_position = position.slope(field[position.y][position.x]);
        if visited[next_position.y][next_position.x] {
            result = None;
        } else {
            result = dfs(
                field,
                visited,
                &next_position,
                current_result + 1,
                ignore_slops,
            );
        }
    } else {
        for neighbor in position.neighbors(field, visited) {
            if let Some(value) = dfs(field, visited, &neighbor, current_result + 1, ignore_slops) {
                if result.is_some() {
                    result = Some(result.unwrap().max(value));
                } else {
                    result = Some(value);
                }
            }
        }
    }
    visited[position.y][position.x] = false;
    result
}

fn find_edges(
    position: &Position,
    visited: &mut Vec<Vec<bool>>,
    vertices: &HashMap<Position, Vertex>,
    field: &Vec<Vec<char>>,
    length: usize,
) -> (Position, usize) {
    if vertices.contains_key(&position) {
        return (position.clone(), length);
    }
    visited[position.y][position.x] = true;
    for neighbor in position.neighbors(field, visited) {
        return find_edges(&neighbor, visited, vertices, field, length + 1);
    }
    panic!("Unexpected");
}

fn build_graph(field: &Vec<Vec<char>>) -> (HashMap<Position, Vertex>, Vec<usize>) {
    let mut vertices = HashMap::new();
    vertices.insert(
        Position { x: 1, y: 0 },
        Vertex {
            position: Position { x: 1, y: 0 },
            targets: vec![],
        },
    );
    vertices.insert(
        Position {
            x: field.len() - 2,
            y: field.len() - 1,
        },
        Vertex {
            position: Position {
                x: field.len() - 2,
                y: field.len() - 1,
            },
            targets: vec![],
        },
    );
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if i == 0 || i == field.len() - 1 {
                continue;
            }
            if field[i][j] != '#' {
                let mut sharps = 0;
                if field[i - 1][j] == '#' {
                    sharps += 1;
                }
                if field[i + 1][j] == '#' {
                    sharps += 1;
                }
                if field[i][j - 1] == '#' {
                    sharps += 1;
                }
                if field[i][j + 1] == '#' {
                    sharps += 1;
                }
                if sharps < 2 {
                    let position = Position { x: j, y: i };
                    let vertex = Vertex {
                        position: position.clone(),
                        targets: vec![],
                    };
                    vertices.insert(position, vertex);
                }
            }
        }
    }
    let positions: Vec<Position> = vertices.keys().map(|x| x.clone()).collect();
    let mut edges = vec![];
    for position in positions {
        let mut visited = vec![vec![false; field[0].len()]; field.len()];
        visited[position.y][position.x] = true;
        for neighbor in position.neighbors(field, &visited) {
            let (target, length) = find_edges(&neighbor, &mut visited, &vertices, &field, 1);
            edges.push(length);
            vertices
                .get_mut(&position)
                .unwrap()
                .targets
                .push((target.clone(), length));
        }
    }
    edges.sort();
    let mut edge_iterator = edges.iter().rev();
    let mut edges_lengths = vec![];
    let mut cumulative_length = 0;
    for _ in 0..vertices.len() {
        cumulative_length += edge_iterator.next().unwrap();
        edge_iterator.next();
        edges_lengths.push(cumulative_length);
    }
    (vertices, edges_lengths)
}

fn process(input: &str, ignore_slops: bool) -> usize {
    let field = parse(input);
    let mut visited = vec![vec![false; field[0].len()]; field.len()];
    let position = Position { x: 1, y: 0 };
    dfs(&field, &mut visited, &position, 0, ignore_slops).unwrap()
}

fn process_graph(input: &str) -> usize {
    let field = parse(input);
    let (vertices, edge_lengths) = build_graph(&field);
    let position = Position { x: 1, y: 0 };
    let mut visited = HashSet::new();
    visited.insert(position.clone());
    let mut record = 0;
    dfs_graph(
        &field,
        &position,
        &vertices,
        &mut visited,
        &edge_lengths,
        0,
        0,
        &mut record,
    );
    record
}

fn dfs_graph(
    field: &Vec<Vec<char>>,
    position: &Position,
    vertices: &HashMap<Position, Vertex>,
    visited: &mut HashSet<Position>,
    edge_lengths: &Vec<usize>,
    current_length: usize,
    edges_count: usize,
    record: &mut usize,
) {
    if position.x == field[0].len() - 2 && position.y == field.len() - 1 {
        *record = (*record).max(current_length);
        return;
    }
    if *record > current_length + edge_lengths[vertices.len() - 2 - edges_count] {
        return;
    }
    let vertex = &vertices[position];
    for target in &vertex.targets {
        if visited.contains(&target.0) {
            continue;
        }
        visited.insert(target.0.clone());
        dfs_graph(
            field,
            &target.0,
            vertices,
            visited,
            edge_lengths,
            current_length + target.1,
            edges_count + 1,
            record,
        );
        visited.remove(&target.0);
    }
}

fn part1(input: &str) -> usize {
    process(input, false)
}

fn part2(input: &str) -> usize {
    process_graph(input)
}

fn main() {
    let input = read_input(23);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    ";

    #[test]
    fn test_day22_part1() {
        assert_eq!(part1(INPUT), 94);
    }

    #[test]
    fn test_day22_part2() {
        assert_eq!(part2(INPUT), 154);
    }
}

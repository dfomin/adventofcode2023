use rand::Rng;
use std::collections::HashMap;

use adventofcode2023::read_input;

fn parse(input: &str) -> (HashMap<String, usize>, HashMap<String, Vec<String>>) {
    let mut vertices = HashMap::new();
    let mut edges = vec![];
    for line in input.trim().lines() {
        let mut iter = line.trim().split(": ");
        let source = iter.next().unwrap();
        vertices.insert(source.to_string(), 1);
        for target in iter.next().unwrap().split_whitespace() {
            vertices.insert(target.to_string(), 1);
            edges.push((source, target));
        }
    }
    let mut edge_map = HashMap::new();
    for vertex in vertices.keys() {
        edge_map.insert(vertex.clone(), Vec::new());
    }
    for edge in edges {
        edge_map.get_mut(edge.0).unwrap().push(edge.1.to_string());
        edge_map.get_mut(edge.1).unwrap().push(edge.0.to_string());
    }
    (vertices, edge_map)
}

fn merge(
    vertices: &mut HashMap<String, usize>,
    edges: &mut HashMap<String, Vec<String>>,
) -> Option<usize> {
    if vertices.len() == 2 {
        if edges.values().next().unwrap().len() == 3 {
            return Some(vertices.values().product());
        } else {
            return None;
        }
    }

    let i = rand::thread_rng().gen_range(0..edges.len());
    let first = edges.keys().skip(i).next().unwrap().clone();
    let j = rand::thread_rng().gen_range(0..edges[&first].len());
    let second = edges[&first][j].clone();
    let value = vertices.remove(&second).unwrap();
    *vertices.get_mut(&first).unwrap() += value;
    let to_update = edges.remove(&second).unwrap();
    for v in to_update {
        edges.get_mut(&v).unwrap().retain(|x| x != &second);
        if v == first {
            continue;
        }
        edges.get_mut(&v).unwrap().push(first.clone());
        edges.get_mut(&first).unwrap().push(v);
    }
    merge(vertices, edges)
}

fn part1(input: &str) -> usize {
    let (vertices, edges) = parse(input);
    loop {
        let mut vertices = vertices.clone();
        let mut edges = edges.clone();
        if let Some(value) = merge(&mut vertices, &mut edges) {
            return value;
        }
    }
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let input = read_input(25);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    ";

    #[test]
    fn test_day25_part1() {
        assert_eq!(part1(INPUT), 54);
    }

    #[test]
    fn test_day25_part2() {}
}

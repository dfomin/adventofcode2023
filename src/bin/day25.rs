use rand::Rng;
use std::collections::{HashMap, HashSet};

use adventofcode2023::read_input;

fn parse(
    input: &str,
    skips: Vec<(String, String)>,
) -> (HashMap<String, usize>, HashMap<String, Vec<String>>) {
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
        let mut is_skip = false;
        for skip in &skips {
            if skip.0 == edge.0 && skip.1 == edge.1 || skip.0 == edge.1 && skip.1 == edge.0 {
                is_skip = true;
            }
        }
        if is_skip {
            continue;
        }
        edge_map.get_mut(edge.0).unwrap().push(edge.1.to_string());
        edge_map.get_mut(edge.1).unwrap().push(edge.0.to_string());
    }
    (vertices, edge_map)
}

fn dfs(visited: &mut HashSet<String>, edges: &HashMap<String, Vec<String>>, vertex: String) {
    if visited.contains(&vertex) {
        return;
    }

    visited.insert(vertex.clone());
    for target in &edges[&vertex] {
        dfs(visited, edges, target.to_string());
    }
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

    let i = rand::thread_rng().gen_range(0..vertices.len());
    let mut j = i;
    while j == i {
        j = rand::thread_rng().gen_range(0..vertices.len());
    }

    let first = vertices.keys().skip(i).next().unwrap().clone();
    let second = vertices.keys().skip(j).next().unwrap().clone();
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
    let (vertices, edges) = parse(
        input,
        vec![
            ("kfr".to_string(), "vkp".to_string()),
            ("bff".to_string(), "rhk".to_string()),
            ("qpp".to_string(), "vnm".to_string()),
        ],
    );
    // loop {
    //     let mut vertices = vertices.clone();
    //     let mut edges = edges.clone();
    //     if let Some(value) = merge(&mut vertices, &mut edges) {
    //         return value;
    //     }
    // }
    let mut visited = HashSet::new();
    dfs(
        &mut visited,
        &edges,
        vertices.keys().next().unwrap().to_string(),
    );
    visited.len() * (vertices.len() - visited.len())
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

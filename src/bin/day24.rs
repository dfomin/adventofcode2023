use core::panic;

use adventofcode2023::read_input;

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from(input: &str, min: f64, scale: f64) -> Self {
        let mut iter = input.split(" @ ");
        let mut coords_iter = iter
            .next()
            .unwrap()
            .split(", ")
            .map(|x| (x.trim().parse::<f64>().unwrap() - min) / scale);
        let mut speed_iter = iter
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.trim().parse::<f64>().unwrap());
        let start = Point {
            x: coords_iter.next().unwrap(),
            y: coords_iter.next().unwrap(),
            z: coords_iter.next().unwrap(),
        };
        let end = Point {
            x: speed_iter.next().unwrap() + start.x,
            y: speed_iter.next().unwrap() + start.y,
            z: speed_iter.next().unwrap() + start.z,
        };
        Self { start, end }
    }

    fn fix(&self, i: f64, j: f64, k: f64) -> Self {
        let dx = (self.end.x - self.start.x) - i;
        let dy = (self.end.y - self.start.y) - j;
        let dz = (self.end.z - self.start.z) - k;
        Self {
            start: self.start.clone(),
            end: Point {
                x: self.start.x + dx,
                y: self.start.y + dy,
                z: self.start.z + dz,
            },
        }
    }
}

fn intersection(line1: &Line, line2: &Line) -> Option<Point> {
    let x1 = line1.start.x;
    let x2 = line1.end.x;
    let y1 = line1.start.y;
    let y2 = line1.end.y;
    let x3 = line2.start.x;
    let x4 = line2.end.x;
    let y3 = line2.start.y;
    let y4 = line2.end.y;
    if ((y2 - y1) * (x4 - x3) - (y4 - y3) * (x2 - x1)).abs() < f64::EPSILON {
        return None;
    }
    let m1 = (y2 - y1) / (x2 - x1);
    let m2 = (y4 - y3) / (x4 - x3);
    let x = (m1 * x1 - m2 * x3 + y3 - y1) / (m1 - m2);
    let y = y1 + m1 * (x - x1);
    Some(Point { x, y, z: 0.0 })
}

fn intersection3d(line1: &Line, line2: &Line, ignore_z: bool) -> Option<Point> {
    let x1 = line1.start.x;
    let x2 = line1.end.x;
    let x3 = line2.start.x;
    let x4 = line2.end.x;
    let y1 = line1.start.y;
    let y2 = line1.end.y;
    let y3 = line2.start.y;
    let y4 = line2.end.y;
    let z1 = line1.start.z;
    let z2 = line1.end.z;
    let z3 = line2.start.z;
    let z4 = line2.end.z;
    if ((y2 - y1) * (x4 - x3) - (y4 - y3) * (x2 - x1)).abs() < 0.1 {
        return None;
    }
    let m1 = (y2 - y1) / (x2 - x1);
    let m2 = (y4 - y3) / (x4 - x3);
    let x = (m1 * x1 - m2 * x3 + y3 - y1) / (m1 - m2);
    let y = y1 + m1 * (x - x1);
    if ignore_z {
        Some(Point { x, y, z: 0.0 })
    } else {
        let z = z1 + (z2 - z1) * (x - x1) / (x2 - x1);
        if (z - (z3 + (z4 - z3) * (x - x3) / (x4 - x3))).abs() < 0.1 {
            Some(Point { x, y, z })
        } else {
            None
        }
    }
}

fn test_lines(fixed_lines: &Vec<Line>, ignore_z: bool) -> Option<Point> {
    if let Some(point1) = intersection3d(&fixed_lines[0], &fixed_lines[1], ignore_z) {
        if let Some(point2) = intersection3d(&fixed_lines[0], &fixed_lines[2], ignore_z) {
            if (point1.x - point2.x).abs()
                + (point1.y - point2.y).abs()
                + if ignore_z {
                    0.0
                } else {
                    (point1.z - point2.z).abs()
                }
                < 0.1
            {
                return Some(point1);
            }
        }
    }
    None
}

fn part1(input: &str, low: f64, high: f64) -> usize {
    let min = (low + high) / 2.0;
    let lines = input
        .trim()
        .lines()
        .map(|x| Line::from(x.trim(), min, 1.0))
        .collect::<Vec<_>>();
    let mut result = 0;
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if let Some(point) = intersection(&lines[i], &lines[j]) {
                let x = point.x + min;
                let y = point.y + min;
                if x >= low
                    && x <= high
                    && y >= low
                    && y <= high
                    && ((point.x > lines[i].end.x) == (lines[i].end.x > lines[i].start.x))
                    && ((point.x > lines[j].end.x) == (lines[j].end.x > lines[j].start.x))
                {
                    result += 1;
                }
            }
        }
    }
    result
}

fn find_indices(input: &str, scale: f64) -> Vec<(i64, i64, i64)> {
    let lines = input
        .trim()
        .lines()
        .take(3)
        .map(|x| Line::from(x.trim(), 0.0, scale))
        .collect::<Vec<_>>();
    let mut result = vec![];
    for i in -1000..1000 {
        for j in -1000..1000 {
            let fixed_lines = lines
                .iter()
                .map(|x| x.fix(i as f64, j as f64, 0 as f64))
                .collect::<Vec<_>>();
            if test_lines(&fixed_lines, true).is_some() {
                for k in -1000..1000 {
                    let fixed_lines_z = lines
                        .iter()
                        .map(|x| x.fix(i as f64, j as f64, k as f64))
                        .collect::<Vec<_>>();
                    if let Some(point) = test_lines(&fixed_lines_z, false) {
                        result.push((i, j, k));
                    }
                }
            }
        }
    }
    result
}

fn part2(input: &str, scale: f64) -> i64 {
    let lines = input
        .trim()
        .lines()
        .take(3)
        .map(|x| Line::from(x.trim(), 0.0, 1.0))
        .collect::<Vec<_>>();
    for (i, j, k) in find_indices(input, scale) {
        let fixed_lines = lines
            .iter()
            .map(|x| x.fix(i as f64, j as f64, k as f64))
            .collect::<Vec<_>>();
        if let Some(point) = intersection3d(&fixed_lines[0], &fixed_lines[1], false) {
            return (point.x.round() + point.y.round() + point.z.round()) as i64;
        }
    }
    panic!("Unexpected")
}

fn main() {
    let input = read_input(24);

    println!("{}", part1(&input, 200000000000000.0, 400000000000000.0));
    println!("{}", part2(&input, 1_000_000_000_000.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    ";

    #[test]
    fn test_day24_part1() {
        assert_eq!(part1(INPUT, 7.0, 27.0), 2);
    }

    #[test]
    fn test_day24_part2() {
        assert_eq!(part2(INPUT, 1.0), 47);
    }
}

use adventofcode2023::read_input;

fn field_size(lines: &Vec<(char, usize)>) -> (usize, usize, usize, usize) {
    let (mut x_min, mut y_min, mut x_max, mut y_max, mut x, mut y) = (0, 0, 0, 0, 0, 0);
    for (direction, step) in lines {
        match direction {
            'R' => x += *step as i32,
            'L' => x -= *step as i32,
            'U' => y -= *step as i32,
            'D' => y += *step as i32,
            _ => panic!("Unexpected"),
        }
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }
    (
        (x_max - x_min + 1) as usize,
        (y_max - y_min + 1) as usize,
        (-x_min) as usize,
        (-y_min) as usize,
    )
}

fn dfs(field: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    if field[y][x] != 0 {
        return;
    }
    field[y][x] = 2;
    if x > 0 {
        dfs(field, x - 1, y);
    }
    if y > 0 {
        dfs(field, x, y - 1);
    }
    if x < field[0].len() - 1 {
        dfs(field, x + 1, y);
    }
    if y < field.len() - 1 {
        dfs(field, x, y + 1);
    }
}

fn process(lines: &Vec<(char, usize)>) -> usize {
    let (width, height, mut x, mut y) = field_size(&lines);
    let mut field = vec![vec![0; width]; height];
    for (direction, step) in lines {
        let (mut next_x, mut next_y) = (x, y);
        match direction {
            'R' => next_x = x + step,
            'L' => next_x = x - step,
            'U' => next_y = y - step,
            'D' => next_y = y + step,
            _ => panic!("Unexpected"),
        }
        while x != next_x || y != next_y {
            field[y][x] = 1;

            if next_x > x {
                x += 1;
            } else if next_x < x {
                x -= 1;
            } else if next_y > y {
                y += 1;
            } else if next_y < y {
                y -= 1;
            }
        }
    }
    for i in 0..field.len() {
        dfs(&mut field, 0, i);
        dfs(&mut field, width - 1, i);
    }
    for i in 0..field[0].len() {
        dfs(&mut field, i, 0);
        dfs(&mut field, i, height - 1);
    }
    for line in &field {
        println!(
            "{}",
            line.iter()
                .map(|x| if *x == 1 { '#' } else { '.' })
                .collect::<String>()
        );
    }
    field
        .into_iter()
        .map(|x| x.into_iter().filter(|y| *y != 2).count())
        .sum()
}

fn part1(input: &str) -> usize {
    let mut lines = vec![];
    for line in input.trim().lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        lines.push((
            parts[0].chars().next().unwrap(),
            parts[1].parse::<usize>().unwrap(),
        ));
    }
    process(&lines)
}

fn part2(input: &str) -> usize {
    let mut lines = vec![];
    for line in input.trim().lines() {
        let mut color = line.split_whitespace().collect::<Vec<_>>()[2];
        color = &color[2..color.len() - 1];
        let direction = match color.chars().last().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!("Unexpected"),
        };
        let steps = usize::from_str_radix(&color[1..color.len() - 1], 16).unwrap();
        lines.push((direction, steps));
    }
    process(&lines)
}

fn main() {
    let input = read_input(18);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    ";

    #[test]
    fn test_day18_part1() {
        assert_eq!(part1(INPUT), 62);
    }

    #[test]
    fn test_day18_part2() {
        assert_eq!(part2(INPUT), 952408144115);
    }
}

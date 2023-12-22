use adventofcode2023::read_input;

#[derive(Debug, Clone)]
struct Brick {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

impl Brick {
    fn fall(&mut self, heights: &mut Vec<Vec<usize>>) -> usize {
        let mut height = usize::MAX;
        for x in self.x.0..=self.x.1 {
            for y in self.y.0..=self.y.1 {
                height = height.min(self.z.0 - heights[y][x]);
            }
        }
        height -= 1;
        self.z.0 -= height;
        self.z.1 -= height;
        for x in self.x.0..=self.x.1 {
            for y in self.y.0..=self.y.1 {
                heights[y][x] = self.z.1;
            }
        }
        height
    }
}

fn parse(input: &str) -> (Vec<Brick>, Vec<Vec<usize>>) {
    let mut bricks = vec![];
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input.trim().lines() {
        let line = line.trim();
        let mut iter = line.split('~');
        let mut start = iter.next().unwrap().split(',').map(|x| x.parse().unwrap());
        let mut end = iter.next().unwrap().split(',').map(|x| x.parse().unwrap());
        let brick = Brick {
            x: (start.next().unwrap(), end.next().unwrap()),
            y: (start.next().unwrap(), end.next().unwrap()),
            z: (start.next().unwrap(), end.next().unwrap()),
        };
        max_x = max_x.max(brick.x.1);
        max_y = max_y.max(brick.y.1);
        bricks.push(brick);
    }
    bricks.sort_by_key(|x| x.z.0);
    (bricks, vec![vec![0; max_x + 1]; max_y + 1])
}

fn process(input: &str) -> (usize, usize) {
    let (mut bricks, mut heights) = parse(input);
    let orig_bricks = bricks.clone();
    let mut brick_heights = vec![];
    for i in 0..bricks.len() {
        brick_heights.push(bricks[i].fall(&mut heights));
    }
    let mut safe_bricks = 0;
    let mut will_fall_bricks = 0;
    for i in 0..bricks.len() {
        let mut new_bricks = orig_bricks.clone();
        let mut new_heights = vec![vec![0; heights[0].len()]; heights.len()];
        let mut will_fall = 0;
        for j in 0..new_bricks.len() {
            if i == j {
                continue;
            }
            if new_bricks[j].fall(&mut new_heights) != brick_heights[j] {
                will_fall += 1;
            }
        }
        if will_fall == 0 {
            safe_bricks += 1;
        }
        will_fall_bricks += will_fall;
    }
    (safe_bricks, will_fall_bricks)
}

fn part1(input: &str) -> usize {
    process(input).0
}

fn part2(input: &str) -> usize {
    process(input).1
}

fn main() {
    let input = read_input(22);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    ";

    #[test]
    fn test_day22_part1() {
        assert_eq!(part1(INPUT), 5);
    }

    #[test]
    fn test_day22_part2() {
        assert_eq!(part2(INPUT), 7);
    }
}

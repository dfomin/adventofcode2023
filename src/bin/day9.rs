use adventofcode2023::read_input;

fn restore(numbers: &mut Vec<i32>, start_index: usize, is_opposite: bool) {
    for i in (1..=start_index).rev() {
        for j in i..numbers.len() {
            if !is_opposite {
                numbers[j] += numbers[j - 1];
            } else {
                numbers[j] = numbers[j - 1] - numbers[j];
            }
        }
    }
}

fn step(numbers: &mut Vec<i32>, start_index: usize) -> bool {
    let mut all_same = true;
    for i in (start_index + 1..numbers.len()).rev() {
        numbers[i] -= numbers[i - 1];
        if numbers[i] != *numbers.last().unwrap() {
            all_same = false;
        }
    }
    all_same
}

fn process(numbers: &mut Vec<i32>) -> i32 {
    let mut start_index = 0;
    while !step(numbers, start_index) {
        start_index += 1;
    }
    numbers.push(*numbers.last().unwrap());
    restore(numbers, start_index + 1, false);
    *numbers.last().unwrap()
}

fn part1(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut numbers = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        result += process(&mut numbers);
    }
    result
}

fn part2(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut numbers = line
            .trim()
            .split_whitespace()
            .rev()
            .map(|x| x.parse().unwrap())
            .collect();
        result += process(&mut numbers);
    }
    result
}

fn main() {
    let input = read_input(9);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore() {
        let mut input = vec![0, 3, 3, 3, 3, 3, 3];
        restore(&mut input, 1, false);
        assert_eq!(input, vec![0, 3, 6, 9, 12, 15, 18]);

        input = vec![1, 2, 1, 1, 1, 1, 1];
        restore(&mut input, 2, false);
        assert_eq!(input, vec![1, 3, 6, 10, 15, 21, 28]);

        input = vec![10, 3, 0, 2, 2, 2, 2];
        restore(&mut input, 3, false);
        assert_eq!(input, vec![10, 13, 16, 21, 30, 45, 68]);

        input = vec![45, 15, 6, 2, 2, 2, 2];
        restore(&mut input, 3, true);
        assert_eq!(input, vec![45, 30, 21, 16, 13, 10, 5]);
    }

    #[test]
    fn test_step() {
        let mut input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(step(&mut input, 0), false);
        assert_eq!(input, vec![10, 3, 3, 5, 9, 15]);

        input = vec![10, 3, 0, 2, 4, 6];
        assert_eq!(step(&mut input, 2), true);
        assert_eq!(input, vec![10, 3, 0, 2, 2, 2]);
    }

    #[test]
    fn test_day9_part1() {
        let input = "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "
        .trim();

        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_day9_part2() {
        let input = "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "
        .trim();

        assert_eq!(part2(input), 2);
    }
}

use adventofcode2023::read_input;

fn parse(input: &str, remove_spaces: bool) -> Vec<Vec<i64>> {
    let mut result = vec![];
    for line in input.lines() {
        let mut line = line.trim();
        line = line.split(':').collect::<Vec<_>>()[1];
        let replacement = if remove_spaces { "" } else { " " };
        result.push(
            line.replace(" ", replacement)
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect(),
        );
    }
    result
}

fn process(input: &str, remove_spaces: bool) -> i64 {
    let mut result = 1;
    let values = parse(input, remove_spaces);
    for i in 0..values[0].len() {
        let n = values[0][i];
        let m = values[1][i];
        let d = ((n * n - 4 * m) as f64).sqrt() as i64;
        let k = (n - d) / 2;
        for j in k..n {
            if j * n - j * j > m {
                result *= n - 2 * (j - 1) - 1;
                break;
            }
        }
    }
    result
}

fn part1(input: &str) -> i64 {
    process(input, false)
}

fn part2(input: &str) -> i64 {
    process(input, true)
}

fn main() {
    let input = read_input(6);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        let input = "
            Time:      7  15   30
            Distance:  9  40  200
        "
        .trim();

        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_day6_part2() {
        let input = "
            Time:      7  15   30
            Distance:  9  40  200
        "
        .trim();

        assert_eq!(part2(input), 71503);
    }
}

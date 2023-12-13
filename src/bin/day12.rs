use core::panic;

use adventofcode2023::read_input;

fn can_consume(plan: &Vec<char>, value: usize, position: usize) -> bool {
    if value > position + 1 {
        return false;
    }

    for i in position + 1 - value..=position {
        if plan[i] == '.' {
            return false;
        }
    }

    if position > value {
        if plan[position - value] == '#' {
            return false;
        }
    }

    true
}

fn process_line(plan: &str, springs: &Vec<usize>) -> usize {
    let mut plan = plan.chars().collect::<Vec<_>>();
    plan.insert(0, '.');
    let mut dp = vec![vec![0; plan.len() + 1]; springs.len() + 1];
    for i in 0..dp[0].len() {
        if i > 0 && plan[i - 1] == '#' {
            break;
        }
        dp[0][i] = 1;
    }
    for i in 1..=springs.len() {
        let spring = springs[i - 1];
        for j in 1..dp[i].len() {
            match plan[j - 1] {
                '.' => dp[i][j] = dp[i][j - 1],
                '#' => {
                    if can_consume(&plan, spring, j - 1) {
                        dp[i][j] = dp[i - 1][j - spring - 1];
                    } else {
                        dp[i][j] = 0;
                    }
                }
                '?' => {
                    if can_consume(&plan, spring, j - 1) {
                        dp[i][j] = dp[i - 1][j - spring - 1] + dp[i][j - 1];
                    } else {
                        dp[i][j] = dp[i][j - 1];
                    }
                }
                _ => panic!("Unexpected symbol"),
            }
        }
    }
    dp[springs.len()][plan.len()]
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        result += process_line(
            parts[0],
            &parts[1].split(',').map(|x| x.parse().unwrap()).collect(),
        );
    }
    result
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let plan = vec![parts[0]; 5];
        let springs = parts[1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        let len = springs.len();
        result += process_line(
            &plan.join("?")[..],
            &springs.into_iter().cycle().take(5 * len).collect(),
        );
    }
    result
}

fn main() {
    let input = read_input(12);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1() {
        assert_eq!(process_line("???.###", &vec![1, 1, 3]), 1);
        assert_eq!(process_line(".??..??...?##.", &vec![1, 1, 3]), 4);
        assert_eq!(process_line("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6]), 1);
        assert_eq!(process_line("????.#...#...", &vec![4, 1, 1]), 1);
        assert_eq!(process_line("????.######..#####.", &vec![1, 6, 5]), 4);
        assert_eq!(process_line("?###????????", &vec![3, 2, 1]), 10);
        assert_eq!(process_line("?????#?????????.??", &vec![1, 4, 3]), 41);
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(part2("?###???????? 3,2,1"), 506250);
    }
}

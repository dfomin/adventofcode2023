use adventofcode2023::read_input;

fn process(input: &String, letters: bool) -> i32 {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut result = 0;
    for line in input.lines() {
        let mut first = -1;
        let mut last = -1;
        for (i, ch) in line.chars().enumerate() {
            let mut number = -1;
            if let Some(unumber) = ch.to_digit(10) {
                number = unumber as i32;
            } else if letters {
                for (j, &word) in words.iter().enumerate() {
                    if i + word.as_bytes().len() <= line.len()
                        && &line[i..i + word.as_bytes().len()] == word
                    {
                        number = j as i32 + 1;
                        break;
                    }
                }
            }

            if number != -1 {
                if first == -1 {
                    first = number as i32;
                }

                last = number as i32;
            }
        }

        result += 10 * first + last;
    }

    result
}

fn part1(input: &String) -> i32 {
    process(input, false)
}

fn part2(input: &String) -> i32 {
    process(input, true)
}

fn main() {
    let input = read_input(1);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part1(&input.to_string()), 142);
    }

    #[test]
    fn test_day1_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(&input.to_string()), 281);
    }
}

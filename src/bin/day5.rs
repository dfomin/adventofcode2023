use adventofcode2023::read_input;

struct Range {
    low: i64,
    high: i64,
    change: i64,
}

impl Range {
    fn check(&self, number: i64) -> Option<i64> {
        if number >= self.low && number < self.high {
            return Some(number + self.change);
        }
        None
    }
}

struct Mapping {
    ranges: Vec<Range>,
}

impl Mapping {
    fn map(&self, number: i64) -> i64 {
        for range in &self.ranges {
            if let Some(value) = range.check(number) {
                return value;
            }
        }
        number
    }
}

struct Mappings {
    mappings: Vec<Mapping>,
}

impl Mappings {
    fn map(&self, mut number: i64) -> i64 {
        for mapping in &self.mappings {
            number = mapping.map(number);
        }
        number
    }
}

fn parse(input: &str) -> (Mappings, Vec<i64>) {
    let mut seeds: Vec<i64> = vec![];
    let mut mappings = vec![];
    let mut mapping = Mapping { ranges: vec![] };
    for line in input.lines() {
        let line = line.trim();
        if seeds.is_empty() {
            seeds = line.split(": ").collect::<Vec<_>>()[1]
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect();
        } else if line.is_empty() {
            if !mapping.ranges.is_empty() {
                mappings.push(mapping);
                mapping = Mapping { ranges: vec![] };
            }
        } else if line.find("map").is_none() {
            let values = line
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            mapping.ranges.push(Range {
                low: values[1],
                high: values[1] + values[2],
                change: values[0] - values[1],
            });
        }
    }

    mappings.push(mapping);

    (Mappings { mappings }, seeds)
}

fn part1(input: &str) -> i64 {
    let (mappings, seeds) = parse(input);

    seeds.into_iter().map(|x| mappings.map(x)).min().unwrap()
}

fn part2(input: &str) -> i64 {
    let (mappings, seeds) = parse(input);

    let mut result = i64::MAX;
    let mut i = 0;
    while i < seeds.len() {
        for j in seeds[i]..seeds[i] + seeds[i + 1] {
            result = result.min(mappings.map(j));
        }
        i += 2;
    }
    result
}

fn main() {
    let input = read_input(5);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1() {
        let input = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "
        .trim();

        assert_eq!(part1(input), 35);
    }

    #[test]
    fn test_day5_part2() {
        let input = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "
        .trim();

        assert_eq!(part2(input), 46);
    }
}

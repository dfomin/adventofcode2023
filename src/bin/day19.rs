use core::panic;
use std::collections::HashMap;

use adventofcode2023::read_input;

struct Part {
    categories: Vec<i32>,
}

#[derive(Debug, Clone)]
struct PartsGroup {
    categories: Vec<(i32, i32)>,
}

impl PartsGroup {
    fn number(&self) -> usize {
        let mut result = 1;
        for (min, max) in &self.categories {
            if min > max {
                return 0;
            }
            result *= (max - min + 1) as usize;
        }
        result
    }
}

struct Rule {
    value: i32,
    index: usize,
    is_less: bool,
    name: String,
}

impl Rule {
    fn check(&self, part: &Part) -> Option<String> {
        if self.is_less == (part.categories[self.index] < self.value) {
            return Some(self.name.clone());
        }
        None
    }

    fn split(&self, parts: PartsGroup) -> (PartsGroup, PartsGroup) {
        let mut true_parts = parts.clone();
        let mut false_parts = parts.clone();
        if self.is_less {
            true_parts.categories[self.index] = (
                parts.categories[self.index].0,
                parts.categories[self.index].1.min(self.value - 1),
            );
            false_parts.categories[self.index] = (
                parts.categories[self.index].0.max(self.value),
                parts.categories[self.index].1,
            );
        } else {
            true_parts.categories[self.index] = (self.value + 1, parts.categories[self.index].1);
            false_parts.categories[self.index] = (parts.categories[self.index].0, self.value);
        }
        (true_parts, false_parts)
    }
}

fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = vec![];
    let mut parse_parts = false;
    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            parse_parts = true;
            continue;
        }
        if !parse_parts {
            let mut workflow_parts = line.split('{');
            let name = workflow_parts.next().unwrap();
            for rule in workflow_parts
                .next()
                .unwrap()
                .split(',')
                .collect::<Vec<_>>()
            {
                let rule_parts = rule.split(':').collect::<Vec<_>>();
                let next_rule = if rule_parts.len() == 1 {
                    Rule {
                        value: 4001,
                        index: 0,
                        is_less: true,
                        name: String::from(&rule_parts[0][..rule_parts[0].len() - 1]),
                    }
                } else {
                    let is_less = rule_parts[0].contains('<');
                    let mut expression_parts = rule_parts[0].split(if is_less { '<' } else { '>' });
                    let index = match expression_parts.next().unwrap() {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => panic!("Unexpected"),
                    };
                    let value = expression_parts.next().unwrap().parse().unwrap();
                    Rule {
                        value,
                        index,
                        is_less,
                        name: rule_parts[1].to_string(),
                    }
                };

                workflows
                    .entry(name.to_string())
                    .or_insert(vec![])
                    .push(next_rule);
            }
        } else {
            let categories = line[1..line.len() - 1]
                .split(',')
                .map(|x| x.split('=').nth(1).unwrap().parse().unwrap())
                .collect();
            parts.push(Part { categories });
        }
    }
    (workflows, parts)
}

fn process(workflows: &HashMap<String, Vec<Rule>>, key: &str, part: &Part) -> usize {
    for rule in &workflows[key] {
        if let Some(next_workflow) = rule.check(&part) {
            match &next_workflow[..] {
                "A" => return part.categories.iter().sum::<i32>() as usize,
                "R" => return 0,
                key => return process(workflows, key, part),
            }
        }
    }
    0
}

fn process_groups(
    workflows: &HashMap<String, Vec<Rule>>,
    key: &str,
    mut parts_group: PartsGroup,
) -> usize {
    if key == "A" {
        return parts_group.number();
    } else if key == "R" {
        return 0;
    }
    let mut result = 0;
    for rule in &workflows[key] {
        let (true_parts, false_parts) = rule.split(parts_group);
        result += process_groups(workflows, &rule.name, true_parts);
        parts_group = false_parts;
    }
    result
}

fn part1(input: &str) -> usize {
    let (workflows, parts) = parse(input);
    let mut result = 0;
    for part in parts {
        result += process(&workflows, "in", &part)
    }
    result
}

fn part2(input: &str) -> usize {
    let (workflows, _) = parse(input);
    let parts_group = PartsGroup {
        categories: vec![(1, 4000); 4],
    };
    process_groups(&workflows, "in", parts_group)
}

fn main() {
    let input = read_input(19);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    ";

    #[test]
    fn test_day19_part1() {
        assert_eq!(part1(INPUT), 19114);
    }

    #[test]
    fn test_day19_part2() {
        assert_eq!(part2(INPUT), 167409079868000);
    }
}

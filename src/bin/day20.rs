use std::collections::{HashMap, HashSet};

use adventofcode2023::read_input;

struct Module {
    inputs: Vec<(String, bool)>,
    outputs: Vec<String>,
    name: String,
    module_type: String,
    is_on: bool,
}

struct Pulse {
    from: String,
    to: String,
    is_low: bool,
}

impl Module {
    fn pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        for i in 0..self.inputs.len() {
            if self.inputs[i].0 == pulse.from {
                self.inputs[i].1 = pulse.is_low;
            }
        }
        let mut output_pulses = Vec::new();
        match &self.module_type[..] {
            "broadcaster" => {
                output_pulses = self
                    .outputs
                    .iter()
                    .map(|x| Pulse {
                        from: self.name.clone(),
                        to: x.clone(),
                        is_low: pulse.is_low,
                    })
                    .collect();
            }
            "%" => {
                if pulse.is_low {
                    output_pulses = self
                        .outputs
                        .iter()
                        .map(|x| Pulse {
                            from: self.name.clone(),
                            to: x.clone(),
                            is_low: self.is_on,
                        })
                        .collect();
                    self.is_on = !self.is_on;
                }
            }
            "&" => {
                if self.inputs.iter().all(|x| !x.1) {
                    output_pulses = self
                        .outputs
                        .iter()
                        .map(|x| Pulse {
                            from: self.name.clone(),
                            to: x.clone(),
                            is_low: true,
                        })
                        .collect();
                } else {
                    output_pulses = self
                        .outputs
                        .iter()
                        .map(|x| Pulse {
                            from: self.name.clone(),
                            to: x.clone(),
                            is_low: false,
                        })
                        .collect();
                }
            }
            _ => panic!("Unexpected"),
        }
        output_pulses
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    let mut mapping = vec![];
    for line in input.trim().lines() {
        let mut iter = line.trim().split(" -> ");
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let name = if first == "broadcaster" {
            first
        } else {
            &first[1..]
        };
        let module = Module {
            inputs: vec![],
            outputs: second.split(", ").map(|x| x.to_string()).collect(),
            name: name.to_string(),
            module_type: if name == "broadcaster" {
                name.to_string()
            } else {
                first[0..1].to_string()
            },
            is_on: false,
        };
        mapping.push((name.to_string(), module.outputs.clone()));
        modules.insert(name.to_string(), module);
    }
    for (input, outputs) in mapping {
        for output in outputs {
            if modules.contains_key(&output) {
                modules
                    .get_mut(&output)
                    .unwrap()
                    .inputs
                    .push((input.to_string(), true));
            }
        }
    }
    modules
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        let t = a;
        a = b;
        b = t;
    }

    loop {
        a = a % b;
        if a == 0 {
            return b;
        }
        let t = a;
        a = b;
        b = t;
    }
}

fn lcm(numbers: Vec<usize>) -> usize {
    let mut current_gcd = numbers[0];
    let mut result = numbers[0];
    for i in 1..numbers.len() {
        current_gcd = gcd(current_gcd, numbers[i]);
        result *= numbers[i];
        result /= current_gcd;
    }
    result
}

fn part1(input: &str) -> usize {
    let mut modules = parse(input);
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let mut pulses = vec![Pulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            is_low: true,
        }];
        let mut i = 0;
        while i < pulses.len() {
            let pulse = &pulses[i];
            if pulse.is_low {
                low += 1;
            } else {
                high += 1;
            }
            if modules.contains_key(&pulse.to[..]) {
                let new_pulses = modules.get_mut(&pulse.to[..]).unwrap().pulse(pulse);
                pulses.extend(new_pulses);
            }
            i += 1;
        }
    }
    low * high
}

fn part2(input: &str) -> usize {
    let mut modules = parse(input);
    let mut presses = 1;
    let mut prev_node_inputs: HashMap<String, usize> = HashMap::new();
    let mut prev_name = String::new();
    for module in modules.values() {
        for output in &module.outputs {
            if output == "rx" {
                prev_name = module.name.clone();
                break;
            }
        }
        if !prev_name.is_empty() {
            break;
        }
    }
    loop {
        let mut pulses = vec![Pulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            is_low: true,
        }];
        let mut i = 0;
        while i < pulses.len() {
            let pulse = &pulses[i];
            if pulse.to == prev_name && !pulse.is_low {
                if !prev_node_inputs.contains_key(&pulse.from) {
                    prev_node_inputs.insert(pulse.from.to_string(), presses);
                    if prev_node_inputs.len() == modules[&prev_name].inputs.len() {
                        return lcm(prev_node_inputs.values().map(|x| *x).collect());
                    }
                }
            }
            if &pulse.to[..] == "rx" {
                if pulse.is_low {
                    return presses;
                }
            }
            if modules.contains_key(&pulse.to[..]) {
                let new_pulses = modules.get_mut(&pulse.to[..]).unwrap().pulse(pulse);
                pulses.extend(new_pulses);
            }
            i += 1;
        }
        presses += 1;
    }
}

fn main() {
    let input = read_input(20);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    ";

    const INPUT2: &str = "
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    ";

    #[test]
    fn test_day20_part1() {
        assert_eq!(part1(INPUT1), 32000000);
        assert_eq!(part1(INPUT2), 11687500);
    }

    #[test]
    fn test_day20_part2() {}
}

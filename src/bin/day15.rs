use adventofcode2023::read_input;

#[derive(Copy, Clone)]
enum Operator {
    Dash,
    Equal,
}

#[derive(Clone)]
struct Lens {
    label: String,
    operator: Operator,
    focus: Option<u64>,
}

impl Lens {
    fn from(input: &str) -> Self {
        if input.contains('-') {
            let mut parts = input.split('-');
            return Self {
                label: parts.next().unwrap().to_string(),
                operator: Operator::Dash,
                focus: None,
            };
        } else {
            let mut parts = input.split('=');
            return Self {
                label: parts.next().unwrap().to_string(),
                operator: Operator::Equal,
                focus: Some(parts.next().unwrap().parse::<u64>().unwrap()),
            };
        }
    }

    fn hash(&self) -> u64 {
        hash(&self.label[..])
    }
}

fn hash(input: &str) -> u64 {
    let mut result = 0;
    for ch in input.chars() {
        result += ch as u64;
        result *= 17;
        result %= 256;
    }
    result
}

fn part1(input: &str) -> u64 {
    input.split(',').map(|x| hash(x)).sum()
}

fn part2(input: &str) -> u64 {
    let lenses = input.split(',').map(|x| Lens::from(x));
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for lens in lenses {
        let mut found = false;
        let index = lens.hash() as usize;
        for i in 0..boxes[index].len() {
            if lens.label == boxes[index][i].label {
                match lens.operator {
                    Operator::Dash => {
                        boxes[index].remove(i);
                    }
                    Operator::Equal => boxes[index][i] = lens.clone(),
                }
                found = true;
                break;
            }
        }
        if !found && matches!(lens.operator, Operator::Equal) {
            boxes[index].push(lens);
        }
    }
    let mut result = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            result += (i as u64 + 1) * (j as u64 + 1) * boxes[i][j].focus.unwrap();
        }
    }
    result
}

fn main() {
    let input = read_input(15);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_day15_part1() {
        assert_eq!(part1(INPUT), 1320);
    }

    #[test]
    fn test_day15_part2() {
        assert_eq!(part2(INPUT), 145);
    }
}

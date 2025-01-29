use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use itertools::Itertools;

/// Day 7: Bridge Repair -- Part Two
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> u64 {
    let (_, equations) = parse(input).expect("should parse");
    equations.iter().filter_map(|eq| eq.is_resolvable()).sum()
}

fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(line_ending, equation)(input)
}

fn equation(input: &str) -> IResult<&str, Equation> {
    let (input, equation) = separated_pair(
        complete::u64,
        tag(": "),
        separated_list1(space1, complete::u64),
    )(input)?;
    Ok((input, Equation::new(equation.0, equation.1)))
}

const OPS: [char; 3] = ['*', '+', '|'];

#[derive(Debug)]
struct Equation {
    value: u64,
    numbers: Vec<u64>,
}
impl Equation {
    fn new(value: u64, numbers: Vec<u64>) -> Self {
        Self { value, numbers }
    }

    fn is_resolvable(&self) -> Option<u64> {
        let range = 0..self.numbers.len() - 1;
        let resolvable = range.map(|_| OPS).multi_cartesian_product().any(|ops| {
            let mut it = ops.iter();
            let mut acc = self.numbers[0];
            for n in self.numbers.iter().copied().skip(1) {
                if acc > self.value {
                    return false;
                }
                acc = match it.next().expect("should have next operator") {
                    '+' => acc + n,
                    '*' => acc * n,
                    '|' => format!("{acc}{n}").parse().unwrap(),
                    _ => panic!("unknown operator"),
                };
            }
            self.value == acc
        });

        if resolvable {
            Some(self.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let test = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = run(test);
        assert_eq!(result, 11387);
    }
}

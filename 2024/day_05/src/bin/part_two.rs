use std::cmp::Ordering;
use std::collections::HashMap;

use nom::{
    character::complete::{self, char, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

/// Day 5: Print Queue -- Part Two
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> u32 {
    let (_, mut manual) = parse(input).expect("should parse");
    manual.check_order()
}

fn parse(input: &str) -> IResult<&str, Manual> {
    let (input, rules) = terminated(rules, line_ending)(input)?;
    let (input, updates) = updates(input)?;
    Ok((input, Manual::new(rules, updates)))
}

fn rules(input: &str) -> IResult<&str, Rules> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, char('|'), complete::u32),
            line_ending,
        ),
        HashMap::new,
        |mut acc: HashMap<u32, Vec<u32>>, (px, py)| {
            acc.entry(px)
                .and_modify(|ys| {
                    ys.push(py);
                })
                .or_insert_with(|| vec![py]);
            acc
        },
    )(input)
}

fn updates(input: &str) -> IResult<&str, Vec<Update>> {
    separated_list1(line_ending, separated_list1(char(','), complete::u32))(input)
}

type Rules = HashMap<u32, Vec<u32>>;
type Update = Vec<u32>;
#[derive(Debug)]
struct Manual {
    rules: Rules,
    updates: Vec<Update>,
}
impl Manual {
    fn new(rules: Rules, updates: Vec<Update>) -> Self {
        Self { rules, updates }
    }

    fn check_order(&mut self) -> u32 {
        self.updates
            .iter_mut()
            .filter_map(|up| {
                if !Manual::is_sorted(&self.rules, up) {
                    up.sort_by(|a, b| {
                        if self.rules.get(a).is_some_and(|p| p.contains(b)) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    });
                    Some(up[up.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }

    fn is_sorted(rules: &Rules, update: &Update) -> bool {
        update
            .windows(2)
            .all(|a| rules.get(&a[0]).is_some_and(|p| p.contains(&a[1])))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let test = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = run(test);
        assert_eq!(result, 123);
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
enum Instruction {
    Mul((u32, u32)),
    Do,
    Dont,
}

/// Day 3: Mull it Over -- Part Two
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> u32 {
    let (_, instructions) = parse(input).expect("should parse");
    let mut exec = true;
    let mut sum = 0;
    for inst in instructions {
        match inst {
            Instruction::Mul((a, b)) => {
                if exec {
                    sum += a * b
                }
            }
            Instruction::Do => exec = true,
            Instruction::Dont => exec = false,
        }
    }
    sum
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, res) = many1(many_till(anychar, instruction))(input)?;
    let inst: Vec<Instruction> = res.into_iter().map(|(_, mul)| mul).collect::<_>();
    Ok((input, inst))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let test = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = run(test);
        assert_eq!(result, 48);
    }
}

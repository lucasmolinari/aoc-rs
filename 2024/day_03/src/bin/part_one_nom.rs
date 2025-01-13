use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

type Mul = (u32, u32);

/// Day 3: Mull it Over -- Part One
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> u32 {
    let (_, muls) = parse(input).expect("should parse");
    muls.iter().map(|(a, b)| a * b).sum()
}

fn mul(input: &str) -> IResult<&str, Mul> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, pair))
}

fn parse(input: &str) -> IResult<&str, Vec<Mul>> {
    let (input, res) = many1(many_till(anychar, mul))(input)?;
    let muls: Vec<Mul> = res.into_iter().map(|(_, mul)| mul).collect::<_>();
    Ok((input, muls))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_nom() {
        let test = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = run(test);
        assert_eq!(result, 161);
    }
}

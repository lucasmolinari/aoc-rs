use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: Vec<u64>,
    distance: Vec<u64>,
}

fn main() {
    let file = include_str!("input.txt");
    let out = run(file);
    println!("{}", out);
}

fn run(input: &str) -> u64 {
    let (_, race) = parse(input).expect("Should parse input");
    dbg!(&race.time, &race.distance);
    unimplemented!()
}

fn parse(input: &str) -> IResult<&str, Race> {
    let (input, time) = preceded(
        tuple((tag("Time:"), space1)),
        separated_list1(space1, complete::u64),
    )(input)?;
    let (input, _) = line_ending(input)?;
    let (input, distance) = preceded(
        tuple((tag("Distance:"), space1)),
        separated_list1(space1, complete::u64),
    )(input)?;
    Ok((input, Race { time, distance }))
}

// fn time(input: &str) -> IResult<&str, u64> {
//     Ok((input, time))
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let input = "Time:        53     71     78     80
Distance:   275   1181   1215   1524
";
        let result = run(input);
        assert_eq!(288, result)
    }
}

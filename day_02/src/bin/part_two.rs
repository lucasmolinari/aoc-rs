use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, i32, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: i32,
}

#[derive(Debug)]
struct Game<'a> {
    rounds: Vec<Vec<Cube<'a>>>,
}
impl<'a> Game<'a> {
    fn get_max(&self, color: &str) -> i32 {
        self.rounds
            .iter()
            .map(|round| {
                round
                    .iter()
                    .filter(|cube| cube.color == color)
                    .map(|cube| cube.amount)
                    .sum::<i32>()
            })
            .max()
            .unwrap()
    }

    fn get_power(&self) -> i32 {
        self.get_max("red") * self.get_max("green") * self.get_max("blue")
    }
}

fn parser(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game_parser)(input)?;
    Ok((input, games))
}

fn game_parser(input: &str) -> IResult<&str, Game> {
    let (input, _) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round_parser))(input)?;

    Ok((input, Game { rounds }))
}

fn round_parser(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube_parser)(input)?;
    Ok((input, cubes))
}

fn cube_parser(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(i32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, amount }))
}

fn main() {
    let file = include_str!("input.txt");
    let out = run(file);
    println!("{}", out);
}

fn run(input: &str) -> i32 {
    let games = parser(input).expect("Should parse input").1;
    games.iter().map(|game| game.get_power()).sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = run(&input);
        assert_eq!(2286, result)
    }
}

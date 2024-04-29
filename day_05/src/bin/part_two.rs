use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    maps: Vec<SeedMap>,
}
impl Almanac {
    fn map_seed(&self) -> Vec<u64> {
        let locations: Vec<u64> = self
            .seeds
            .iter()
            .flat_map(|range| range.clone())
            .map(|x| {
                let mut current_value = x;
                for map in &self.maps {
                    current_value = self.convert(current_value, map);
                }
                current_value
            })
            .collect();

        locations
    }

    fn convert(&self, val: u64, seed_map: &SeedMap) -> u64 {
        for (src, location) in &seed_map.lines {
            if src.contains(&val) {
                return location.start + (val - src.start);
            }
        }
        val
    }
}
#[derive(Debug)]
struct SeedMap {
    lines: Vec<(Range<u64>, Range<u64>)>,
}

fn main() {
    let file = include_str!("input.txt");
    let out = run(file);
    println!("{}", out);
}

fn run(input: &str) -> u64 {
    let (_, alm) = parser(input).expect("Should parse input");
    *alm.map_seed()
        .iter()
        .min()
        .expect("Should have a min value")
}

fn parser(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = preceded(tuple((tag("seeds:"), space1)), seed)(input)?;
    let (input, maps) = many1(maps)(input)?;
    Ok((input, Almanac { seeds, maps }))
}

fn seed(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let (input, seeds) =
        separated_list1(space1, separated_pair(complete::u64, space1, complete::u64))(input)?;
    Ok((input, seeds.iter().map(|&(start, n)| start..(start + n)).collect()))
}

fn maps(input: &str) -> IResult<&str, SeedMap> {
    let (input, _) = tuple((line_ending, take_until("map:"), tag("map:")))(input)?;
    let (input, lines) = preceded(line_ending, separated_list1(line_ending, line))(input)?;
    Ok((input, SeedMap { lines }))
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (dest, src, n)) = tuple((
        complete::u64,
        preceded(space1, complete::u64),
        preceded(space1, complete::u64),
    ))(input)?;
    Ok((input, (src..(src + n), dest..(dest + n))))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let input = "seeds: 79 14 55 13

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
        ";
        let result = run(input);
        assert_eq!(46, result)
    }
}

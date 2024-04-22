fn main() {
    let file = include_str!("input.txt");
    let out = run(file);
    println!("{}", out);
}

fn run(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|index| {
                match &line[index..] {
                    line if line.starts_with("one") => Some(1),
                    line if line.starts_with("two") => Some(2),
                    line if line.starts_with("three") => Some(3),
                    line if line.starts_with("four") => Some(4),
                    line if line.starts_with("five") => Some(5),
                    line if line.starts_with("six") => Some(6),
                    line if line.starts_with("seven") => Some(7),
                    line if line.starts_with("eight") => Some(8),
                    line if line.starts_with("nine") => Some(9),
                    line => {
                        line.chars().next().unwrap().to_digit(10)
                    }
                }
            });
            let first = match it.next() {
                Some(a) => a,
                None => panic   !("Got None"),
            };
            match it.last() {
                Some(n) => format!("{first}{n}").parse::<u32>(),
                None => format!("{first}{first}").parse::<u32>(),
            }.expect("Should be u32")
        })
        .sum::<u32>();
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let result = run("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, 281)
    }
}

fn main() {
    let file = include_str!("input.txt");
    let out = run(file);
    println!("{}", out);
}

fn run(input: &str) -> u32 {
    let output = input
        .lines()
        .map(|lines| {
            let mut it = lines.chars().filter_map(|character| character.to_digit(10));
            let first = it.next().expect("Should be u32");
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
    fn part_one() {
        let result = run("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, 142)
    }
}

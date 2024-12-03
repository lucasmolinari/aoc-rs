/// Day 1: Historian Hysteria -- Part One
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> u32 {
    let mut a: Vec<u32> = vec![];
    let mut b: Vec<u32> = vec![];
    input.lines().for_each(|l| {
        let mut parts = l.split_whitespace().take(2);
        a.push(parts.next().map(|n| n.parse::<u32>().unwrap()).unwrap());
        b.push(parts.next().map(|n| n.parse::<u32>().unwrap()).unwrap());
    });
    assert_eq!(a.len(), b.len());
    a.sort_unstable();
    b.sort_unstable();

    a.iter()
        .enumerate()
        .map(|(i, n)| {
            if *n >= b[i] {
                n.saturating_sub(b[i])
            } else {
                b[i].saturating_sub(*n)
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let test = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = run(test);
        assert_eq!(result, 11);
    }
}

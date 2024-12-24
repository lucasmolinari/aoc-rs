/// Day 2: Red-Nosed Reports -- Part Two
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let rep: Vec<u32> = l
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            rep.iter().enumerate().any(|(i, _)| {
                let mut repc = rep.clone();
                repc.remove(i);
                check(&repc)
            })
        })
        .filter(|u| *u)
        .count()
}

fn check(rep: &[u32]) -> bool {
    let expected_ord = match rep[0] < rep[1] {
        true => Order::Asc,
        false => Order::Des,
    };
    rep.windows(2).all(|pair| match pair {
        [a, b] => match expected_ord {
            Order::Asc => a < b && a.abs_diff(*b) <= 3,
            Order::Des => a > b && a.abs_diff(*b) <= 3,
        },
        _ => panic!("pair with a unexpected length"),
    })
}

enum Order {
    Asc,
    Des,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let test = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = run(test);
        assert_eq!(result, 4);
    }
}

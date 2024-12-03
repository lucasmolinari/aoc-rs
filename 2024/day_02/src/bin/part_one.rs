/// Day 2: Red-Nosed Reports -- Part One
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let nums: Vec<u32> = l
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            let expected_ord = match nums[0] < nums[1] {
                true => Order::Asc,
                false => Order::Des,
            };
            nums.windows(2).all(|pair| match pair {
                [a, b] => match expected_ord {
                    Order::Asc => a < b && a.abs_diff(*b) <= 3,
                    Order::Des => a > b && a.abs_diff(*b) <= 3,
                },
                _ => panic!("pair with a unexpected length"),
            })
        })
        .filter(|u| *u)
        .count()
}

enum Order {
    Asc,
    Des,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let test = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = run(test);
        assert_eq!(result, 2);
    }
}

use multipeek::multipeek;

#[derive(Debug, PartialEq, Clone)]
enum PointType {
    Number(i32),
    Symbol, // * % @ & / + $
    Empty,  // .
}

#[derive(Debug, Clone)]
struct Coordinate {
    y: i32,
    x: i32,
}

#[derive(Debug, Clone)]
struct Point {
    coordinate: Coordinate,
    point_type: PointType,
}

struct Grid {
    points: Vec<Vec<Point>>,
}

fn main() {
    let file = include_str!("input.txt");
    let out = run(file);
    println!("{}", out);
}

fn run(input: &str) -> i32 {
    let grid = Grid {
        points: input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let point_type = match c {
                            '.' => PointType::Empty,
                            c if c.is_ascii_alphanumeric() => {
                                PointType::Number(c.to_digit(10).expect("Should be a number") as i32)
                            }
                            _ => PointType::Symbol,
                        };
                        Point {
                            coordinate: Coordinate { y: y as i32, x: x as i32 },
                            point_type,
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>(),
    };

    let mut numbers: Vec<Vec<Point>> = Vec::new();
    for row in grid.points {
        let mut iter = multipeek(row.iter());
        while let Some(point) = iter.next() {
            let mut current_number: Vec<Point> = Vec::new();
            if let PointType::Number(_) = point.point_type {
                current_number.push(point.clone());
                while let Some(next) = iter.peek() {
                    if let PointType::Number(_) = next.point_type {
                        current_number.push(Point {
                            coordinate: next.coordinate.clone(),
                            point_type: next.point_type.clone(),
                        });
                        iter.next();
                    } else {
                        break;
                    }
                }
            }
            if !current_number.is_empty() {
                numbers.push(current_number);
            }
        }
    }
    dbg!(numbers.len());
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let result = run(input);
        assert_eq!(4361, result)
    }
}

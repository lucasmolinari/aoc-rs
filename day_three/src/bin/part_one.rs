use multipeek::multipeek;

#[derive(Debug, PartialEq, Clone)]
enum PointType {
    Number,
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
    value: String,
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
                            c if c.is_ascii_alphanumeric() => PointType::Number,
                            _ => PointType::Symbol,
                        };
                        Point {
                            coordinate: Coordinate {
                                y: y as i32,
                                x: x as i32,
                            },
                            point_type,
                            value: c.to_string(),
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>(),
    };

    let mut numbers: Vec<Vec<Point>> = Vec::new();
    for row in &grid.points {
        let mut iter = multipeek(row.iter());
        while let Some(point) = iter.next() {
            let mut current_number: Vec<Point> = Vec::new();
            if let PointType::Number = point.point_type {
                current_number.push(point.clone());
                while let Some(next) = iter.peek() {
                    if let PointType::Number = next.point_type {
                        current_number.push(Point {
                            coordinate: next.coordinate.clone(),
                            point_type: next.point_type.clone(),
                            value: next.value.clone(),
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
    check_symbol_adjacent(&grid, numbers)
}

fn check_symbol_adjacent(grid: &Grid, numbers: Vec<Vec<Point>>) -> i32 {
    let directions = vec![
        Coordinate { x: 0, y: -1 },  // Up
        Coordinate { x: 0, y: 1 },   // Down
        Coordinate { x: -1, y: 0 },  // Left
        Coordinate { x: 1, y: 0 },   // Right
        Coordinate { x: -1, y: -1 }, // Up Left
        Coordinate { x: 1, y: -1 },  // Up Right
        Coordinate { x: -1, y: 1 },  // Down Left
        Coordinate { x: 1, y: 1 },   // Down Right
    ];
    let mut valid_numbers: Vec<i32> = Vec::new();
    for number in numbers {
        let mut has_symbol_adj = false;
        for point in &number {
            for direction in &directions {
                let x = point.coordinate.x + direction.x;
                let y = point.coordinate.y + direction.y;

                if x < 0 || y < 0 {
                    continue;
                }

                if y as usize >= grid.points.len() {
                    continue;
                }

                if x as usize >= grid.points[y as usize].len() {
                    continue;
                }

                let grid_point = &grid.points[y as usize][x as usize];
                match grid_point.point_type {
                    PointType::Symbol => {
                        has_symbol_adj = true;
                        break;
                    }
                    _ => continue,
                }
            }
        }
        if has_symbol_adj {
            valid_numbers.push(number.iter().fold(0, |acc, elem| {
                acc * 10 + elem.value.parse::<i32>().expect("Should be a number")
            }));
        }
    }
    valid_numbers.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let input = "
        467..114..
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

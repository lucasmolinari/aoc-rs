use std::collections::HashSet;
use multipeek::multipeek;

#[derive(Debug, PartialEq, Clone)]
enum PointType {
    Number,
    Symbol, // * % @ & / + $
    Empty,  // .
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    y: i32,
    x: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Number {
    value: usize,
    coordinates: Vec<Coordinate>,
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

fn run(input: &str) -> usize {
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

    let numbers = convert_numbers(&grid);
    mount_gears(&grid, numbers)
}
fn convert_numbers(grid: &Grid) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    for row in &grid.points {
        let mut iter = multipeek(row.iter());
        while let Some(point) = iter.next() {
            let mut digits = String::new();
            let mut coordinates: Vec<Coordinate> = Vec::new();
            if let PointType::Number = point.point_type {
                digits.push_str(&point.value);
                coordinates.push(point.coordinate.clone());
                while let Some(next) = iter.peek() {
                    if let PointType::Number = next.point_type {
                        digits.push_str(&next.value);
                        coordinates.push(next.coordinate.clone());
                        iter.next();
                    } else {
                        break;
                    }
                }
            }
            if !digits.is_empty() && !coordinates.is_empty() {
                numbers.push(Number {
                    value: digits.parse::<usize>().expect("Should be a number"),
                    coordinates,
                });
            }
        }
    }
    numbers
}

fn mount_gears(grid: &Grid, numbers: Vec<Number>) -> usize {
    let directions = vec![
        Coordinate { x: 1, y: 0 },   // Right
        Coordinate { x: 1, y: -1 },  // Up Right
        Coordinate { x: 0, y: -1 },  // Up
        Coordinate { x: -1, y: -1 }, // Up Left
        Coordinate { x: -1, y: 0 },  // Left
        Coordinate { x: -1, y: 1 },  // Down Left
        Coordinate { x: 0, y: 1 },   // Down
        Coordinate { x: 1, y: 1 },   // Down Right
    ];
    let gears: Vec<HashSet<Number>> = grid
        .points
        .iter()
        .flatten()
        .map(|point| {
            let mut adjs: HashSet<Number> = HashSet::new();
            if PointType::Symbol == point.point_type {
                if "*" != point.value {
                    return adjs;
                }
                for direction in &directions {
                    let x = point.coordinate.x + direction.x;
                    let y = point.coordinate.y + direction.y;

                    if x < 0 || y < 0 {
                        continue;
                    }

                    for n in numbers.iter() {
                        if n.coordinates.iter().any(|c| c.x == x && c.y == y) {
                            adjs.insert(n.clone());
                            break;
                        }
                    }
                }
            }
            adjs
        })
        .collect();
    
    gears.iter().map(|adj| {
        if adj.len() != 2 {
            return 0;
        };
        adj.iter().map(|n| n.value).product::<usize>()
    }).sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
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
        assert_eq!(467835, result);
    }
}

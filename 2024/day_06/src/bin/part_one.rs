use std::collections::HashSet;

use util::grid::{Coord, Direction, Grid};

/// Day 6: Guard Gallivant -- Part One
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut grid = Grid::new(lines[0].len() as i64, lines.len() as i64);

    grid.set_all(|p| {
        let c = lines[p.y as usize]
            .chars()
            .nth(p.x as usize)
            .expect("should have character");
        match c {
            '.' => Point::Dot,
            '#' => Point::Wall,
            '<' | '>' | '^' | 'v' => Point::Guard,
            _ => unimplemented!(),
        }
    });
    predict(grid)
}

fn predict(grid: Grid<Point>) -> usize {
    let mut pos = grid.find(Point::Guard).expect("should find a guard");
    let mut dir = Coord::from(Direction::Up);

    let mut visited = HashSet::new();
    while grid.contains(pos + dir) {
        visited.insert(pos);

        let next = pos + dir;
        if grid[next] == Point::Wall {
            dir = dir.clockwise();
            continue;
        }

        pos += dir;
    }
    visited.len() + 1
}

#[derive(Debug, PartialEq)]
enum Point {
    Wall,
    Guard,
    Dot,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let test = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = run(test);
        assert_eq!(result, 41);
    }
}

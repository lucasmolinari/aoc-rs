use std::{collections::HashSet, fmt::Display};

use util::grid::{Coord, Direction, Grid};

/// Day 6: Guard Gallivant -- Part Two (Floyd's Tortoise and Hare)
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> u64 {
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

    let pos = grid.find(Point::Guard).expect("should find a guard");
    let visited = predict(&grid, pos);
    let mut sum = 0;

    for p in visited {
        if p == pos {
            continue;
        }
        grid[p] = Point::Wall;
        if check_cycle(&grid, pos) {
            sum += 1;
        };
        grid[p] = Point::Dot;
    }
    sum
}

fn predict(grid: &Grid<Point>, init_pos: Coord) -> HashSet<Coord> {
    let mut pos = init_pos;
    let mut dir = Coord::from(Direction::Up);

    let mut visited = HashSet::new();
    while grid.contains(pos + dir) {
        let next = pos + dir;
        if grid[next] == Point::Wall {
            dir = dir.clockwise();
            continue;
        }

        pos += dir;
        visited.insert(pos);
    }
    visited
}

fn check_cycle(grid: &Grid<Point>, init_pos: Coord) -> bool {
    let mut tpos = init_pos;
    let mut tdir = Coord::from(Direction::Up);
    let mut hpos = tpos;
    let mut hdir = tdir;

    let make_move = |pos: Coord, dir: Coord| -> Option<(Coord, Coord)> {
        let next = pos + dir;
        if !grid.contains(next) {
            return None;
        }

        if grid[next] == Point::Wall {
            Some((pos, dir.clockwise()))
        } else {
            Some((next, dir))
        }
    };

    loop {
        match make_move(tpos, tdir) {
            Some((new_pos, new_dir)) => {
                tpos = new_pos;
                tdir = new_dir;
            }
            None => return false, // tortoise exited
        }
        for _ in 0..2 {
            match make_move(hpos, hdir) {
                Some((new_pos, new_dir)) => {
                    hpos = new_pos;
                    hdir = new_dir;
                }
                None => return false, // hare exited
            }
        }

        if tpos == hpos && tdir == hdir {
            return true;
        }
    }
}

#[derive(Debug, PartialEq)]
enum Point {
    Wall,
    Guard,
    Dot,
    X,
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Wall => write!(f, "#"),
            Point::Guard => write!(f, "g"),
            Point::Dot => write!(f, "."),
            Point::X => write!(f, "X"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two_floyd() {
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
        assert_eq!(result, 6);
    }
}

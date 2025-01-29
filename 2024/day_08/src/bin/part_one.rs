use std::collections::HashSet;

use util::grid::{Coord, Grid};

/// Day 8: Resonant Collinearity -- Part One
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out);
}

fn run(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut grid = Grid::new(lines[0].len() as i64, lines.len() as i64);
    grid.set_all(|p| {
        let c = lines[p.y as usize]
            .chars()
            .nth(p.x as usize)
            .expect("should have a character");
        match c {
            '.' => Point::Dot,
            _ => Point::Antenna(c),
        }
    });
    calc_unique_antinodes(&mut grid)
}

fn calc_unique_antinodes(grid: &mut Grid<Point>) -> usize {
    let mut antinodes: HashSet<Coord> = HashSet::new();
    for (coord, point) in grid.iter() {
        if let Point::Antenna(c) = point {
            for fcoord in find_frequencies(coord, c, grid) {
                let diff = fcoord - coord;
                let a1 = fcoord + diff;
                let a2 = coord - diff;
                if grid.contains(a1) {
                    antinodes.insert(a1);
                }
                if grid.contains(a2) {
                    antinodes.insert(a2);
                }
            }
        }
    }
    antinodes.len()
}

fn find_frequencies(start: Coord, ch: &char, grid: &Grid<Point>) -> Vec<Coord> {
    let mut coords = vec![];
    for (coord, point) in grid.iter_from(start).skip(1) {
        if let Point::Antenna(c) = point {
            if c == ch {
                coords.push(coord);
            }
        }
    }
    coords
}

#[derive(Debug)]
enum Point {
    Antenna(char),
    Dot,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let test = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let result = run(test);
        assert_eq!(result, 14);
    }
}

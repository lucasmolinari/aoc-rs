use std::collections::HashSet;

use util::grid::{Coord, Grid};

/// Day 8: Resonant Collinearity -- Part Two
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
            antinodes.insert(coord);
            for fcoord in find_frequencies(coord, c, grid) {
                antinodes.insert(fcoord);
                let diff = fcoord - coord;

                let mut a1 = fcoord + diff;
                while grid.contains(a1) {
                    antinodes.insert(a1);
                    a1 += diff;
                }

                let mut a2 = coord - diff;
                while grid.contains(a2) {
                    antinodes.insert(a2);
                    a2 -= diff;
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
    fn part_two() {
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
        assert_eq!(result, 34);
    }
}

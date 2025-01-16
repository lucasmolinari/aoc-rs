use std::collections::HashSet;

/// Day 6: Guard Gallivant -- Part One
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.predict()
}

struct Grid {
    points: Vec<Vec<Point>>,
    guard_pos: (usize, usize),
}
impl Grid {
    fn new(input: &str) -> Self {
        let mut guard_pos = (0, 0);
        let points: Vec<Vec<Point>> = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Point::Dot,
                        '#' => Point::Wall,
                        '<' => {
                            guard_pos = (x, y);
                            Point::Guard(Direction::Left)
                        }
                        '>' => {
                            guard_pos = (x, y);
                            Point::Guard(Direction::Right)
                        }
                        '^' => {
                            guard_pos = (x, y);
                            Point::Guard(Direction::Up)
                        }
                        'v' => {
                            guard_pos = (x, y);
                            Point::Guard(Direction::Down)
                        }
                        _ => unimplemented!(),
                    })
                    .collect()
            })
            .collect();
        Self { points, guard_pos }
    }

    fn predict(&mut self) -> usize {
        let mut visited = HashSet::new();
        let max_y = self.points.len();
        let max_x = self.points[0].len();
        while self.guard_pos.1 < max_y && self.guard_pos.0 < max_x {
            let (gx, gy) = (self.guard_pos.0, self.guard_pos.1);
            visited.insert(self.guard_pos);

            match self.points[gy][gx] {
                Point::Guard(dir) => {
                    let (nx, ny) = match dir.next_pos(gx, gy, max_x, max_y) {
                        Some((x, y)) => (x, y),
                        None => break,
                    };
                    match &self.points[ny][nx] {
                        Point::Wall => {
                            let nd = dir.rotate();
                            let n = match nd.next_pos(gx, gy, max_x, max_y) {
                                Some(n) => n,
                                None => break,
                            };
                            self.update_guard((gx, gy), n, nd);
                        }
                        Point::Dot => self.update_guard((gx, gy), (nx, ny), dir),
                        _ => panic!("found multiple guards"),
                    }
                }
                _ => panic!(
                    "point at guard_pos is not a guard: ({},{}) {:?}",
                    gx, gy, self.points[gy][gx]
                ),
            }
        }
        visited.len()
    }

    fn update_guard(
        &mut self,
        curr_p: (usize, usize),
        next_p: (usize, usize),
        direction: Direction,
    ) {
        self.guard_pos.0 = next_p.0;
        self.guard_pos.1 = next_p.1;
        self.points[curr_p.1][curr_p.0] = Point::Dot;
        self.points[next_p.1][next_p.0] = Point::Guard(direction);
    }
}

#[derive(Debug)]
enum Point {
    Wall,
    Guard(Direction),
    Dot,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn rotate(&self) -> Self {
        match &self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    fn next_pos(&self, x: usize, y: usize, max_x: usize, max_y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Left => x.checked_sub(1).map(|new_x| (new_x, y)),
            Direction::Right => {
                let new_x = x.checked_add(1)?;
                if new_x < max_x {
                    Some((new_x, y))
                } else {
                    None
                }
            }
            Direction::Up => y.checked_sub(1).map(|new_y| (x, new_y)),
            Direction::Down => {
                let new_y = y.checked_add(1)?;
                if new_y < max_y {
                    Some((x, new_y))
                } else {
                    None
                }
            }
        }
    }
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

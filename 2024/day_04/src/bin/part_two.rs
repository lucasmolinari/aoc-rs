/// Day 4: Ceres Search -- Part Two
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> u32 {
    let grid = Grid::new(input);
    grid.start_search()
}

struct Grid {
    points: Vec<Vec<Letter>>,
}
impl Grid {
    fn new(input: &str) -> Self {
        let points: Vec<Vec<Letter>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'X' => Letter::X,
                        'M' => Letter::M,
                        'A' => Letter::A,
                        'S' => Letter::S,
                        _ => unimplemented!(),
                    })
                    .collect()
            })
            .collect();
        Grid { points }
    }

    fn start_search(&self) -> u32 {
        let directions = [
            (-1, -1), // Up Left
            (1, 1),   // Down Right
            (1, -1),  // Up Right
            (-1, 1),  // Down Left
        ];

        let mut sum = 0;
        for (y, col) in self.points.iter().enumerate() {
            for (x, letter) in col.iter().enumerate() {
                if letter != &Letter::A {
                    continue;
                }

                let neighbors: Vec<&Letter> = directions
                    .iter()
                    .filter_map(|dir| {
                        let new_x = (dir.0 + x as isize) as usize;
                        let new_y = (dir.1 + y as isize) as usize;
                        let size = (self.points.len(), self.points[0].len());

                        if new_x >= size.0 || new_y >= size.1 {
                            return None;
                        }

                        let to_check = &self.points[new_y][new_x];
                        if matches!(to_check, Letter::M | Letter::S) {
                            Some(to_check)
                        } else {
                            None
                        }
                    })
                    .collect();

                if neighbors.len() != 4 {
                    continue;
                }
                if neighbors[0] != neighbors[1] && neighbors[2] != neighbors[3] {
                    sum += 1
                }
            }
        }
        sum
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let test = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = run(test);
        assert_eq!(result, 9);
    }
}

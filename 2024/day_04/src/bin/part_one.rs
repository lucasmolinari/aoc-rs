/// Day 4: Ceres Search -- Part One
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
            (0, -1),  // Up
            (0, 1),   // Down
            (-1, 0),  // Left
            (1, 0),   // Right
            (-1, -1), // Up Left
            (1, -1),  // Up Right
            (-1, 1),  // Down Left
            (1, 1),   // Down Right
        ];

        self.points.iter().enumerate().fold(0, |sum, (y, col)| {
            sum + col
                .iter()
                .enumerate()
                .filter(|&(_, letter)| letter == &Letter::X)
                .map(|(x, _)| {
                    directions
                        .iter()
                        .filter(|&&dir| self.search_word((x, y), dir, Letter::M))
                        .count() as u32
                })
                .sum::<u32>()
        })
    }
    fn search_word(&self, coord: (usize, usize), dir: (isize, isize), expected: Letter) -> bool {
        let new_x = (dir.0 + coord.0 as isize) as usize;
        let new_y = (dir.1 + coord.1 as isize) as usize;

        let size = (self.points.len(), self.points[0].len());
        if new_x >= size.0 || new_y >= size.1 {
            return false;
        }

        let to_check = &self.points[new_y][new_x];
        if to_check == &expected {
            return match to_check {
                Letter::X => self.search_word((new_x, new_y), dir, Letter::M),
                Letter::M => self.search_word((new_x, new_y), dir, Letter::A),
                Letter::A => self.search_word((new_x, new_y), dir, Letter::S),
                Letter::S => true,
            };
        }

        false
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
    fn part_one() {
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
        assert_eq!(result, 18);
    }
}

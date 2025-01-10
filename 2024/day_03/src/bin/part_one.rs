/// Day 3: Mull it Over -- Part One
/// Brute force with a weird little handmade parser.
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out)
}

fn run(input: &str) -> u64 {
    let mut parser = Parser::new(input);
    parser.parse()
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    read_pos: usize,
    ch: char,
}
impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut p = Self {
            input,
            pos: 0,
            read_pos: 0,
            ch: ' ',
        };
        p.read_char();
        p
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_pos).unwrap_or('\0');
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn parse(&mut self) -> u64 {
        let mut sum = 0;
        while self.ch != '\0' {
            sum += self.read_mul().unwrap_or_default();
        }
        sum
    }

    fn read_mul(&mut self) -> Option<u64> {
        if self.pos + 3 <= self.input.len() && &self.input[self.pos..self.pos + 3] == "mul" {
            self.read_pos += 2;
            self.read_char();
            self.read_numbers()
        } else {
            self.read_char();
            None
        }
    }

    fn read_numbers(&mut self) -> Option<u64> {
        self.expect_peek('(').ok()?;
        let n1 = self.read_int()?;
        self.expect_peek(',').ok()?;
        let n2 = self.read_int()?;
        self.expect_peek(')').ok()?;

        Some(n1 * n2)
    }

    fn read_int(&mut self) -> Option<u64> {
        let prev_pos = self.pos;
        while self.ch.is_alphanumeric() {
            self.read_char();
        }
        let res = &self.input[prev_pos..self.pos];

        if res.len() > 3 {
            None
        } else {
            res.parse::<u64>().ok()
        }
    }

    fn expect_peek(&mut self, ch: char) -> Result<(), ()> {
        if self.ch == ch {
            self.read_char();
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let test = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = run(test);
        assert_eq!(result, 161);
    }
}

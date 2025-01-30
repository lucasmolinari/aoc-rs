/// Day 9: Disk Fragmenter -- Part One
fn main() {
    let input = include_str!("input.txt");
    let out = run(input);
    println!("-> {}", out);
}

fn run(input: &str) -> usize {
    let mut disk = parse(input.trim());
    let mut free_idx = disk
        .iter()
        .position(|d| *d == Data::Space)
        .expect("should have a free index");
    let max = disk
        .iter()
        .filter(|&d| *d != Data::Space)
        .collect::<Vec<_>>()
        .len();
    for i in (0..disk.len()).rev() {
        if disk[0..max].iter().all(|d| *d != Data::Space) {
            break;
        }
        if free_idx >= disk.len() {
            break;
        }

        let data = disk[i].clone();
        if data != Data::Space {
            disk[free_idx] = data;
            disk[i] = Data::Space;
            while free_idx < disk.len() && disk[free_idx] != Data::Space {
                free_idx += 1;
            }
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, d)| {
            if let Data::File(id) = d {
                Some(id * i)
            } else {
                None
            }
        })
        .sum()
}

fn parse(input: &str) -> Vec<Data> {
    let mut disk: Vec<Data> = vec![];

    let mut is_file = true;
    let mut fid = 0;
    for c in input.chars() {
        let range = 0..c.to_digit(10).expect("should be digit");
        if is_file {
            for _ in range {
                disk.push(Data::File(fid));
            }
            fid += 1;
        } else {
            for _ in range {
                disk.push(Data::Space);
            }
        }
        is_file = !is_file;
    }
    disk
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Data {
    File(usize),
    Space,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let test = "2333133121414131402";
        let result = run(test);
        assert_eq!(result, 1928);
    }
}

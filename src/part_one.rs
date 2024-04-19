use std::fs::File;
use std::io::{prelude::*, BufReader};


pub fn run(){
    let file = File::open("./inputs/input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut res = String::new();
    let mut sum = 0;
    for l in reader.lines() {
        for c in l.unwrap().chars() {
            if c.is_numeric() {
                if res.len() > 1 {
                    res.replace_range(1..2, &c.to_string());
                } else {
                    res.push(c);
                    res.push(c);
                }
            }
        }
        sum += res.parse::<i32>().unwrap();
        res.clear()
    }
    println!("{}", sum)

}
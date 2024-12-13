use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Map {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}

impl Map {
    fn add(&mut self, row: Row) -> () {
        self.left.push(row.left);
        self.right.push(row.right);
    }

    fn sort(&mut self) -> () {
        self.left.sort();
        self.right.sort();
    }

    fn sum(&mut self) -> i32 {
        let mut sum: i32 = 0;
        for (left, right) in self.left.iter().zip(self.right.iter()) {
            let partsum: i32 = (left - right).into();
            sum = sum + <i32 as Into<i32>>::into(partsum.abs());
        }
        sum
    }

    fn similarity(&mut self) -> i32 {
        let mut similarity: i32 = 0;
        for left in self.left.iter() {
            let mut count: i32 = 0;
            for right in self.right.iter() {
                if left == right {
                    count += 1;
                }
            }
            similarity += left * count;
        }
        similarity
    }
}

pub struct Row {
    pub left: i32,
    pub right: i32,
}

impl Row {
    fn new(row: String) -> Row {
        let numbers: Vec<i32> = row
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        Row {
            left: numbers[0],
            right: numbers[1],
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.left, self.right)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut map = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            map.push(numbers);
        }
    }

    map.sort();
    let sum = map.sum();
    println!("Length: {}", sum);

    let similarity = map.similarity();
    println!("Similarity: {}", similarity);
}

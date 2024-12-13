use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn check_level(arr: &[i32]) -> bool {
    let first_diff = arr[1] - arr[0];
    let direction = match first_diff {
        1..=3 => 1,
        -3..=-1 => -1,
        _ => return false,
    };

    arr.iter()
        .zip(arr.iter().skip(1))
        .skip(1)
        .all(|(&a, &b)| match (direction, b - a) {
            (1, 1..=3) => true,
            (-1, -3..=-1) => true,
            _ => false,
        })
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

    let mut data = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            data.push(levels);
        }
    }

    let mut safe_levels: i32 = 0;

    for report in data.iter() {
        if check_level(report) {
            safe_levels += 1;
        }
    }

    println!("Safe levels: {}", safe_levels);

    safe_levels = 0;

    for report in data.iter() {
        let mut safe = 0;

        for i in 0..report.len() {
            let mut cut = report.clone();
            cut.remove(i);
            if check_level(&cut) {
                safe = 1;
                break;
            }
        }
        safe_levels += safe;
    }
    println!("Safe levels: {}", safe_levels);
}

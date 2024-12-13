use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Div, Mul, Rem};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn len(number: usize) -> usize {
    return (number.checked_ilog10().unwrap_or(0) + 1)
        .try_into()
        .unwrap();
}

fn split(stone: usize) -> (usize, usize) {
    let l = len(stone);
    return (
        stone.div(10_usize.pow(l.div(2).try_into().unwrap())),
        stone.rem(10_usize.pow(l.div(2).try_into().unwrap())),
    );
}

fn timetravel(cache: &mut HashMap<(usize, usize), usize>, stone: usize, step: usize) -> usize {
    let len = len(stone);
    match cache.get(&(stone, step)) {
        Some(result) => {
            return *result;
        }
        None => {
            if step == 101 {
                return 1;
            } else if stone == 0 {
                let result = timetravel(cache, stone + 1, step + 1);
                cache.insert((stone, step), result);
                return result;
            } else if len.rem(2) == 0 {
                let mut result: usize = 0;
                let pair = split(stone);
                result += timetravel(cache, pair.0, step + 1);
                result += timetravel(cache, pair.1, step + 1);
                cache.insert((stone, step), result);
                return result;
            } else {
                let result = timetravel(cache, stone.mul(2024), step + 1);
                cache.insert((stone, step), result);
                return result;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut stones: Vec<usize> = Vec::new();

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            for stone in line.split_whitespace() {
                stones.push(stone.parse::<usize>().unwrap());
            }
        }
    }

    let mut count: usize = 0;

    //count += timetravel(stones[0], 0);

    for stone in stones {
        //println!("Stone: {}", stone);
        count += timetravel(&mut cache, stone, 0);
    }

    //println!("{:?}", cache);
    println!("P1: {}", count);
}

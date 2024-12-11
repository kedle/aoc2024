use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Div, Mul, Rem};
use std::path::Path;
use std::thread;

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

fn timetravel(stone: usize, step: usize) -> usize {
    let len = len(stone);
    if step == 25 {
        return 1;
    } else if stone == 0 {
        println!("ZERO! step:{},", step);
        return timetravel(stone + 1, step + 1);
    } else if len.rem(2) == 0 {
        let mut count: usize = 0;
        let pair = split(stone);
        println!("SPLIT! step: {}, pair: {:?}", step, pair);
        count += timetravel(pair.0, step + 1);
        count += timetravel(pair.1, step + 1);
        return count;
    } else {
        println!("MUL! step:{}", step);
        timetravel(stone.mul(2024), step + 1)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut stones: Vec<usize> = Vec::new();

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
        count += timetravel(stone, 0);
    }

    //println!("{:?}", stones);
    println!("P1: {}", count);
}

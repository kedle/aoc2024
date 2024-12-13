use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Puzzle {
    goal: usize,
    pieces: Vec<usize>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn solve_p1(puzzle: &Puzzle, i: usize, sum: usize) -> usize {
    match puzzle.pieces.get(i) {
        Some(next_val) => {
            let mut mid_sum: usize = 0;
            mid_sum += solve_p1(puzzle, i + 1, sum + next_val);
            if mid_sum != puzzle.goal {
                mid_sum += solve_p1(puzzle, i + 1, sum * next_val);
            }
            return mid_sum;
        }
        None if sum == puzzle.goal => {
            return sum;
        }
        None => {
            return 0;
        }
    }
}

fn concatenate(l: usize, r: usize) -> usize {
    let concatenated = format!("{}{}", l.to_string(), r.to_string());
    return concatenated.parse().unwrap();
}

fn solve_p2(puzzle: &Puzzle, i: usize, sum: usize) -> usize {
    match puzzle.pieces.get(i) {
        Some(next_val) => {
            let mut mid_sum: usize = 0;
            mid_sum += solve_p2(puzzle, i + 1, sum + next_val);
            if mid_sum != puzzle.goal {
                mid_sum += solve_p2(puzzle, i + 1, sum * next_val);
            }
            if mid_sum != puzzle.goal {
                mid_sum += solve_p2(puzzle, i + 1, concatenate(sum, *next_val));
            }
            return mid_sum;
        }
        None if sum == puzzle.goal => {
            return sum;
        }
        None => {
            return 0;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut data: Vec<Puzzle> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut colon = line.split(':');
            let goal = colon.next().unwrap().parse::<usize>().unwrap();
            let pieces: Vec<usize> = colon
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse().unwrap())
                .collect();
            data.push(Puzzle { goal, pieces });
        }
    }

    let mut sum1: usize = 0;

    for puzzle in data.iter() {
        let psum = solve_p1(&puzzle, 1, *puzzle.pieces.get(0).unwrap());
        sum1 += psum;
    }

    println!("P1: {}", sum1);

    let mut sum2: usize = 0;

    for puzzle in data.iter() {
        let psum = solve_p2(&puzzle, 1, *puzzle.pieces.get(0).unwrap());
        sum2 += psum;
    }

    println!("P2: {}", sum2);
}

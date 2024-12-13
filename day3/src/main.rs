use regex::Regex;
use std::env;
use std::num::ParseIntError;

enum Operation {
    Mul(i32, i32),
    Do,
    Dont,
}

impl Operation {
    fn from_caps(caps: &regex::Captures) -> Result<Self, ParseIntError> {
        match caps.name("op").unwrap().as_str() {
            "mul" => {
                let (val1, val2) = caps["vals"]
                    .split_once(',')
                    .expect("Expected two values separated by a comma");
                let val1: i32 = val1.parse()?;
                let val2: i32 = val2.parse()?;
                Ok(Operation::Mul(val1, val2))
            }
            "do" => Ok(Operation::Do),
            "don't" => Ok(Operation::Dont),
            _ => panic!("Unknown operation"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let data = std::fs::read_to_string(filename).expect("Unable to read file");

    let re = Regex::new(r"(?<op>mul|do|don\'t)(\((?<vals>(\d{1,3})\,(\d{1,3}))?\))").unwrap();

    let ops: Vec<Operation> = re
        .captures_iter(&data)
        .map(|caps| Operation::from_caps(&caps))
        .collect::<Result<_, _>>()
        .expect("Failed to parse operations");

    // P1
    let sum: i32 = ops
        .iter()
        .filter_map(|op| {
            if let Operation::Mul(a, b) = op {
                Some(a * b)
            } else {
                None
            }
        })
        .sum();

    println!("P1: {}", sum);

    // P2
    let mut enabled = true;
    let mut sum2 = 0;

    for op in &ops {
        match op {
            Operation::Mul(a, b) if enabled => sum2 += a * b,
            Operation::Do => enabled = true,
            Operation::Dont => enabled = false,
            _ => {}
        }
    }

    println!("P2: {}", sum2);
}

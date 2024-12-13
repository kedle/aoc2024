use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Div;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// Returns (true, 0) for allowed updates
// Returns (false, i) for disallowed updates (pointing the fault)
// true and false are reversed for reasons unknown to me
fn is_allowed(update: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> (bool, Vec<usize>) {
    let mut checked: HashMap<usize, bool> = HashMap::new();
    let mut faults: Vec<usize> = Vec::new();
    let mut faulty = true;
    for (i, page) in update.iter().enumerate() {
        if let Some(&ref ruleset) = rules.get(&page) {
            let checklist: Vec<&usize> = ruleset.iter().filter(|x| update.contains(x)).collect();
            if !checklist.iter().all(|key| checked.contains_key(key)) {
                faults.push(i);
                faulty = false;
            }
        }
        checked.insert(*page, true);
    }
    return (faulty, faults);
}

// Returns true if the update should be retained for P2
// Updates sum to count the score for P1
fn check(update: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>, sum: &mut usize) -> bool {
    let allowed = is_allowed(&update, &rules);
    match allowed {
        (true, _) => {
            *sum += *update.get(update.len().div(2)).unwrap();
            return false;
        }
        _ => {
            return true;
        }
    }
}

fn check_and_repair(update: &mut Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> usize {
    let (allowed, faults) = is_allowed(&update, &rules);
    match allowed {
        false => {
            for fault in faults.iter().rev() {
                update.swap(*fault, *fault + 1);
            }

            check_and_repair(update, rules)
        }
        true => {
            return *update.get(update.len().div(2)).unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let rulefile = &args[1];
    let updatefile = &args[2];

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    // Read rules
    if let Ok(lines) = read_lines(rulefile) {
        for line in lines.flatten() {
            let rule: Vec<usize> = line
                .split("|")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            match rules.get_mut(&rule[1]) {
                Some(val) if !val.contains(&rule[1]) => val.push(rule[0]),
                None => {
                    rules.insert(rule[1], vec![rule[0]]);
                    ()
                }
                _ => {}
            }
        }
    }

    // Read updates
    if let Ok(lines) = read_lines(updatefile) {
        for line in lines.flatten() {
            let update: Vec<usize> = line
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            updates.push(update);
        }
    }

    let mut p1_sum: usize = 0;
    updates.retain(|mut x| check(&mut x, &rules, &mut p1_sum));
    println!("P1: {}", p1_sum);

    let p2: usize = updates
        .into_iter()
        .map(|mut x| check_and_repair(&mut x, &rules))
        .collect::<Vec<usize>>()
        .iter()
        .fold(0, |sum, e| sum + e);

    println!("P2: {}", p2);
}

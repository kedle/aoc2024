use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::{Enumerate, Rev};
use std::path::Path;
use std::slice::Iter;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn construct_filesystem(data: &Vec<usize>) -> Vec<Option<usize>> {
    let mut filesystem: Vec<Option<usize>> = Vec::new();

    let mut file_index: usize = 0;
    let mut data_mode: bool = true;

    for op in data.iter() {
        for _k in 0..*op {
            if data_mode {
                filesystem.push(Some(file_index));
            } else {
                filesystem.push(None);
            }
        }
        if data_mode {
            file_index += 1;
        }
        data_mode = !data_mode;
    }

    return filesystem;
}

fn find_free_slot(forwards: &mut Enumerate<Iter<Option<usize>>>) -> usize {
    match forwards.next() {
        Some((i, None)) => return i.try_into().unwrap(),
        _ => find_free_slot(forwards),
    }
}

fn find_data(backwards: &mut Rev<Enumerate<Iter<Option<usize>>>>) -> usize {
    match backwards.next() {
        Some((i, Some(_data))) => return i.try_into().unwrap(),
        Some((_i, None)) => find_data(backwards),
        _ => 0,
    }
}

fn find_free_slot_with_size(filesystem: &Vec<Option<usize>>, size: usize) -> usize {
    for (i, window) in filesystem.windows(size).enumerate() {
        match window.iter().all(|x| x.is_none()) {
            true => {
                return i;
            }
            false => {}
        }
    }
    return 0;
}

// Finds the next data section starting at start (from the end). Returns (position, length of data)
fn find_data_with_size(filesystem: &Vec<Option<usize>>, start: usize) -> (usize, usize) {
    let mut iterator = filesystem
        .iter()
        .enumerate()
        .rev()
        .skip(filesystem.len() - start)
        .peekable();
    let pos: usize;
    let data: usize;

    // Find first data beginning from start
    loop {
        match iterator.next() {
            Some((i, Some(file))) => {
                data = *file;
                pos = i;
                break;
            }
            _ => {}
        }
    }

    // Find other files belonging to that data
    let mut length: usize = 1;
    loop {
        match iterator.peek() {
            Some((_i, Some(file))) if *file == data => {
                length += 1;
                iterator.next();
            }
            _ => break,
        }
    }

    return (pos, length);
}

fn reorder(filesystem: &Vec<Option<usize>>) -> Vec<(usize, usize)> {
    let mut forwards = filesystem.iter().enumerate();
    let mut backwards = filesystem.iter().enumerate().rev();

    let mut swaplist: Vec<(usize, usize)> = Vec::new();

    loop {
        let free_slot = find_free_slot(&mut forwards);
        let data = find_data(&mut backwards);

        // Stop in the middle, maybe not correct..
        if free_slot > data {
            return swaplist;
        }

        swaplist.push((free_slot, data));
    }
}

fn reorder2(filesystem: &mut Vec<Option<usize>>) -> () {
    let mut right_pos: usize = filesystem.len();

    loop {
        if right_pos == 0 {
            break;
        };
        let (pos, length) = find_data_with_size(filesystem, right_pos);
        let slot = find_free_slot_with_size(filesystem, length);

        let a = slot;
        let b = pos;
        if slot != 0 && slot < pos {
            for i in 0..length {
                filesystem.swap(a + i, b - i);
            }
        }

        if length <= pos {
            right_pos = pos - length + 1;
        } else {
            break;
        }
    }
}

fn swap(filesystem: &mut Vec<Option<usize>>, swaplist: &Vec<(usize, usize)>) -> () {
    for swap in swaplist.iter() {
        filesystem.swap(swap.0, swap.1);
    }
}

fn checksum(filesystem: &Vec<Option<usize>>) -> usize {
    let mut sum: usize = 0;
    for (i, file) in filesystem.iter().enumerate() {
        match file {
            Some(val) => sum += i * val,
            None => {}
        }
    }
    return sum;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut data: Vec<usize> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        const RADIX: u32 = 10;
        for line in lines.flatten() {
            data = line
                .chars()
                .map(|x| x.to_digit(RADIX).unwrap().try_into().unwrap())
                .collect::<Vec<usize>>();
        }
    }

    let mut filesystem = construct_filesystem(&data);
    let mut filesystem2 = filesystem.clone();

    // Part 1
    let swaplist = reorder(&filesystem);
    swap(&mut filesystem, &swaplist);
    let sump1 = checksum(&filesystem);
    println!("P1: {}", sump1);

    // Part 2
    reorder2(&mut filesystem2);
    let sump2 = checksum(&filesystem2);
    println!("P2: {}", sump2);
}

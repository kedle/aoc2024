use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Div, Rem};
use std::path::Path;

#[derive(Clone, Debug, Default)]
struct Matrix<T> {
    cols: usize,
    data: Vec<T>,
}

impl<T: Default + Copy + std::fmt::Display> Matrix<T> {
    pub fn new(cols: usize, data: Vec<T>) -> Self {
        Self { cols, data }
    }

    pub fn get_mut(&mut self, pos: isize) -> Option<&mut T> {
        if pos >= 0 && pos < self.data.len() as isize {
            return Some(&mut self.data[pos as usize]);
        } else {
            return None;
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if self.check_boundaries(x, y) {
            let pos = self.pos_from_coords(x.try_into().unwrap(), y.try_into().unwrap());
            if pos >= self.data.len() {
                return None;
            }
            return Some(&self.data[pos]);
        }
        None
    }

    fn check_boundaries(&self, x: isize, y: isize) -> bool {
        if x >= 0
            && y >= 0
            && x < self.cols.try_into().unwrap()
            && y < self.data.len().div(self.cols).try_into().unwrap()
        {
            return true;
        } else {
            return false;
        }
    }

    pub fn pos_from_coords(&self, x: usize, y: usize) -> usize {
        let asd = &self.cols * y + x;
        //println!("asd: {}", &asd);
        return asd;
    }

    pub fn coords_from_pos(&self, pos: usize) -> (usize, usize) {
        let x = pos.rem(&self.cols);
        let y = pos.div(&self.cols);
        return (x, y);
    }

    pub fn icoords_from_pos(&self, pos: usize) -> (isize, isize) {
        let x = pos.rem(&self.cols) as isize;
        let y = pos.div(&self.cols) as isize;
        return (x, y);
    }
}

impl fmt::Display for Matrix<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, c) in self.data.iter().enumerate() {
            if i.rem(self.cols) == 0 && i >= self.cols {
                write!(f, "\n")?;
            }
            write!(f, "{} ", c)?;
        }
        Ok(())
    }
}

impl Matrix<u32> {
    fn dbg(&self, pos: usize, dir: usize) -> () {
        for (i, c) in self.data.iter().enumerate() {
            if i.rem(self.cols) == 0 {
                println!("");
            }
            if i == pos {
                print!("[{}]", c);
            } else if i == dir {
                print!("<{}>", c);
            } else {
                print!(" {} ", c);
            }
        }
        println!("");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn traverse(
    pos: usize,
    map: &Matrix<u32>,
    i: u32,
    directions: &Vec<(isize, isize)>,
    visited: &mut HashSet<usize>,
) -> usize {
    let (x, y) = map.icoords_from_pos(pos);
    let mut path_count: usize = 0;
    for (xdir, ydir) in directions.iter() {
        let new_x = x + xdir;
        let new_y = y + ydir;
        match map.get(new_x, new_y) {
            Some(next_val) if *next_val == 9 && i == 8 => {
                let new_pos = map.pos_from_coords(new_x as usize, new_y as usize);
                match visited.contains(&new_pos) {
                    false => {
                        visited.insert(new_pos);
                        path_count += 1;
                    }
                    true => {}
                }
            }
            Some(next_val) if *next_val == i + 1 => {
                let new_pos = map.pos_from_coords(new_x as usize, new_y as usize);
                path_count += traverse(new_pos, map, i + 1, directions, visited);
            }
            Some(_) => {}
            None => {}
        }
    }
    return path_count;
}

fn traverse2(pos: usize, map: &Matrix<u32>, i: u32, directions: &Vec<(isize, isize)>) -> usize {
    let (x, y) = map.icoords_from_pos(pos);
    let mut path_count: usize = 0;
    for (xdir, ydir) in directions.iter() {
        let new_x = x + xdir;
        let new_y = y + ydir;
        match map.get(new_x, new_y) {
            Some(next_val) if *next_val == 9 && i == 8 => {
                path_count += 1;
            }
            Some(next_val) if *next_val == i + 1 => {
                let new_pos = map.pos_from_coords(new_x as usize, new_y as usize);
                path_count += traverse2(new_pos, map, i + 1, directions);
            }
            Some(_) => {}
            None => {}
        }
    }
    return path_count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut data: Vec<u32> = Vec::new();
    let mut cols: usize = 0;
    const RADIX: u32 = 10;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut levels: Vec<u32> = line.chars().map(|x| x.to_digit(RADIX).unwrap()).collect();
            cols = levels.len();
            data.append(&mut levels);
        }
    }

    let map: Matrix<u32> = Matrix { data, cols };
    let directions = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut p1_sum: usize = 0;

    for (pos, point) in map.data.iter().enumerate() {
        let mut visited: HashSet<usize> = HashSet::new();
        if *point == 0 {
            let trailheads = traverse(pos, &map, 0, &directions, &mut visited);
            p1_sum += trailheads;
        }
    }

    println!("P1: {}", p1_sum);

    let mut p2_sum: usize = 0;

    for (pos, point) in map.data.iter().enumerate() {
        if *point == 0 {
            let trailheads = traverse2(pos, &map, 0, &directions);
            p2_sum += trailheads;
        }
    }
    println!("P2: {}", p2_sum);
}

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Div, Rem};
use std::path::Path;

#[derive(Debug, Default)]
pub struct Matrix<T> {
    cols: usize,
    data: Vec<T>,
}

impl<T: Default + Copy + std::fmt::Display> Matrix<T> {
    pub fn new(cols: usize, data: Vec<T>) -> Self {
        Self { cols, data }
    }
    // Returns T, pos, and direction from the origin
    // i=x, k=y
    pub fn neighbours(&self, pos: usize) -> Vec<Neighbour<T>> {
        let ipos = pos as isize;
        //println!("ipos: {}", ipos);
        let (cur_x, cur_y) = self.icoords_from_pos(pos);
        let icols = self.cols as isize;
        let mut neighbours: Vec<Neighbour<T>> = Vec::new();
        for i in -1..=1 {
            for k in -1..=1 {
                let new_row = cur_y + i;
                let new_col = cur_x + k;
                //println!(
                //    "i: {}, k: {}, new_col: {}, new_row: {}",
                //    i, k, new_col, new_row
                //);
                if !(i == 0 && k == 0)
                    && new_row >= 0
                    && new_row < self.data.len().div(self.cols) as isize
                    && new_col >= 0
                    && new_col < icols
                {
                    let new_pos = ipos + icols * i + k;
                    let unew_pos = new_pos as usize;
                    //println!(
                    //    "i: {}, k: {}, new_pos: {}, new_col: {}, index: {} undex: {}",
                    //    i, k, new_pos, new_row, new_pos, unew_pos
                    //);
                    match self.data.get(unew_pos) {
                        Some(n) => {
                            //println!("Creating neighbour, i: {}, k: {}", i, k);
                            let neighbour = Neighbour {
                                c: *n,
                                pos: unew_pos,
                                x_dir: k,
                                y_dir: i,
                            };
                            neighbours.push(neighbour)
                        }
                        _ => {}
                    }
                }
            }
        }
        neighbours
    }

    pub fn get(&self, x: isize, y: isize) -> Option<T> {
        if x >= 0
            && y >= 0
            && x < self.cols.try_into().unwrap()
            && y < self.data.len().div(self.cols).try_into().unwrap()
        {
            let pos = self.pos_from_coords(x.try_into().unwrap(), y.try_into().unwrap());
            if pos >= self.data.len() {
                return None;
            }
            return Some(self.data[pos]);
        }
        None
    }

    pub fn pos_from_coords(&self, x: usize, y: usize) -> usize {
        return &self.cols * y + x;
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

impl fmt::Display for Matrix<char> {
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

#[derive(Debug, Default)]
pub struct Neighbour<T> {
    c: T,
    pos: usize,
    x_dir: isize,
    y_dir: isize,
}

impl Neighbour<char> {
    pub fn solve(&self, data: &Matrix<char>) -> bool {
        if self.c == 'M' {
            match &self.next(&data) {
                Some(a) if a.c == 'A' => {
                    match a.next(&data) {
                        Some(s) if s.c == 'S' => {
                            return true;
                        }
                        _ => return false,
                    };
                }
                _ => return false,
            };
        };
        return false;
    }

    fn next(&self, data: &Matrix<char>) -> Option<Self> {
        let (cur_x, cur_y) = data.coords_from_pos(self.pos);
        let next_x = cur_x as isize + self.x_dir as isize;
        let next_y = cur_y as isize + self.y_dir as isize;

        match data.get(next_x, next_y) {
            Some(next_neighbour) => {
                let next_pos =
                    data.pos_from_coords(next_x.try_into().unwrap(), next_y.try_into().unwrap());
                let next_neighbour = Neighbour {
                    c: next_neighbour,
                    pos: next_pos,
                    x_dir: self.x_dir,
                    y_dir: self.y_dir,
                };
                return Some(next_neighbour);
            }
            None => return None,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn solve2(neighbours: Vec<Neighbour<char>>) -> bool {
    if neighbours.len() < 8 {
        return false;
    };

    let first: Vec<char> = neighbours
        .iter()
        .filter(|n| n.x_dir == -1 && n.y_dir == -1 || n.x_dir == 1 && n.y_dir == 1)
        .map(|x| x.c)
        .collect();
    let second: Vec<char> = neighbours
        .iter()
        .filter(|n| n.x_dir == -1 && n.y_dir == 1 || n.x_dir == 1 && n.y_dir == -1)
        .map(|x| x.c)
        .collect();

    let criteria: Vec<char> = vec!['M', 'S'];

    if criteria.iter().all(|item| first.contains(item))
        && criteria.iter().all(|item| second.contains(item))
    {
        return true;
    }

    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut data = Vec::new();
    let mut rows: usize = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            for c in line.chars() {
                data.push(c);
            }
            rows += 1;
        }
    }

    let cols = data.len().div(rows);
    let puzzle: Matrix<char> = Matrix::new(cols, data);

    let mut count: usize = 0;
    for (i, point) in puzzle.data.iter().enumerate() {
        if *point == 'X' {
            for neighbour in puzzle.neighbours(i) {
                if neighbour.solve(&puzzle) {
                    count += 1;
                }
            }
        }
    }

    let mut count2: usize = 0;

    for (i, point) in puzzle.data.iter().enumerate() {
        if *point == 'A' {
            let neighbours = puzzle.neighbours(i);
            if solve2(neighbours) {
                count2 += 1;
            }
        }
    }

    println!("P1: {}", count);
    println!("P2: {}", count2);
}

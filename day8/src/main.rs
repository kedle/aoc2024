use std::collections::HashMap;
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

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    antenna: char,
    antinodes: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.antenna)?;
        Ok(())
    }
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

    pub fn get(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if self.check_boundaries(x, y) {
            let pos = self.pos_from_coords(x.try_into().unwrap(), y.try_into().unwrap());
            if pos >= self.data.len() {
                return None;
            }
            return Some(&mut self.data[pos]);
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

    //pub fn get_antinode(&self, x: us

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

impl fmt::Display for Matrix<Point> {
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

impl Matrix<Point> {
    fn dbg(&self) -> () {
        for (i, c) in self.data.iter().enumerate() {
            if i.rem(self.cols) == 0 {
                println!("");
            }
            print!("{} ", c);
        }
        println!("");
        for (i, c) in self.data.iter().enumerate() {
            if i.rem(self.cols) == 0 {
                println!("");
            }
            print!("{} ", c.antinodes);
        }
        println!("\n");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn map_antennas(puzzle: &mut Matrix<Point>) -> HashMap<char, Vec<usize>> {
    // Loop through all antennas
    let mut antenna_map: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, point) in puzzle.data.iter().enumerate() {
        if point.antenna != '.' {
            match antenna_map.get_mut(&point.antenna) {
                Some(antenna_locations) => {
                    antenna_locations.push(i);
                }
                None => {
                    antenna_map.insert(point.antenna, vec![i]);
                    ()
                }
            }
        }
    }
    antenna_map
}

fn check_antinodes(puzzle: &mut Matrix<Point>, antennas: &Vec<usize>) -> () {
    if antennas.len() < 2 {
        return ();
    }

    for (i, _antenna1) in antennas.iter().enumerate() {
        for (k, _antenna2) in antennas.iter().enumerate().skip(i + 1) {
            let antenna_pair = (&antennas[i], &antennas[k]);
            check(puzzle, antenna_pair);
        }
    }
    ()
}

fn check(puzzle: &mut Matrix<Point>, pair: (&usize, &usize)) -> () {
    let (x1, y1) = puzzle.icoords_from_pos(*pair.0);
    let (x2, y2) = puzzle.icoords_from_pos(*pair.1);
    let dx = x2 - x1;
    let dy = y2 - y1;
    let delta = *pair.1 - *pair.0;
    let antenna_brand = puzzle.data[*pair.0].antenna;

    // Check backwards
    if *pair.0 > delta {
        if let Some(pos) = puzzle.get(x1 - dx, y1 - dy) {
            if pos.antenna != antenna_brand {
                pos.antinodes += 1;
            }
        }
    }

    // Check forwards
    if let Some(pos) = puzzle.get(x2 + dx, y2 + dy) {
        if pos.antenna != antenna_brand {
            pos.antinodes += 1;
        }
    }

    ()
}

fn check_antinodes_recursive(puzzle: &mut Matrix<Point>, antennas: &Vec<usize>) -> () {
    if antennas.len() < 2 {
        return ();
    }

    for (i, _antenna1) in antennas.iter().enumerate() {
        for (k, _antenna2) in antennas.iter().enumerate().skip(i + 1) {
            let antenna_pair = (&antennas[i], &antennas[k]);
            let (x1, y1) = puzzle.icoords_from_pos(*antenna_pair.0);
            let (x2, y2) = puzzle.icoords_from_pos(*antenna_pair.1);
            let dx = x2 - x1;
            let dy = y2 - y1;
            let delta = *antenna_pair.1 - *antenna_pair.0;
            let antenna_brand = puzzle.data[*antenna_pair.0].antenna;

            if *antenna_pair.0 > delta {
                check_recursive(puzzle, antenna_brand, x1, -dx, y1, -dy);
            }
            check_recursive(puzzle, antenna_brand, x2, dx, y2, dy);
        }
    }
    ()
}

fn check_recursive(
    puzzle: &mut Matrix<Point>,
    antenna: char,
    x: isize,
    dx: isize,
    y: isize,
    dy: isize,
) -> () {
    match puzzle.get(x + dx, y + dy) {
        Some(pos) => {
            if pos.antenna != antenna {
                pos.antinodes += 1;
                let new_x = x + dx;
                let new_y = y + dy;
                check_recursive(puzzle, antenna, new_x, dx, new_y, dy);
            };
            ()
        }
        _ => (),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut data = Vec::new();
    let mut rows: usize = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            for c in line.chars() {
                data.push(Point {
                    antenna: c,
                    antinodes: 0,
                });
            }
            rows += 1;
        }
    }

    let cols = data.len().div(rows);
    let mut puzzle: Matrix<Point> = Matrix::new(cols, data);
    let mut p2puzzle = puzzle.clone();

    let map = map_antennas(&mut puzzle);
    let map2 = map.clone();

    for (_key, value) in map.into_iter() {
        check_antinodes(&mut puzzle, &value);
    }

    let p1_sum: usize = puzzle
        .data
        .iter()
        .map(|location| location.antinodes)
        .collect::<Vec<usize>>()
        .iter()
        .filter(|x| **x > 0)
        .count();

    println!("P1: {:?}", p1_sum);

    for (_key, value) in map2.into_iter() {
        check_antinodes_recursive(&mut p2puzzle, &value);
    }

    let p2_sum: usize = p2puzzle
        .data
        .iter()
        .filter(|x| x.antinodes > 0 || x.antenna != '.')
        .count();

    println!("P2: {:?}", p2_sum);
}

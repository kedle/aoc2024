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

    pub fn get_pos(&self, pos: usize) -> Option<&T> {
        return Some(&self.data[pos as usize]);
    }

    pub fn get_pos_mut(&mut self, pos: usize) -> Option<&mut T> {
        return Some(&mut self.data[pos as usize]);
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

impl fmt::Display for Matrix<Plant> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, c) in self.data.iter().enumerate() {
            if i.rem(self.cols) == 0 && i >= self.cols {
                write!(f, "\n")?;
            }
            write!(f, "{} ", c.kind)?;
        }
        Ok(())
    }
}

impl Matrix<Plant> {
    fn dbg(&self) -> () {
        for (i, c) in self.data.iter().enumerate() {
            if i.rem(self.cols) == 0 {
                println!("");
            }
            print!("[{}]", c.neighbours);
        }
        println!("");
    }
}

#[derive(Clone, Debug, Copy, Default)]
struct Plant {
    kind: char,
    visited: bool,
    neighbours: usize,
    perimeter: usize,
}

impl fmt::Display for Plant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.kind)?;
        Ok(())
    }
}

impl Plant {
    fn new(kind: char) -> Self {
        Plant {
            kind,
            visited: false,
            neighbours: 0,
            perimeter: 0,
        }
    }

    fn visit(&mut self) -> () {
        self.visited = true
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn explore(pos: usize, garden: &mut Matrix<Plant>, directions: &Vec<(isize, isize)>) -> () {
    let (x, y) = garden.icoords_from_pos(pos);

    {
        let mother = garden.get_pos_mut(pos).unwrap();
        mother.visit();
    }

    let kind = garden.get_pos(pos).unwrap().kind;
    let mut neighbour_count: usize = 0;
    for (xdir, ydir) in directions.iter() {
        let new_x = x + xdir;
        let new_y = y + ydir;
        match garden.get(new_x, new_y) {
            Some(plant) if plant.kind == kind => {
                neighbour_count += 1;
            }
            _ => {}
        }
    }
    let mother = garden.get_pos_mut(pos).unwrap();
    mother.neighbours = neighbour_count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut data: Vec<Plant> = Vec::new();
    let mut cols: usize = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut plants: Vec<Plant> = line
                .chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|x| Plant::new(*x))
                .collect();
            cols = plants.len();
            data.append(&mut plants);
        }
    }

    let mut garden: Matrix<Plant> = Matrix {
        data: data.clone(),
        cols,
    };
    let directions = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];
    println!("{}", garden);

    for i in 0..garden.data.len() {
        match garden.get_pos(i) {
            Some(plant) if !plant.visited => {
                explore(i, &mut garden, &directions);
            }
            _ => {}
        }
    }

    let fence_sum = garden.data.iter().fold(0, |fences, plant| {
        let f = 4 - plant.neighbours;
        println!(
            "plant: {}, neighbours: {}, fences: {}",
            plant, plant.neighbours, f
        );
        fences + f
    });

    garden.dbg();

    println!("P1: {}", fence_sum);
}

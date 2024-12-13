use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Div, Rem};
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct Matrix<T> {
    cols: usize,
    data: Vec<T>,
}

impl<T: Default + Copy + std::fmt::Display> Matrix<T> {
    pub fn new(cols: usize, data: Vec<T>) -> Self {
        Self { cols, data }
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

impl Matrix<char> {
    fn dbg(&self) -> () {
        for (i, c) in self.data.iter().enumerate() {
            if i.rem(self.cols) == 0 {
                println!("");
            }
            print!("{} ", c);
        }
    }
}

#[derive(Debug)]
struct Step {
    pos: usize,
    dir: usize, // 0: up, 1: right, 2: down, 3: left
    count: usize,
}

impl Step {
    fn advance(&self) -> (isize, isize) {
        match self.dir {
            0 => (0, -1),
            1 => (1, 0),
            2 => (0, 1),
            3 => (-1, 0),
            _ => panic!("invalid direction"),
        }
    }

    fn turn(&mut self) -> () {
        match self.dir {
            3 => self.dir = 0,
            _ => self.dir += 1,
        }
    }

    fn dir(&self) -> &str {
        match self.dir {
            0 => "up",
            1 => "right",
            2 => "down",
            3 => "left",
            _ => "ananas",
        }
    }
    fn path(&self) -> char {
        match self.dir {
            0 => '|',
            1 => '-',
            2 => '|',
            3 => '-',
            _ => '.',
        }
    }
    fn dirsym(&self) -> char {
        match self.dir {
            0 => '^',
            1 => '>',
            2 => 'v',
            3 => '<',
            _ => '.',
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

fn next(map: &mut Matrix<char>, step: &mut Step, path: &mut HashSet<(usize, usize)>) -> usize {
    let (cur_x, cur_y) = map.icoords_from_pos(step.pos);
    let (dx, dy) = step.advance();
    let next_x = cur_x + dx;
    let next_y = cur_y + dy;

    // We are currently in this step
    map.data[step.pos] = step.dirsym();

    //map.dbg();
    if check_loop(step, path) {
        let coords = &map.icoords_from_pos(step.pos);
        //map.dbg();
        return 0;
    }

    match map.get(next_x, next_y) {
        Some(c) if c == '#' => {
            step.turn();
            map.data[step.pos] = '+';
            //path.insert((step.pos, step.dir));
            let step_count = next(map, step, path);
            return step_count;
        }
        Some(_) => {
            let mut next_step = Step {
                pos: map.pos_from_coords(next_x as usize, next_y as usize),
                dir: step.dir,
                count: step.count + 1,
            };
            map.data[step.pos] = step.path();
            path.insert((step.pos, step.dir));
            let step_count = next(map, &mut next_step, path);
            return step_count;
        }
        None => {
            map.data[step.pos] = step.path();
            path.insert((step.pos, step.dir));
            return step.count;
        }
    };
}

fn check_loop(step: &mut Step, path: &HashSet<(usize, usize)>) -> bool {
    match path.get(&(step.pos, step.dir)) {
        Some(_) => {
            //println!("loop");
            return true;
        }
        None => return false,
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
                data.push(c);
            }
            rows += 1;
        }
    }

    let cols = data.len().div(rows);
    let puzzle: Matrix<char> = Matrix::new(cols, data);

    let mut p1puzzle = puzzle.clone();

    let guard = puzzle.data.iter().position(|&c| c == '^').unwrap();
    let mut start = Step {
        pos: guard,
        dir: 0,
        count: 0,
    };

    let mut p1path: HashSet<(usize, usize)> = HashSet::new();
    next(&mut p1puzzle, &mut start, &mut p1path);

    let count = p1puzzle
        .data
        .iter()
        .fold(0, |sum, &e| if e == 'X' { sum + 1 } else { sum + 0 });
    println!("P1: {:?}", count);

    let mut p2count: usize = 0;

    for (i, point) in puzzle.data.iter().enumerate() {
        println!("i: {}", i);
        let mut path: HashSet<(usize, usize)> = HashSet::new();

        if *point == '.' {
            let mut pusle = puzzle.clone();
            pusle.data[i] = '#';
            //println!("{}", i);

            let mut start = Step {
                pos: guard,
                dir: 0,
                count: 0,
            };

            let steps = next(&mut pusle, &mut start, &mut path);
            if steps == 0 {
                p2count += 1;
            }
        }
    }

    // Debug pusle nro 63
    //let mut pusle = puzzle.clone();
    //pusle.data[63] = '#';
    //
    //let steps = next(&mut pusle, &mut start, &mut path);

    println!("P2:: {}", p2count);
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Pair {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Game {
    a: Pair,
    b: Pair,
    prize: Pair,
}

impl Game {
    fn check_win(&self, na: isize) -> isize {
        let remx = self.prize.x - (na * self.a.x);
        let remy = self.prize.y - (na * self.a.y);

        if remx % self.b.x == 0 && remy % self.b.y == 0 {
            let nb_x = remx / self.b.x;
            let nb_y = remy / self.b.y;

            if nb_x == nb_y && nb_x >= 0 && nb_x <= 100 {
                let cost = 3 * na + nb_x;
                return cost;
            }
        }
        -1
    }

    fn play(&self) -> isize {
        let mut wins: Vec<isize> = Vec::new();
        for na in 0..=100 {
            let cost = self.check_win(na);
            if cost != -1 {
                wins.push(cost);
            }
        }

        match wins.iter().min() {
            Some(v) => return *v,
            None => return -1 as isize,
        }
    }
}

fn parse_values(line: &str) -> Pair {
    let coords_part = line.split(':').nth(1).unwrap().trim();

    let parts: Vec<&str> = coords_part.split(',').map(|p| p.trim()).collect();

    fn parse_coord(segment: &str) -> isize {
        if let Some((_, val)) = segment.split_once('=') {
            val.parse().unwrap()
        } else if let Some((_, val)) = segment.split_once('+') {
            val.parse().unwrap()
        } else {
            panic!("Something else");
        }
    }

    let x = parse_coord(parts[0]);
    let y = parse_coord(parts[1]);

    Pair { x, y }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut strlines: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            if !line.is_empty() {
                strlines.push(line);
            }
        }
    }

    let mut games: Vec<Game> = Vec::new();

    for piece in strlines.chunks(3) {
        let a = parse_values(&piece[0]);
        let b = parse_values(&piece[1]);
        let prize = parse_values(&piece[2]);
        let game = Game { a, b, prize };
        games.push(game);
    }

    let p1 = games
        .iter()
        .map(|game| game.play())
        .filter(|game| *game != -1)
        .fold(0, |sum, game| sum + game);

    println!("P1: {}", p1);
}

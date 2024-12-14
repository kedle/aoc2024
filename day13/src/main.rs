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
    fn play(&self, part1: bool) -> isize {
        let a = self.a.x as isize;
        let b = self.b.x as isize;
        let c = self.a.y as isize;
        let d = self.b.y as isize;
        let x = self.prize.x as isize;
        let y = self.prize.y as isize;

        let det = a * d - b * c;
        if det == 0 {
            return -1;
        }

        let na_num = x * d - b * y;
        let nb_num = a * y - x * c;

        if na_num % det != 0 || nb_num % det != 0 {
            return -1;
        }

        let na = na_num / det;
        let nb = nb_num / det;

        if part1 {
            if na < 0 || na > 100 || nb < 0 || nb > 100 {
                return -1;
            }
        }

        let cost = 3 * na + nb;
        cost
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
        .map(|game| game.play(true))
        .filter(|game| *game != -1)
        .fold(0, |sum, game| sum + game);

    println!("P1: {}", p1);

    let p2 = games
        .iter_mut()
        .map(|game| {
            game.prize.x += 10000000000000;
            game.prize.y += 10000000000000;
            game
        })
        .map(|game| game.play(false))
        .filter(|game| *game != -1)
        .fold(0, |sum, game| sum + game);

    println!("P2: {:?}", p2);
}

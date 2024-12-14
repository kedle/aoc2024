use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Div;
use std::path::Path;
const WIDTH: isize = 101;
const HEIGHT: isize = 103;

#[derive(Default, Clone, Debug)]
struct Robot {
    x: isize,
    vx: isize,
    y: isize,
    vy: isize,
}

impl Robot {
    fn swarm(&mut self) -> () {
        self.x = (self.x + self.vx).rem_euclid(WIDTH);
        self.y = (self.y + self.vy).rem_euclid(HEIGHT);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_values(line: &str) -> Robot {
    let mut parts = line.split_whitespace();
    let pos_part = parts.next().unwrap();
    let vel_part = parts.next().unwrap();

    let parse_part = |part: &str, prefix: &str| -> (isize, isize) {
        let stripped = part.strip_prefix(prefix).unwrap();
        let mut split = stripped.split(',');
        let a = split.next().unwrap().parse::<isize>().unwrap();
        let b = split.next().unwrap().parse::<isize>().unwrap();
        (a, b)
    };

    let (x, y) = parse_part(pos_part, "p=");
    let (vx, vy) = parse_part(vel_part, "v=");

    Robot { x, y, vx, vy }
}

fn safety_factor(robots: &Vec<Robot>) -> usize {
    let mut top_left: usize = 0;
    let mut top_right: usize = 0;
    let mut bot_left: usize = 0;
    let mut bot_right: usize = 0;

    let vdiv = WIDTH.div(2);
    let hdiv = HEIGHT.div(2);

    for robot in robots.iter() {
        if robot.x < vdiv && robot.y < hdiv {
            top_left += 1;
        } else if robot.x > vdiv && robot.y < hdiv {
            top_right += 1;
        } else if robot.x < vdiv && robot.y > hdiv {
            bot_left += 1;
        } else if robot.x > vdiv && robot.y > hdiv {
            bot_right += 1;
        }
    }

    return top_left * top_right * bot_left * bot_right;
}

fn find_the_tree(robots: &Vec<Robot>) -> bool {
    let mut bathroom = vec![false; (WIDTH * HEIGHT) as usize];
    for robot in robots {
        let pos = robot.y * WIDTH + robot.x;
        bathroom[pos as usize] = true;
    }

    let mut robot_counter: usize = 0;
    for robot in bathroom {
        if robot_counter == 30 {
            return true;
        }
        if robot {
            robot_counter += 1;
        } else {
            robot_counter = 0;
        }
    }

    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut robots: Vec<Robot> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let robot = parse_values(&line);
            robots.push(robot);
        }
    }

    let mut p2_robots = robots.clone();

    for _i in 0..100 {
        for robot in robots.iter_mut() {
            robot.swarm();
        }
    }
    let p1 = safety_factor(&robots);
    println!("P1: {}", p1);

    let mut p2_answer: usize = 0;
    loop {
        for robot in p2_robots.iter_mut() {
            robot.swarm();
        }
        p2_answer += 1;
        if find_the_tree(&p2_robots) {
            println!("P2: {}", p2_answer);
            break;
        }
    }
}

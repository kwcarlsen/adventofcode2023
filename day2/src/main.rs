use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Game {
    id: i32,
    blue: i32,
    red: i32,
    green: i32,
}

impl Game {
    fn new(id: i32, blue: i32, red: i32, green: i32) -> Game {
        Game {
            id: id,
            blue: blue,
            red: red,
            green: green,
        }
    }

    fn empty() -> Game {
        Game {
            id: 0,
            blue: 0,
            red: 0,
            green: 0,
        }
    }
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Vec<Game> {
    // println!("{}", line);
    let mut token = line.split(": ");
    let game_id_token = token.next().unwrap();
    let games_token = token.next().unwrap();
    token = game_id_token.split(" ");
    token.next();
    let id = token.next().unwrap().parse::<i32>().unwrap();
    // println!("id {}", id);

    let mut set = Vec::new();
    for game_token in games_token.split(";") {
        let mut game = Game {
            id: id,
            blue: 0,
            red: 0,
            green: 0,
        };
        for balls_token in game_token.split(", ") {
            let mut ball_token = balls_token.trim().split(" ");
            // println!("{}", balls_token);
            let amount = ball_token.next().unwrap().parse::<i32>().unwrap();
            let color = ball_token.next().unwrap();
            match color {
                "blue" => {
                    game.blue = amount;
                }
                "red" => {
                    game.red = amount;
                }
                "green" => {
                    game.green = amount;
                }
                _ => panic!("Invalid color {}", color),
            }
        }
        set.push(game);
    }
    set
}

fn solution(file: &str) -> i32 {
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        'outer: for line in lines.flatten() {
            let set = parse_line(&line);
            for g in set.iter() {
                if g.blue > 14 || g.red > 12 || g.green > 13 {
                    continue 'outer;
                }
            }
            sum += set[0].id;
        }
    }
    sum
}

fn solution2(file: &str) -> i32 {
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let mut minimum_set = Game::empty();
            let set = parse_line(&line);
            for g in set.iter() {
                minimum_set.blue = minimum_set.blue.max(g.blue);
                minimum_set.green = minimum_set.green.max(g.green);
                minimum_set.red = minimum_set.red.max(g.red);
            }
            sum += minimum_set.blue * minimum_set.green * minimum_set.red;
        }
    }
    sum
}

fn main() {
    println!("Example: {}", solution("example.txt"));
    println!("Solution: {}", solution("input.txt"));
    println!("Solution2: {}", solution2("input.txt"));
}

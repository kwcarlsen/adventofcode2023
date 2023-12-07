use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    picked_numbers: Vec<i32>,
    value: i32
}

impl Card {
    fn from_str(line: &str) -> Card {
        let re = regex::Regex::new(r"^Card\W+\d+: ([^\|]+) \| (.+)$").unwrap();
        let caps = re.captures(line).unwrap();
        // println!("{:?}", &caps[1]);
        // println!("{:?}", &caps[2]);
        Card {
            winning_numbers: caps[1]
                .replace("  ", " ")
                .trim()
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
            picked_numbers: caps[2]
                .replace("  ", " ")
                .trim()
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
            value: 0
        }
    }

    fn get_winning_numbers(&self) -> u32 {
        let mut s = 0;
        for i in self.winning_numbers.iter() {
            if self.picked_numbers.contains(i) {
                s += 1;
            }
        }
        s
    }
}

fn solution(file: &str) -> i32 {
    let mut sum = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let card = Card::from_str(&line);
            let w = card.get_winning_numbers();
            if w > 0 {
                sum += 2_i32.pow(w-1);
            }
            // println!("{} {}", line, p);
            // println!("{:?}", card);
        }
    }
    sum
}

fn solution2(file: &str) -> i32 {
    let mut cards = Vec::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let card = Card::from_str(&line);
            cards.push(card)
        }
    }
    let mut sum = 0;
    for i in (0..cards.len()).rev() {
        sum += solve(&mut cards, i);
        // println!("{}: {}", i, sum);
    }
    sum
}

fn solve(cards: &mut Vec<Card>, card: usize) -> i32 {
    let winning_numbers = cards[card].get_winning_numbers() as usize;
    let mut sum = 1;
    for i in 0..winning_numbers {
        sum += cards[card+i+1].value;
    }
    cards[card].value = sum;
    sum
}
    
fn main() {
    println!("Example: {}", solution("example.txt"));
    println!("Solution: {}", solution("input.txt"));
    println!("Example2: {}", solution2("example.txt"));
    println!("Solution2: {}", solution2("input.txt"));
}

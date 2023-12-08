use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Flatten;
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Map {
    translation: Vec<Translation>,
}

impl Map {
    fn from_file(lines: &mut Flatten<std::io::Lines<BufReader<File>>>) -> Option<Map> {
        if let None = lines.next() {
            return None;
        }
        let mut translation = Vec::new();
        while let Some(t) = Translation::from_file(lines) {
            translation.push(t);
        }
        Some(Map { translation })
    }

    fn translate(&self, source: i64) -> Option<i64> {
        for t in self.translation.iter() {
            if let Some(destination) = t.translate(source) {
                return Some(destination);
            }
        }
        Some(source)
    }
}

#[derive(Debug)]

struct Translation {
    source: i64,
    destination: i64,
    length: i64,
}

impl Translation {
    fn translate(&self, source: i64) -> Option<i64> {
        let t = source - self.source;
        if t < 0 || t > self.length {
            return None;
        }
        Some(self.destination + t)
    }

    fn from_file(file: &mut Flatten<std::io::Lines<BufReader<File>>>) -> Option<Translation> {
        if let Some(l) = file.next() {
            if l.is_empty() {
                return None;
            }
            // println!("{:?}", l);
            let re = regex::Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
            let caps = re.captures(&l).unwrap();

            let translation = Translation {
                source: caps[2].parse::<i64>().unwrap(),
                destination: caps[1].parse::<i64>().unwrap(),
                length: caps[3].parse::<i64>().unwrap(),
            };
            return Some(translation);
        }
        None
    }
}

fn parse_seeds(seeds: String) -> Vec<i64> {
    let mut s = seeds.split(": ");
    s.next();
    s.next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn solution(file: &str) -> i64 {
    if let Ok(lines) = read_lines(file) {
        let mut l = lines.flatten();
        let seeds = parse_seeds(l.next().unwrap());
        l.next();

        let mut maps = Vec::new();
        // println!("{:?}", seeds);
        while let Some(m) = Map::from_file(&mut l) {
            maps.push(m);
        }

        let mut locations = Vec::new();
        for s in seeds {
            let mut v = s;
            for m in maps.iter() {
                if let Some(d) = m.translate(v) {
                    // println!("{:?}", m);
                    // println!("{} -> {}", v, d);
                    v = d;
                }
            }
            locations.push(v);
            // println!("Seed {} -> Location {}", s, v);
        }
        return *locations.iter().min().unwrap();
    }
    0
}

fn solution2(file: &str) -> i64 {
    if let Ok(lines) = read_lines(file) {
        let mut l = lines.flatten();
        let seeds = parse_seeds(l.next().unwrap());
        l.next();

        let mut maps = Vec::new();
        // println!("{:?}", seeds);
        while let Some(m) = Map::from_file(&mut l) {
            maps.push(m);
        }

        let mut i = seeds.into_iter();
        let mut min = None;
        while let Some(s) = i.next() {
            let r = i.next().unwrap()+1;
            for x in 0..r {
                let mut v = s + x;
                for m in maps.iter() {
                    if let Some(d) = m.translate(v) {
                        // println!("{:?}", m);
                        // println!("{} -> {}", v, d);
                        v = d;
                    }
                }
                if min.is_none() || v < min.unwrap() {
                    min = Some(v);
                }
                // println!("Seed {} -> Location {}", s, v);
            }
        }
        return min.unwrap();
    }
    0
}

fn main() {
    println!("Example: {}", solution("example.txt"));
    println!("Solution: {}", solution("input.txt"));
    println!("Example2: {}", solution2("example.txt"));
    println!("Solution2: {}", solution2("input.txt"));
}

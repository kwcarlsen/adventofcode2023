use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn read_digit(line: &[char], x: usize) -> usize {
    let mut number = 0;
    for i in line[x..].iter() {
        if !i.is_ascii_digit() {
            break;
        }
        number = number * 10 + i.to_digit(10).unwrap() as usize;
    }
    number
}

fn print_map(map: &Vec<Vec<char>>) {
    for i in map.iter() {
        for j in i.iter() {
            print!("{}", j);
        }
        println!();
    }
    println!();
}

fn check_position_for_symbol(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    if x <= 0 || y <= 0 {
        return false;
    }
    let xu = x as usize;
    let yu = y as usize;
    if xu >= map.len() || yu >= map[xu].len() {
        return false;
    }

    if map[yu][xu] == '.' {
        return false;
    }
    if map[yu][xu].is_ascii_digit() {
        return false;
    }
    true
}

fn check_position_for_gear(map: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    if x <= 0 || y <= 0 {
        return false;
    }
    let xu = x as usize;
    let yu = y as usize;
    if xu >= map.len() || yu >= map[xu].len() {
        return false;
    }

    if map[yu][xu] == '*' {
        return true;
    }
    
    false
}

fn scan_for_symbol(map: &Vec<Vec<char>>, x: i32, y: i32, length: i32) -> bool {
    let index_start = x - 1;
    let index_end = x + length + 1;

    for j in index_start..index_end {
        if check_position_for_symbol(map, j, y - 1) {
            // println!("1: found {} as {},{} (started at {}, {}) ", map[(y - 1) as usize][j as usize], j, y - 1, x,y);
            return true;
        }

        if check_position_for_symbol(map, j, y + 1) {
            // println!("2: found {} as {},{} ", map[(y + 1) as usize][j as usize], j, y + 1);
            return true;
        }
    }
    if check_position_for_symbol(map, x - 1, y) {
        // println!("3: found {} as {},{} ", map[(y+1) as usize][(x-1) as usize], x - 1, y + 1);
        return true;
    }
    if check_position_for_symbol(map, x + length, y) {
        // println!("4: found {} as {},{} ", map[(y + 1) as usize][(x+length+1) as usize], x+length, y);
        return true;
    }

    false
}

fn scan_for_gear(map: &Vec<Vec<char>>, x: i32, y: i32, length: i32) -> Vec<(usize, usize)> {
    let index_start = x - 1;
    let index_end = x + length + 1;
    let mut gears = Vec::new();

    for j in index_start..index_end {
        if check_position_for_gear(map, j, y - 1) {
            // println!("1: found {} as {},{} (started at {}, {}) ", map[(y - 1) as usize][j as usize], j, y - 1, x,y);
            gears.push((j as usize, (y - 1) as usize));
        }

        if check_position_for_gear(map, j, y + 1) {
            // println!("2: found {} as {},{} ", map[(y + 1) as usize][j as usize], j, y + 1);
            gears.push((j as usize, (y + 1) as usize));
        }
    }
    if check_position_for_gear(map, x - 1, y) {
        println!("3: found {} as {},{} ", map[y as usize][(x-1) as usize], x - 1, y);
        gears.push(((x-1) as usize, (y) as usize));
    }
    if check_position_for_gear(map, x + length, y) {
        println!("4: found {} at {},{} ", map[y as usize][(x+length) as usize], x+length, y);
        gears.push(((x+length) as usize, (y) as usize));
    }

    if gears.len() == 2 {
        println!("Found gears: {:?}", gears);
    }

    gears
}

fn solution(file: &str) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            map.push(parse_line(&line));
        }
    }
    print_map(&map);
    let mut sum = 0;
    let mut i = 0;
    while i < map.len() {
        let mut j = 0;
        while j < map[i].len() {
            if map[i][j].is_ascii_digit() {
                let number = read_digit(&map[i], j);
                println!("{}", number);
                if scan_for_symbol(&map, j as i32, i as i32, number.to_string().len() as i32) {
                    sum += number;
                }
                j += number.to_string().len() - 1;
            } 
            j += 1;
        }
        i += 1;
    }
    sum
}

fn solution2(file: &str) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            map.push(parse_line(&line));
        }
    }
    print_map(&map);
    let mut i = 0;
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    while i < map.len() {
        let mut j = 0;
        while j < map[i].len() {
            if map[i][j].is_ascii_digit() {
                let number = read_digit(&map[i], j);
                for (x,y) in scan_for_gear(&map, j as i32, i as i32, number.to_string().len() as i32) {
                    if let Some(gear) = gears.get_mut(&(x, y)) {
                        gear.push(number);
                    } else {
                        gears.insert((x, y), vec![number]);
                    }
                }
                j += number.to_string().len() - 1;
            } 
            j += 1;
        }
        i += 1;
    }

    let mut sum = 0;
    for gear in gears {
        if gear.1.len() == 2 {
            println!("{:?}", gear);
            sum += gear.1.iter().product::<usize>();
        }
    }
    sum
}

fn main() {
    // println!("Example: {}", solution("example.txt"));
    // println!("Solution: {}", solution("input.txt"));
    // println!("Example2: {}", solution2("example.txt"));
    println!("Solution2: {}", solution2("input.txt"));
}

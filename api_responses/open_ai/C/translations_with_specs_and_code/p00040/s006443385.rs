use std::io::{self, BufRead};

fn affine(c: char, a: i32, b: i32) -> char {
    let offset = (a * (c as i32 - 'a' as i32) + b) % 26;
    (offset + 'a' as i32) as u8 as char
}

fn this_check(s: &str, a: i32, b: i32) -> bool {
    s.len() >= 4 && 
    affine(s.chars().nth(0).unwrap(), a, b) == 's' &&
    affine(s.chars().nth_back(1).unwrap(), a, b) == 'i' &&
    affine(s.chars().nth_back(2).unwrap(), a, b) == 'h' &&
    affine(s.chars().nth_back(3).unwrap(), a, b) == 't'
}

fn that_check(s: &str, a: i32, b: i32) -> bool {
    s.len() >= 4 && 
    affine(s.chars().nth(0).unwrap(), a, b) == 't' &&
    affine(s.chars().nth_back(1).unwrap(), a, b) == 'a' &&
    affine(s.chars().nth_back(2).unwrap(), a, b) == 'h' &&
    affine(s.chars().nth_back(3).unwrap(), a, b) == 't'
}

fn search(line: &str, a: i32, b: i32) -> bool {
    for (i, c) in line.chars().enumerate().skip(3) {
        if c == ' ' {
            continue;
        }
        let y = affine(c, a, b);
        if (y == 't' && that_check(&line[i..], a, b)) || (y == 's' && this_check(&line[i..], a, b)) {
            return true;
        }
    }
    false
}

fn print_affine_line(s: &str, a: i32, b: i32) {
    for c in s.chars() {
        if c == ' ' {
            print!(" ");
        } else {
            print!("{}", affine(c, a, b));
        }
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        for a in 0..100 {
            for b in 0..100 {
                if search(&line, a, b) {
                    print_affine_line(&line, a, b);
                    return; // Exit after the first match
                }
            }
        }
    }
}

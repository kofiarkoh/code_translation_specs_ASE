use std::io::{self, BufRead};

fn affine(c: char, a: i32, b: i32) -> char {
    // Perform the affine transformation and wrap around using modulo 26
    let c_index = c as i32 - 'a' as i32;
    let transformed = (a * c_index + b) % 26;
    (transformed + 'a' as i32) as u8 as char
}

fn this_check(s: &[char], a: i32, b: i32) -> bool {
    // Check the last four characters for the string "shit"
    s.get(0) == Some(&'s') && 
    s.get(s.len().wrapping_sub(1)) == Some(&affine('i', a, b)) &&
    s.get(s.len().wrapping_sub(2)) == Some(&affine('h', a, b)) &&
    s.get(s.len().wrapping_sub(3)) == Some(&affine('t', a, b))
}

fn that_check(s: &[char], a: i32, b: i32) -> bool {
    // Check the last four characters for the string "that"
    s.get(0) == Some(&'t') && 
    s.get(s.len().wrapping_sub(1)) == Some(&affine('a', a, b)) &&
    s.get(s.len().wrapping_sub(2)) == Some(&affine('h', a, b)) &&
    s.get(s.len().wrapping_sub(3)) == Some(&affine('t', a, b))
}

fn search(line: &[char], a: i32, b: i32) -> bool {
    for i in 3..line.len() {
        if line[i] == ' ' {
            continue;
        }
        let y = affine(line[i], a, b);
        if (y == 't' && that_check(&line[i..], a, b)) || (y == 's' && this_check(&line[i..], a, b)) {
            return true;
        }
    }
    false
}

fn print_affine_line(s: &[char], a: i32, b: i32) {
    for &c in s {
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
    
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let line_chars: Vec<char> = line.chars().collect();
        
        for a in 0..100 {
            for b in 0..100 {
                if search(&line_chars, a, b) {
                    print_affine_line(&line_chars, a, b);
                    return;
                }
            }
        }
    }
}

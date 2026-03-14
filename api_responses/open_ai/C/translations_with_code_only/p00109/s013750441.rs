use std::io::{self, BufRead};

fn expr(s: &mut &str) -> i32 {
    let mut val = term(s);
    while let Some(&c) = s.chars().next() {
        if c == '+' {
            *s = &s[1..]; // consume '+'
            val += term(s);
        } else if c == '-' {
            *s = &s[1..]; // consume '-'
            val -= term(s);
        } else {
            break;
        }
    }
    val
}

fn term(s: &mut &str) -> i32 {
    let mut val = factor(s);
    while let Some(&c) = s.chars().next() {
        if c == '*' {
            *s = &s[1..]; // consume '*'
            val *= factor(s);
        } else if c == '/' {
            *s = &s[1..]; // consume '/'
            val /= factor(s);
        } else {
            break;
        }
    }
    val
}

fn factor(s: &mut &str) -> i32 {
    if let Some(&c) = s.chars().next() {
        if c == '(' {
            *s = &s[1..]; // consume '('
            let val = expr(s);
            *s = &s[1..]; // consume ')'
            return val;
        }
    }
    number(s)
}

fn number(s: &mut &str) -> i32 {
    let mut val = 0;
    while let Some(&c) = s.chars().next() {
        if c >= '0' && c <= '9' {
            val = val * 10 + (c as i32 - '0' as i32);
            *s = &s[1..]; // consume digit
        } else {
            break;
        }
    }
    val
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n {
        let s = lines.next().unwrap().unwrap();
        let mut p = &s[..];
        println!("{}", expr(&mut p));
    }
}

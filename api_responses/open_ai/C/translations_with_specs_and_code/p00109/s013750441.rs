use std::io::{self, BufRead};

fn number(s: &mut &str) -> i32 {
    let mut val = 0;
    while let Some(c) = s.chars().next() {
        if c.is_digit(10) {
            val = val * 10 + c.to_digit(10).unwrap() as i32;
            *s = &s[1..]; // Move the slice forward
        } else {
            break;
        }
    }
    val
}

fn factor(s: &mut &str) -> i32 {
    if s.starts_with('(') {
        *s = &s[1..]; // Skip '('
        let val = expr(s);
        *s = &s[1..]; // Skip ')'
        return val;
    }
    number(s)
}

fn term(s: &mut &str) -> i32 {
    let mut val = factor(s);
    while let Some(&c) = s.chars().next() {
        if c == '*' {
            *s = &s[1..]; // Skip '*'
            val *= factor(s);
        } else if c == '/' {
            *s = &s[1..]; // Skip '/'
            val /= factor(s);
        } else {
            break;
        }
    }
    val
}

fn expr(s: &mut &str) -> i32 {
    let mut val = term(s);
    while let Some(&c) = s.chars().next() {
        if c == '+' {
            *s = &s[1..]; // Skip '+'
            val += term(s);
        } else if c == '-' {
            *s = &s[1..]; // Skip '-'
            val -= term(s);
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

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..n {
        let expression = lines.next().unwrap().unwrap();
        let mut s = expression.as_str();
        println!("{}", expr(&mut s));
    }
}

use std::io::{self, BufRead};

const DIGIT: fn(usize) -> f64 = |j| 10f64.powi((100 - 1 - j) as i32);

fn solved(num: &str, i: i32) -> i32 {
    let mut s = 0;
    let len = num.len();
    for (j, c) in num.chars().enumerate() {
        let digit_value = if c == 'X' {
            i * DIGIT(len - 1 - j) as i32
        } else {
            (c as i32 - '0' as i32) * DIGIT(len - 1 - j) as i32
        };
        s += digit_value;
    }
    s
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    for line in reader.lines() {
        let calc = line.unwrap();
        let parts: Vec<&str> = calc.split(&['+', '=', ','][..]).collect();
        
        let f = parts[0];
        let s = parts[1];
        let ans = parts[2];

        let mut found = false;
        for i in 0..10 {
            if solved(f, i) + solved(s, i) == solved(ans, i) {
                println!("{}", i);
                found = true;
                break;
            }
        }
        if !found {
            println!("NA");
        }
    }
}

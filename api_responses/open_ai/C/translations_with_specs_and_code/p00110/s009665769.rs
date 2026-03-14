use std::io::{self, BufRead};

fn solved(num: &str, i: u32) -> u32 {
    let mut s = 0;
    let len = num.len();
    for (j, ch) in num.chars().enumerate() {
        let digit_value = if ch == 'X' {
            i * 10u32.pow((len - 1 - j) as u32)
        } else {
            (ch.to_digit(10).unwrap()) * 10u32.pow((len - 1 - j) as u32)
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

        if parts.len() != 3 {
            // This case should not happen according to the spec, so we assume input is well-formed.
            continue;
        }

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

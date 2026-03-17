use std::io::{self, Write};

fn func(s: &str, n: usize, m: usize) -> i32 {
    let mut sum = 0;
    let mut p = s;

    for i in 0..n {
        if m & (1 << i) != 0 {
            let t = &s[i..=i]; // Get the character at index i
            let new_s = format!("{}{}", &s[..i], &s[i + 1..]); // Create a new string without the character
            sum += t.parse::<i32>().unwrap_or(0); // Convert to integer and add to sum
            p = &new_s; // Update p to the new string
        }
    }
    sum + p.parse::<i32>().unwrap_or(0) // Add the last part of the string
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        let mut parts = input.split_whitespace();
        
        let target: i32 = match parts.next() {
            Some(num) => num.parse().unwrap_or(0),
            None => break,
        };

        let paper: String = match parts.next() {
            Some(s) => s.to_string(),
            None => break,
        };

        if target <= 0 || paper.is_empty() || paper.len() > 10 {
            break; // Exit if preconditions are not met
        }

        let n = paper.len();
        let mut res = 0;
        let mut best = -1;
        let mut cnt = 0;

        for i in (1 << (n - 1))..(1 << n) {
            let t = func(&paper, n, i);
            if t <= target {
                if best < t {
                    best = t;
                    res = i;
                    cnt = 1;
                } else if best == t {
                    cnt += 1;
                }
            }
        }

        if cnt != 1 {
            if cnt == 0 {
                writeln!(stdout, "error").expect("Failed to write to stdout");
            } else {
                writeln!(stdout, "rejected").expect("Failed to write to stdout");
            }
        } else {
            write!(stdout, "{} ", best).expect("Failed to write to stdout");
            for i in 0..n {
                write!(stdout, "{}", paper.chars().nth(i).unwrap()).expect("Failed to write to stdout");
                if res & (1 << i) != 0 {
                    if i == n - 1 {
                        writeln!(stdout).expect("Failed to write to stdout");
                    } else {
                        write!(stdout, " ").expect("Failed to write to stdout");
                    }
                }
            }
        }
    }
}

use std::io::{self, BufRead};

const M: i64 = 998244353;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    // Read the first input which indicates the number of subsequent integers
    let first_line = input.next().unwrap().unwrap();
    let mut s: i64 = first_line.trim().parse().expect("Invalid input");
    
    let mut d = vec![0; 90310];
    let mut e = vec![0; 90310];
    let mut r: i64 = 1;

    while let Some(line) = input.next() {
        match line {
            Ok(value) => {
                if let Ok(a) = value.trim().parse::<i64>() {
                    // Update the arrays and result based on the input integer `a`
                    for t in (0..=s).rev() {
                        d[(t + a) as usize] = (d[(t + a) as usize] + d[t as usize] * 2 % M) % M;
                        e[(t + a) as usize] = (e[(t + a) as usize] + e[t as usize]) % M;
                    }
                    s += a;
                    r = r * 3 % M;
                } else {
                    break; // Terminate on non-integer input
                }
            }
            Err(_) => break, // Terminate on read error
        }
    }

    // Final result computation
    for t in (0..=(s / 2)).rev() {
        r = (r - d[t as usize] * 3) % M;
    }
    if s % 2 != 0 {
        r = (r + M - e[(s / 2) as usize] * 3 % M) % M;
    }

    println!("{}", r);
}

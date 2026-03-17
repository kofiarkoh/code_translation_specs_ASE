/*
  AOJ 1241
  Title: Lagrange's Four-Square Theorem
  @kankichi573
*/

use std::io::{self, BufRead};

fn solve(n: i32, c: i32, low: i32) -> i32 {
    let mut ret = 0;
    let mut i = low;

    while i * i <= n {
        if n == i * i {
            ret += 1;
        } else if c < 4 {
            ret += solve(n - i * i, c + 1, i);
        }
        i += 1;
    }
    ret
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let n: i32 = match line {
            Ok(num) => match num.trim().parse() {
                Ok(n) if n > 0 => n,
                _ => continue, // Ignore invalid input
            },
            Err(_) => continue, // Ignore read errors
        };

        if n == 0 {
            break; // Terminate on zero
        }

        let ret = solve(n, 1, 1);
        println!("{}", ret);
    }
}

use std::io::{self, BufRead};

fn main() {
    let mut p = vec![0; 80000];
    p[0] = 2;
    let mut n = 1;

    // Generate prime numbers up to 60000
    for a in 3..60000 {
        let mut is_prime = true;
        for b in 0..n {
            if a % p[b] == 0 {
                is_prime = false;
                break;
            }
            if a / p[b] < p[b] {
                p[n] = a;
                n += 1;
                is_prime = false;
                break;
            }
        }
        if is_prime {
            p[n] = a;
            n += 1;
        }
    }
    p[n] = 60000;

    // Read input and process
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        let line = line.unwrap();
        let a: i32 = line.trim().parse().unwrap();

        let mut n = 0;
        while a >= p[n] {
            n += 1;
        }

        if p[n - 1] == a {
            println!("{} {}", p[n - 2], p[n]);
        } else {
            println!("{} {}", p[n - 1], p[n]);
        }
    }
}

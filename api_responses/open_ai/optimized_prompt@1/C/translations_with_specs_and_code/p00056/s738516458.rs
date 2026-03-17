use std::io::{self, BufRead};

const MAX: usize = 50001;

fn make_prime_table() -> Vec<bool> {
    let mut table = vec![false; MAX];
    
    for i in 2..MAX {
        if !table[i] {
            for j in 2..(MAX / i) {
                table[i * j] = true;
            }
        }
    }
    table
}

fn count_prime_pairs(n: usize, ptable: &[bool]) -> usize {
    let mut count = 0;
    for i in 2..=n / 2 {
        if !ptable[i] && !ptable[n - i] {
            count += 1;
        }
    }
    count
}

fn main() {
    let ptable = make_prime_table();
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                if let Ok(n) = input.trim().parse::<usize>() {
                    if n == 0 {
                        break;
                    } else {
                        let count = count_prime_pairs(n, &ptable);
                        println!("{}", count);
                    }
                }
            }
            Err(_) => {
                // If input is not valid, we simply ignore it as per the spec.
            }
        }
    }
}

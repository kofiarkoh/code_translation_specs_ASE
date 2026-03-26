use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: u64 = iter.next().unwrap().parse().expect("Invalid input for n");
    let mut p: u64 = iter.next().unwrap().parse().expect("Invalid input for p");

    // Preconditions check
    if n == 0 {
        panic!("n must be greater than 0");
    }

    let mut ans: u64 = 1;

    // Factorization loop
    let mut i = 2;
    while i * i <= p {
        let mut cnt = 0;
        while p % i == 0 {
            cnt += 1;
            p /= i;
        }
        if cnt >= n {
            ans *= i.pow((cnt / n) as u32);
        }
        i += 1;
    }

    // Handle the case where p is a prime number greater than 1
    if p > 1 && n == 1 {
        ans *= p;
    }

    // Output the result
    println!("{}", ans);
}

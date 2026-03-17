use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: u64 = iter.next().expect("Missing n").parse().expect("Invalid n");
    let mut p: u64 = iter.next().expect("Missing p").parse().expect("Invalid p");

    // Preconditions: n must be greater than 0
    if n == 0 {
        panic!("n must be greater than 0");
    }

    let mut ans: u64 = 1;

    // Factorization loop
    for i in 2..=((p as f64).sqrt() as u64) {
        let mut cnt = 0;
        while p % i == 0 {
            cnt += 1;
            p /= i;
        }
        if cnt >= n {
            ans *= i.pow((cnt / n) as u32);
        }
    }

    // If p is still greater than 1 and n is 1, multiply ans by p
    if p > 1 && n == 1 {
        ans *= p;
    }

    // Output the result
    println!("{}", ans);
}

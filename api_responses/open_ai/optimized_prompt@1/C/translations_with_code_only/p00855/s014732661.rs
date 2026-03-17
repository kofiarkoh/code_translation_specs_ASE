fn p(mut n: i32) -> i32 {
    let mut t = 2; // Start t at 2
    let mut i = 2; // Start i at 2
    while i * i <= n {
        if n % i == 0 {
            // If n is divisible by i, do nothing (n%i is 0)
        } else {
            t += 1; // Increment t if n is not divisible by i
        }
        i += 1; // Increment i
    }
    t - 2 // Return t - 2
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut l: i32;
    let mut u: i32;

    for line in handle.lines() {
        let line = line.unwrap();
        l = line.trim().parse().unwrap(); // Read l from input
        u = l; // Initialize u to l

        while p(l) > 0 {
            l -= 1; // Decrement l while p(l) is true
        }
        while p(u) > 0 {
            u += 1; // Increment u while p(u) is true
        }
        println!("{}", u - l); // Print the difference
    }
}

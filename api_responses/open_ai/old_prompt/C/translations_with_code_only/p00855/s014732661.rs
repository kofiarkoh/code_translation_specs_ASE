fn p(mut n: i32) -> i32 {
    let mut t = 2; // Start t at 2
    let mut i = 2; // Start i at 2
    while i * i <= n {
        if n % i == 0 {
            // If n is divisible by i, increment t
            t += 1;
        }
        i += 1; // Increment i
    }
    t - 2 // Return t - 2
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let handle = stdin.lock();
    
    for line in handle.lines() {
        let l: i32 = match line {
            Ok(line) => line.trim().parse().unwrap(),
            Err(_) => break, // Break on read error
        };
        
        let mut u = l; // Initialize u to l

        // Decrement l until p(l) returns 0
        while p(l) > 0 {
            l -= 1;
        }

        // Increment u until p(u) returns 0
        while p(u) > 0 {
            u += 1;
        }

        // Print the difference
        println!("{}", u - l);
    }
}

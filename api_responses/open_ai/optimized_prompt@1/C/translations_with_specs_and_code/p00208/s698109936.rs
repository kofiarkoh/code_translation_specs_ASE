use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let input = line.unwrap();
        let a: i32 = input.trim().parse().unwrap(); // Assumes valid input as per spec
        if a == 0 {
            break; // Sentinel value
        }

        let mut n = [0; 200]; // Array to hold octal digits
        let mut b = 0;

        // Convert to octal
        let mut temp = a;
        while temp > 0 {
            n[b] = temp % 8;
            temp /= 8;
            b += 1;
        }

        // Mark the end of valid octal digits
        n[b] = 0;

        // Transform octal digits
        for i in 0..b {
            if n[i] > 3 {
                n[i] += 1;
            }
            if n[i] > 5 {
                n[i] += 1;
            }
        }

        // Output in reverse order
        for i in (0..b).rev() {
            print!("{}", n[i]);
        }
        println!(); // Newline after each output
    }
}

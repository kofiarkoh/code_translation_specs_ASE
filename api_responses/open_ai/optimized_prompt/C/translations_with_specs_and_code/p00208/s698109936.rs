use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let input = line.expect("Failed to read line");
        let a: i32 = input.trim().parse().expect("Expected a valid integer");

        if a == 0 {
            break; // Sentinel value encountered, exit the loop
        }

        let mut n = [0; 200]; // Array to store octal digits
        let mut b = 0;
        let mut temp = a;

        // Convert decimal to octal
        while temp > 0 {
            n[b] = temp % 8;
            temp /= 8;
            b += 1;
        }

        // Modify octal digits according to the specification
        for i in 0..b {
            if n[i] > 3 {
                n[i] += 1;
            }
            if n[i] > 5 {
                n[i] += 1;
            }
        }

        // Output the modified octal representation
        for i in (0..b).rev() {
            print!("{}", n[i]);
        }
        println!(); // Newline after each output
    }
}

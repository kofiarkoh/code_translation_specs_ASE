use std::io::{self, Write};

fn main() {
    let mut n = [0; 200]; // Array to hold the octal digits
    loop {
        let mut input = String::new();
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input
        io::stdin().read_line(&mut input).unwrap();
        let a: i32 = input.trim().parse().unwrap();
        
        if a == 0 {
            break;
        }
        
        let mut b = 0;
        let mut a = a; // Shadowing to modify `a`
        
        // Convert to octal
        while a != 0 {
            n[b] = a % 8;
            a /= 8;
            b += 1;
        }
        
        n[b] = 0; // Null-terminate the array (not strictly necessary in Rust)
        let mut c = 0;
        let mut i = 0;
        
        // Adjust values based on the conditions
        while i <= b {
            if n[i] > 3 {
                n[i] += 1;
            }
            if n[i] > 5 {
                n[i] += 1;
            }
            i += 1;
        }
        
        // Print the result in reverse order
        for i in (0..b).rev() {
            print!("{}", n[i]);
        }
        println!(); // Print newline
    }
}

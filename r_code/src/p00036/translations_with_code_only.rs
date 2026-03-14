use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut pat = [0; 4];
    
    loop {
        let mut h = 0;
        let mut minv = 255;

        for i in 0..8 {
            let mut buf = String::new();
            if stdin.lock().read_line(&mut buf).is_err() {
                return; // Handle EOF
            }
            let buf = buf.trim(); // Remove trailing newline

            if buf.is_empty() {
                break; // Blank line
            }

            let mut v = 0;
            for j in 0..8 {
                v <<= 1;
                if buf.chars().nth(j) == Some('1') {
                    v += 1;
                }
            }

            if v > 0 {
                pat[h] = v;
                h += 1;
                if v < minv {
                    minv = v;
                }
            }
        }

        if h == 0 {
            return; // No valid input, exit
        }

        // Normalize the pattern values
        for i in 0..h {
            pat[i] /= minv;
        }

        // Output based on the number of patterns and their values
        match h {
            1 => println!("C"),
            4 => println!("B"),
            2 => {
                if pat[0] == 2 {
                    println!("E");
                } else if pat[1] == 2 {
                    println!("G");
                } else {
                    println!("A");
                }
            }
            _ => {
                if pat[0] == 1 {
                    println!("D");
                } else {
                    println!("F");
                }
            }
        }
    }
}

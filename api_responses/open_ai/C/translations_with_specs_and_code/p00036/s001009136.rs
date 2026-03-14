use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut pat = Vec::new();

    loop {
        let mut minv = 255;
        let mut h = 0;

        for line in stdin.lock().lines() {
            let line = line.expect("Failed to read line");
            if line.is_empty() {
                break; // End of input for this batch
            }

            // Ensure the line is exactly 8 characters long and consists of '0' and '1'
            if line.len() != 8 || !line.chars().all(|c| c == '0' || c == '1') {
                continue; // Skip invalid lines
            }

            // Convert binary string to integer
            let v = isize::from_str_radix(&line, 2).expect("Failed to convert binary string to integer");

            if v > 0 {
                pat.push(v);
                if v < minv {
                    minv = v;
                }
                h += 1;
            }
        }

        if h == 0 {
            return; // No valid input, terminate the program
        }

        // Normalize the values
        for i in 0..h {
            pat[i] /= minv;
        }

        // Determine output label based on the processed values
        let output = match h {
            1 => "C",
            4 => "B",
            2 => {
                if pat[0] == 2 {
                    "E"
                } else if pat[1] == 2 {
                    "G"
                } else {
                    "A"
                }
            }
            _ => {
                if pat[0] == 1 {
                    "D"
                } else {
                    "F"
                }
            }
        };

        println!("{}", output);
        pat.clear(); // Clear the pattern for the next batch
    }
}

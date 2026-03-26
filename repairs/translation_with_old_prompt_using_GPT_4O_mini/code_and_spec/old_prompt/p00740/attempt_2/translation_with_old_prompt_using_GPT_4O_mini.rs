use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        
        // Safely parse n and p, handling potential parsing errors
        let n: i32 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(value) => value,
            None => continue, // Skip if parsing fails
        };
        
        let p: i32 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(value) => value,
            None => continue, // Skip if parsing fails
        };

        if n == 0 {
            break;
        }

        let mut i = 0;
        let mut w = p;
        let mut c = vec![0; n as usize]; // Initialize the counter array with size n

        loop {
            if w > 0 {
                w -= 1;
                c[i as usize % n as usize] += 1;

                if w == 0 {
                    let mut found = false;
                    for j in 0..n {
                        if j != i % n {
                            if c[j as usize] > 0 {
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        println!("{}", i % n);
                        break;
                    }
                }
            } else {
                w += c[i as usize % n as usize];
                c[i as usize % n as usize] = 0;
            }
            i += 1;
        }
    }
}

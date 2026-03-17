use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());

        let n = match nums.next() {
            Some(num) => num,
            None => continue,
        };

        if n == 0 {
            break; // Terminate on zero input
        }

        let p = match nums.next() {
            Some(num) => num,
            None => continue,
        };

        let mut c = vec![0; 50]; // Array to track counts, size based on C code
        let mut i = 0;
        let mut w = p;

        loop {
            if w > 0 {
                w -= 1;
                c[i % n as usize] += 1;

                if w == 0 {
                    // Check if all other participants have zero count
                    if c.iter().enumerate().all(|(j, &count)| j == (i % n as usize) || count == 0) {
                        println!("{}", i % n);
                        break;
                    }
                }
            } else {
                w += c[i % n as usize];
                c[i % n as usize] = 0;
            }
            i += 1;
        }
    }
}

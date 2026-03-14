use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Loop to read input until EOF
    for line in reader.lines() {
        let line = line.unwrap();
        let mut inputs = line.split_whitespace();

        // Read three integers from input
        let c1: i32 = match inputs.next() {
            Some(num) => num.parse().unwrap(),
            None => continue, // Skip if not enough inputs
        };
        let c2: i32 = match inputs.next() {
            Some(num) => num.parse().unwrap(),
            None => continue, // Skip if not enough inputs
        };
        let c3: i32 = match inputs.next() {
            Some(num) => num.parse().unwrap(),
            None => continue, // Skip if not enough inputs
        };

        let mut cnt = 0;

        // Check valid integers from 1 to 10
        for i in 1..=10 {
            if c1 != i && c2 != i && c3 != i {
                if c1 + c2 + i <= 20 {
                    cnt += 1;
                }
            }
        }

        // Output based on the count of valid integers
        if cnt >= 4 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

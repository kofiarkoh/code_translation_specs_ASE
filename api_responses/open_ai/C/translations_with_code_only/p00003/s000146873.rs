use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of test cases
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        // Read a, b, c
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        let c = nums.next().unwrap();

        // Check the conditions in the same order as the C code
        if a <= b && b <= c {
            if c * c == a * a + b * b {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if a <= c && c <= b {
            if b * b == a * a + c * c {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if b <= c && c <= a {
            if a * a == b * b + c * c {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if b <= a && a <= c {
            if c * c == a * a + b * b {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if c <= a && a <= b {
            if b * b == a * a + c * c {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if c <= b && b <= a {
            if a * a == b * b + c * c {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}

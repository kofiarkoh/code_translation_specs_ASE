use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let s: i32 = line.unwrap().trim().parse().unwrap();
        let mut ans = 0;

        for a in 0..=1000 {
            if s < a {
                break; // Exit if s < a
            }
            for b in 0..=1000 {
                if s < a + b {
                    break; // Exit if s < a + b
                }
                ans += 501 - (500 - (s - a - b)).abs();
            }
        }
        println!("{}", ans);
    }
}

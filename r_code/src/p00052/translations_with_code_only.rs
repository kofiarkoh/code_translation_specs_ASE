use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.unwrap();
        let n: i64 = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut a = 0;
        for i in 5..=n {
            let mut b = i;
            while b > 0 {
                if b % 5 != 0 {
                    break;
                }
                a += 1;
                b /= 5; // This division is safe as b is guaranteed to be > 0
            }
        }
        println!("{}", a);
    }
}

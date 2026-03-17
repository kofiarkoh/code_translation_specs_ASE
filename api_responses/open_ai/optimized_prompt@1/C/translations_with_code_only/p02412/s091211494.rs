use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        let x = nums.next().unwrap();

        if n == 0 && x == 0 {
            break;
        }

        let mut b = 0;

        for k in 1..=n {
            for j in 1..=n {
                for i in 1..=n {
                    if i != j && j != k && i != k {
                        let a = i + j + k;
                        if a == x {
                            b += 1;
                        }
                    }
                }
            }
        }

        // Integer division in Rust is the same as in C for positive integers
        println!("{}", b / 6);
    }
}

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut lines = handle.lines();
    let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut odd = 1;
    for _ in 0..n {
        let a: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        if a % 2 == 0 {
            odd *= 2;
        }
    }

    let mut exp = 1;
    for _ in 1..=n {
        exp *= 3;
    }

    // Print the result as per the original C code
    println!("{}", exp - odd);
}

use std::io::{self, BufRead};

fn max(x: usize, y: usize) -> usize {
    if x > y { x } else { y }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let w: i32 = iter.next().unwrap().parse().unwrap();
        let d: i32 = iter.next().unwrap().parse().unwrap();

        if w == 0 {
            break; // Exit the loop if w is 0, similar to the C code behavior
        }

        let mut a = [0; 21]; // Array to hold counts for a
        let mut b = [0; 21]; // Array to hold counts for b

        for _ in 0..w {
            let t: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
            a[t] += 1;
        }

        for _ in 0..d {
            let t: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
            b[t] += 1;
        }

        let mut res = 0;
        for i in 1..=20 {
            res += i * max(a[i], b[i]);
        }

        println!("{}", res);
    }
}

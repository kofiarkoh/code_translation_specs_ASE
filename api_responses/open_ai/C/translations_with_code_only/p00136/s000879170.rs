use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a number");

    let mut cnt = [0; 7]; // cnt[0] is unused, cnt[1] to cnt[6] are used

    for _ in 0..n {
        let mut sin = String::new();
        io::stdin().read_line(&mut sin).expect("Failed to read line");
        let sin: f64 = sin.trim().parse().expect("Please enter a number");

        if sin < 165.0 {
            cnt[1] += 1;
        } else if sin < 170.0 {
            cnt[2] += 1;
        } else if sin < 175.0 {
            cnt[3] += 1;
        } else if sin < 180.0 {
            cnt[4] += 1;
        } else if sin < 185.0 {
            cnt[5] += 1;
        } else {
            cnt[6] += 1;
        }
    }

    for i in 1..=6 {
        print!("{}:", i);
        for _ in 0..cnt[i] {
            print!("*");
        }
        println!();
    }
}

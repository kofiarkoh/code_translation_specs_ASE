// Aizu Vol-0 0097: Sum of Integers II
// 2017.9.1 bal4u@uu

use std::io::{self, BufRead};

const P: [[[i32; 902]; 101]; 10] = Default::default();

fn main() {
    let mut p = [[[0; 902]; 101]; 10];

    // Initialize the base case
    for i in 0..=100 {
        p[1][i][i] = 1;
    }

    let (mut min, mut max, mut n) = (0, 100, 1);
    while n < 9 {
        for i in 0..=100 {
            for j in min..=max {
                for k in 1..=100 - i {
                    p[n + 1][i + k][j + i + k] += p[n][i][j];
                }
            }
        }
        min += 1;
        max += 100;
        n += 1;
    }

    let stdin = io::stdin();
    let reader = stdin.lock();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        let s = nums.next().unwrap();

        if n == 0 {
            break;
        }

        if s > 864 {
            println!("0");
        } else {
            let mut ans = 0;
            for i in 0..=100 {
                ans += p[n as usize][i][s as usize];
            }
            println!("{}", ans);
        }
    }
}

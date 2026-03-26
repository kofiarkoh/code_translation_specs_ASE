use std::io::{self, BufRead};

fn counter(z: i32) -> i32 {
    if z == 0 {
        0
    } else {
        counter(z >> 1) + (z & 1)
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut a = vec![0; 20];
    let mut x = vec![vec![0; 20]; 20];
    let mut y = vec![vec![0; 20]; 20];

    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
        for j in 0..a[i] {
            let line = lines.next().unwrap().unwrap();
            let nums: Vec<i32> = line.split_whitespace()
                .map(|s| s.trim().parse().unwrap())
                .collect();
            x[i][j] = nums[0];
            y[i][j] = nums[1];
        }
    }

    let mut max = 0;

    for i in 0..(1 << n) {
        let mut flag = 1;
        for j in 0..n {
            if (i & (1 << j)) == 0 {
                continue;
            }
            for k in 0..a[j] {
                if ((i >> (x[j][k] - 1)) & 1) ^ y[j][k] != 0 {
                    flag = 0;
                    break;
                }
            }
        }
        // If there were no contradictions
        if flag == 1 {
            let count = counter(i);
            if max < count {
                max = count;
            }
        }
    }

    println!("{}", max);
}

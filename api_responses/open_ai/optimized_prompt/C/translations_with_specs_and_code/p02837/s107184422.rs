use std::io::{self, BufRead};

fn count_set_bits(z: u32) -> u32 {
    if z == 0 {
        0
    } else {
        count_set_bits(z >> 1) + (z & 1)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of groups
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; 20];
    let mut x = vec![vec![0; 20]; 20];
    let mut y = vec![vec![0; 20]; 20];

    // Read constraints for each group
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap();
        for j in 0..a[i] {
            let line = lines.next().unwrap().unwrap();
            let nums: Vec<u32> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            x[i][j] = nums[0];
            y[i][j] = nums[1];
        }
    }

    let mut max = 0;

    // Iterate through all combinations of groups
    for i in 0..(1 << n) {
        let mut flag = true;

        for j in 0..n {
            if (i & (1 << j)) == 0 {
                continue;
            }
            for k in 0..a[j] {
                if ((i >> (x[j][k] - 1)) & 1) ^ y[j][k] != 0 {
                    flag = false;
                    break;
                }
            }
            if !flag {
                break;
            }
        }

        // Update max count of set bits if valid combination
        if flag {
            let count = count_set_bits(i);
            if max < count {
                max = count;
            }
        }
    }

    // Output the maximum number of set bits
    println!("{}", max);
}

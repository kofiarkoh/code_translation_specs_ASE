use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read H, W, K
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let h: usize = parts.next().unwrap().parse().unwrap();
    let w: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read the grid
    let mut grid = vec![vec!['0'; w + 1]; h + 1]; // 1-based indexing
    for i in 1..=h {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            grid[i][j + 1] = c; // Fill the grid
        }
    }

    let mut flag = vec![0; h + 2];
    flag[h + 1] = 1; // Mark the end flag
    let mut ans = usize::MAX;

    while flag[h + 1] == 0 {
        let mut t = vec![1];
        let mut count = vec![0; h + 1];
        let mut b = vec![0; h + 1];

        for i in 0.. {
            if t[i] > h {
                break;
            }
            count[i] = 0;
            b[i] = t[i];
            while flag[b[i]] == 0 {
                b[i] += 1;
            }
            t.push(b[i] + 1);
        }

        let mut tmp = 0;
        for j in 1..=w {
            let mut valid = true;
            for i in 0.. {
                if t[i] > h {
                    break;
                }
                count[i] = 0;
                for k in t[i]..=b[i] {
                    if grid[k][j] == '1' {
                        count[i] += 1;
                    }
                }
                if count[i] > k {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
            tmp += 1;
        }

        if tmp < ans {
            ans = tmp;
        }

        let mut i = 1;
        while i <= h && flag[i] == 1 {
            i += 1;
        }
        flag[i] = 1;
        for j in (1..=i - 1).rev() {
            flag[j] = 0;
        }
    }

    // Ensure we print the correct answer
    if ans == usize::MAX {
        println!("0");
    } else {
        println!("{}", ans);
    }
}

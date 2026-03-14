use std::io::{self, BufRead};

const INT_MIN: i32 = -1_000_000_000;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let n: i32 = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let n = n as usize;
        let mut a = vec![0; 5000];
        let mut left = vec![0; 5010];
        let mut right = vec![0; 5010];

        for i in 0..n {
            a[i] = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();
        }

        left[0] = 0;
        for i in 1..n {
            left[i] = if left[i - 1] + a[i - 1] > 0 {
                left[i - 1] + a[i - 1]
            } else {
                0
            };
        }

        right[n - 1] = 0;
        for i in (0..n - 1).rev() {
            right[i] = if right[i + 1] + a[i + 1] > 0 {
                right[i + 1] + a[i + 1]
            } else {
                0
            };
        }

        let mut ans = INT_MIN;
        for i in 0..n {
            ans = ans.max(left[i] + a[i] + right[i]);
        }

        println!("{}", ans);
    }
}

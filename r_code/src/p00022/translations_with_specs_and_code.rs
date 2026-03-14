use std::io::{self, BufRead};

const INT_MIN: i32 = -1_000_000_000;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let n = nums[0];
        if n == 0 {
            break;
        }

        let mut a = vec![0; n as usize];
        let mut left = vec![0; (n + 1) as usize];
        let mut right = vec![0; (n + 1) as usize];

        for i in 0..n {
            a[i as usize] = nums[(i + 1) as usize];
        }

        left[0] = 0;
        for i in 1..n {
            left[i as usize] = (left[(i - 1) as usize] + a[(i - 1) as usize]).max(0);
        }

        right[(n - 1) as usize] = 0;
        for i in (0..n - 1).rev() {
            right[i as usize] = (right[(i + 1) as usize] + a[(i + 1) as usize]).max(0);
        }

        let mut ans = INT_MIN;
        for i in 0..n {
            ans = ans.max(left[i as usize] + a[i as usize] + right[i as usize]);
        }

        println!("{}", ans);
    }
}

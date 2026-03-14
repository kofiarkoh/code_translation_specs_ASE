use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        let m = nums.next().unwrap();

        if m + n == 0 {
            break;
        }

        let mut d = vec![1; n as usize];
        let mut i = 0;
        let mut sum = 0;
        let mut f = 0;

        loop {
            sum += d[i];
            if sum == m {
                d[i] = 0;
                sum = 0;
                f += 1;
            }
            if f == n {
                break;
            }
            if i == n - 1 {
                i = 0;
                continue;
            }
            i += 1;
        }
        println!("{}", i + 1);
    }
}

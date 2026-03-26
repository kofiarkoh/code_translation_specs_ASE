use std::io::{self, BufRead};

const MAX_SIZE: usize = 100010;

fn dfs(a: &mut [i32], n: usize) {
    let mut t = a[n];
    while a[t as usize] != -1 {
        let temp = a[t as usize];
        a[t as usize] = -1;
        t = temp;
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>());
    let n = nums.next().unwrap().expect("Failed to parse n") as usize;
    let k = nums.next().unwrap().expect("Failed to parse k");

    let mut a = vec![0; MAX_SIZE];
    for i in 1..=n {
        let value: i32 = lines.next().unwrap().unwrap().parse().expect("Failed to parse array value");
        a[i] = value;
    }

    if n == k as usize {
        let x = a[1] - 1;
        let mut flag = true;
        for i in 1..=n {
            flag &= (a[i] - i as i32 + n as i32) % n as i32 == x;
        }
        println!("{}", if flag { "Yes" } else { "No" });
    } else {
        let mut cnt = 0;
        for i in 1..=n {
            if a[i] != -1 {
                dfs(&mut a, i);
                cnt += 1;
            }
        }
        println!("{}", if (n - cnt) % 2 != 0 && k % 2 != 0 { "No" } else { "Yes" });
    }
}

use std::io::{self, BufRead};

fn dfs(a: &mut Vec<i32>, n: usize) {
    let mut t = a[n];
    while a[t as usize] != -1 {
        let temp = a[t as usize];
        a[t as usize] = -1;
        t = temp;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let k = nums.next().unwrap();

    // Preconditions check
    if n <= 0 {
        // Behavior is not defined for non-positive n, but we will not handle this case as per the spec.
        return;
    }

    // Read the array
    let mut a = vec![0; (n + 1) as usize]; // Using 1-based indexing
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        a[i as usize] = line.trim().parse::<i32>().unwrap();
    }

    if n == k {
        let x = a[1] - 1;
        let mut flag = true;
        for i in 1..=n {
            flag &= (a[i as usize] - i + n) % n == x;
        }
        println!("{}", if flag { "Yes" } else { "No" });
    } else {
        let mut cnt = 0;
        for i in 1..=n {
            if a[i as usize] != -1 {
                dfs(&mut a, i);
                cnt += 1;
            }
        }
        println!("{}", if (n - cnt) % 2 != 0 && k % 2 != 0 { "No" } else { "Yes" });
    }
}

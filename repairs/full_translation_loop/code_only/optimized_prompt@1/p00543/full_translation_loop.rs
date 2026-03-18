use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>());
    let n = nums.next().unwrap().unwrap();
    let m = nums.next().unwrap().unwrap();

    let mut a = vec![0; 1001]; // Using 1001 to match 1-based indexing
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        a[i as usize] = line.trim().parse::<i32>().unwrap();
    }

    let mut k = 1;
    while k <= m {
        for i in 1..n {
            let j = i + 1;
            if j <= n {
                if a[i as usize] % k > a[j as usize] % k {
                    // Swap a[i] and a[j]
                    a.swap(i as usize, j as usize);
                }
            }
        }
        k += 1;
    }

    for i in 1..=n {
        println!("{}", a[i as usize]);
    }
}

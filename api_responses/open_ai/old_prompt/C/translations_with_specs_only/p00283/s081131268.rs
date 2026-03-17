use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let d: i32 = parts.next().unwrap().parse().unwrap();
        let n: usize = parts.next().unwrap().parse().unwrap();

        // Termination condition
        if d == 0 && n == 0 {
            break;
        }

        let mut a = vec![0; n];
        let mut e = vec![0; n];
        let mut r = vec![0; n];

        for i in 0..n {
            let item_line = reader.lines().next().unwrap().unwrap();
            let mut item_parts = item_line.split_whitespace();
            a[i] = item_parts.next().unwrap().parse().unwrap();
            e[i] = item_parts.next().unwrap().parse().unwrap();
            r[i] = item_parts.next().unwrap().parse().unwrap();
        }

        let result = min_items(d, n, &a, &e, &r);
        match result {
            Some(count) => println!("{}", count),
            None => println!("NA"),
        }
    }
}

fn min_items(d: i32, n: usize, a: &[i32], e: &[i32], r: &[i32]) -> Option<i32> {
    // Initialize memoization array
    let mut memo = vec![-1; (d + 1) as usize];

    // Base case: 0 items needed to reach 0 value
    memo[0] = 0;

    for i in 0..n {
        for j in (0..=d).rev() {
            if memo[j as usize] != -1 {
                let mut total_value = j + a[i];
                let mut count = memo[j as usize] + 1;

                while total_value <= d {
                    if memo[total_value as usize] == -1 || count < memo[total_value as usize] {
                        memo[total_value as usize] = count;
                    }
                    total_value += e[i];
                    count += 1;
                }
            }
        }
    }

    // Find the minimum number of items needed to reach or exceed D
    if memo[d as usize] == -1 {
        None
    } else {
        Some(memo[d as usize])
    }
}

// AOJ 1634 Balance Scale
// bal4u 2019.8.12

use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let n: usize = line.trim().parse().expect("Invalid input");
        let m: usize = lines.next().unwrap().trim().parse().expect("Invalid input");

        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            let value: usize = lines.next().unwrap().trim().parse().expect("Invalid input");
            a.push(value);
        }

        let mut w = Vec::with_capacity(m);
        for _ in 0..m {
            let weight: usize = lines.next().unwrap().trim().parse().expect("Invalid input");
            w.push(weight);
        }

        let mut q = vec![0; 60000]; // 3^10
        let mut end = 1;

        for &weight in &w {
            let e = end;
            for j in 0..e {
                q[end] = q[j] + weight;
                end += 1;
                q[end] = q[j].wrapping_sub(weight);
                end += 1;
            }
        }

        let mut hash_set = HashSet::new();
        for &value in &q[..end] {
            if value > 0 {
                hash_set.insert(value);
            }
        }

        let mut x = 0;
        let mut ans = usize::MAX;

        for &value in &a {
            if !hash_set.contains(&value) {
                if x == 0 {
                    let mut temp_ans = usize::MAX;
                    let mut hash_set2 = HashSet::new();
                    for &computed in &q[..end] {
                        let t = (value as isize - computed as isize).abs() as usize;
                        if t < temp_ans {
                            temp_ans = t;
                        }
                        hash_set2.insert(t);
                    }
                    ans = temp_ans;
                    x = 1;
                } else {
                    let mut found = false;
                    let mut temp_ans = usize::MAX;
                    x += 1;
                    for &computed in &q[..end] {
                        let t = (value as isize - computed as isize).abs() as usize;
                        if hash_set.contains(&(t as usize)) {
                            if t < temp_ans {
                                temp_ans = t;
                            }
                            found = true;
                        }
                    }
                    if !found {
                        ans = usize::MAX; // No valid difference found
                        break;
                    } else {
                        ans = temp_ans;
                    }
                }
            }
        }

        if ans == usize::MAX {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}

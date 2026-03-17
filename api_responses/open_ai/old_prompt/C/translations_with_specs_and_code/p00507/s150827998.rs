use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Preconditions check
    if n < 2 {
        panic!("N must be greater than 1");
    }

    // Read the integers into a vector
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for line in lines.take(n) {
        let num: i32 = line.unwrap().parse().unwrap();
        a.push(num);
    }

    // To store the three smallest unique combinations
    let mut r: Vec<i64> = vec![i64::MAX; 3];
    let mut unique_combinations = HashSet::new();

    // Generate combinations of two distinct integers
    for i in 0..n {
        for j in 0..n {
            if i != j {
                // Find the multiplier for A[j]
                let mut k = 10;
                while a[j] % k != a[j] {
                    k *= 10;
                }
                let combination = (a[i] as i64) * k + (a[j] as i64);

                // Insert the combination into the set to ensure uniqueness
                if unique_combinations.insert(combination) {
                    // Update the three smallest unique combinations
                    for k in 0..3 {
                        if r[k] > combination {
                            r[k] = combination;
                            break;
                        }
                    }
                }
            }
        }
    }

    // Output the third smallest unique combination
    println!("{}", r[2]);
}

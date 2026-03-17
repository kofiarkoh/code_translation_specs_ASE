use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut inputs = first_line.split_whitespace();
    let n: i32 = inputs.next().unwrap().parse().unwrap();
    let m: i32 = inputs.next().unwrap().parse().unwrap();

    // Preconditions: n and m must be positive integers
    assert!(n > 0 && m > 0);

    let mut a = vec![0; (m + 1) as usize];
    let mut b = vec![0; (m + 1) as usize];
    let mut count1 = 0;
    let mut count2 = 0;

    // Read pairs of integers
    for i in 1..=m {
        let line = lines.next().unwrap().unwrap();
        let mut pair = line.split_whitespace();
        let a_i: i32 = pair.next().unwrap().parse().unwrap();
        let b_i: i32 = pair.next().unwrap().parse().unwrap();

        // Process the input pairs
        if a_i >= n {
            a[i as usize] = 0;
            b[i as usize] = 0;
            count1 += 1;
        } else {
            a[i as usize] = a_i;
            b[i as usize] = b_i;
        }
    }

    // If there are more pairs than those set to zero
    if m - 1 > count1 {
        for i in 1..=m {
            b[i as usize] = n - a[i as usize];
        }

        // Sort b using a simple bubble sort (O(m^2) complexity)
        for i in 1..m {
            for j in (i + 1)..=m {
                if b[i as usize] > b[j as usize] {
                    b.swap(i as usize, j as usize);
                }
            }
        }

        // Sum the first m - count1 - 1 elements of b
        for i in 1..=m - count1 - 1 {
            count2 += b[i as usize];
        }
    }

    // Output the result
    println!("{}", count2);
}

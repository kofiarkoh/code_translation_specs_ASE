use std::io::{self, BufRead};
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read the first line for n
    let n: usize = input.next().unwrap().unwrap().parse().unwrap();
    let mut data: Vec<i32> = Vec::new();
    
    // Read the data array
    if n > 0 {
        data.extend(input.next().unwrap().unwrap().split_whitespace().map(|x| x.parse().unwrap()));
    }

    // Read the second line for m
    let m: usize = input.next().unwrap().unwrap().parse().unwrap();
    let mut b: Vec<i32> = Vec::new();
    let mut c: Vec<i64> = Vec::new();

    // Read the b array
    if m > 0 {
        b.extend(input.next().unwrap().unwrap().split_whitespace().map(|x| x.parse().unwrap()));
    }

    // Read the c array
    if m > 0 {
        c.extend(input.next().unwrap().unwrap().split_whitespace().map(|x| x.parse().unwrap()));
    }

    // Sort the data array
    data.sort();

    // Create a cumulative sum array
    let mut sum: Vec<i64> = vec![0; n];
    if n > 0 {
        sum[0] = data[0] as i64;
        for i in 1..n {
            sum[i] = sum[i - 1] + data[i] as i64;
        }
    }

    // Process each element in b
    for i in 0..m {
        let key = b[i];
        let upper_bound_index = upper_bound(&data, key);
        
        // Check if the cumulative sum at the upper bound index meets or exceeds c[i]
        if upper_bound_index > 0 && sum[upper_bound_index - 1] >= c[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

// Function to find the upper bound index
fn upper_bound(data: &[i32], key: i32) -> usize {
    let mut low = 0;
    let mut high = data.len();
    
    while low < high {
        let mid = (low + high) / 2;
        if data[mid] <= key {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low // This will be the upper bound index
}

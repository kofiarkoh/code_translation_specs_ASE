use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for n and m
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // If there are no integers or weights, output -1
    if n == 0 || m == 0 {
        println!("-1");
        return;
    }

    // Read the integers a[]
    let mut a = Vec::with_capacity(n);
    let mut second_line = String::new();
    reader.read_line(&mut second_line).unwrap();
    for num in second_line.split_whitespace() {
        a.push(num.parse::<i32>().unwrap());
    }

    // Read the weights w[]
    let mut w = Vec::with_capacity(m);
    let mut third_line = String::new();
    reader.read_line(&mut third_line).unwrap();
    for weight in third_line.split_whitespace() {
        w.push(weight.parse::<i32>().unwrap());
    }

    // Use a HashSet to store all possible sums of weights
    let mut possible_sums = HashSet::new();
    possible_sums.insert(0); // Start with the zero sum

    // Generate all possible sums of weights using bit manipulation
    for i in 0..m {
        let current_weight = w[i];
        let current_sums: Vec<i32> = possible_sums.iter().copied().collect();
        for sum in current_sums {
            possible_sums.insert(sum + current_weight);
            possible_sums.insert(sum - current_weight);
        }
    }

    // Find the minimum absolute difference
    let mut min_diff = i32::MAX;
    for &target in &a {
        for &sum in &possible_sums {
            let diff = (sum - target).abs();
            if diff < min_diff {
                min_diff = diff;
            }
        }
    }

    // If min_diff is still MAX, it means no suitable weight was found
    if min_diff == i32::MAX {
        println!("-1");
    } else {
        println!("{}", min_diff);
    }
}

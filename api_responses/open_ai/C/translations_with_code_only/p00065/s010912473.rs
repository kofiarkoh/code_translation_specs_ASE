use std::io::{self, BufRead};
use std::f64;

fn sosu(n: usize) -> Vec<i32> {
    let mut num = vec![0; n + 1];
    
    for i in 2..=n {
        num[i] = i as i32;
    }
    
    for i in 2..=((n as f64).sqrt() as usize) {
        for j in 2..=(n / i) {
            num[j * i] = 0;
        }
    }
    
    num
}

fn main() {
    let stdin = io::stdin();
    let mut number = Vec::new();
    let mut ac = Vec::new();
    
    // Read first set of inputs
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut parts = line.split(',');
        if let (Some(num_str), Some(ac_str)) = (parts.next(), parts.next()) {
            if let (Ok(num), Ok(ac_val)) = (num_str.parse::<i32>(), ac_str.parse::<i32>()) {
                number.push(num);
                ac.push(ac_val);
            }
        } else {
            break;
        }
    }
    
    // Read second set of inputs
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut parts = line.split(',');
        if let (Some(num_str), Some(ac_str)) = (parts.next(), parts.next()) {
            if let (Ok(num), Ok(ac_val)) = (num_str.parse::<i32>(), ac_str.parse::<i32>()) {
                number.push(num);
                ac.push(ac_val);
            }
        } else {
            break;
        }
    }
    
    let count = number.len();
    
    // Sort the numbers in descending order
    for i in 0..count {
        for j in 0..count {
            if number[i] < number[j] {
                number.swap(i, j);
            }
        }
    }
    
    let mut i = 0;
    while i < count {
        let mut k = 0;
        for j in (i + 1)..count {
            if number[i] == number[j] {
                k += 1;
            }
        }
        if k >= 1 {
            println!("{} {}", number[i], k + 1);
        }
        i += k + 1; // Move to the next unique number
    }
}

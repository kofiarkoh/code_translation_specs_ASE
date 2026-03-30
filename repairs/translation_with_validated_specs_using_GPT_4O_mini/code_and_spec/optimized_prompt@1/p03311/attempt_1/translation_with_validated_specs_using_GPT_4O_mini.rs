use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the number of elements
    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");
    let n: i64 = input.trim().parse().expect("Input must be a positive integer");
    
    // Preconditions check
    if n <= 0 {
        panic!("The number of elements must be a positive integer.");
    }

    // Read the sequence of long integers
    let mut a: Vec<i64> = Vec::with_capacity(n as usize);
    for i in 0..n {
        input.clear();
        handle.read_line(&mut input).expect("Failed to read line");
        let value: i64 = input.trim().parse().expect("Input must be a long integer");
        a.push(value - (i + 1)); // Modify by subtracting index + 1
    }

    // Sort the modified sequence
    a.sort();

    // Calculate the total difference
    let all: i64 = a.iter().skip(1).map(|&x| x - a[0]).sum();
    let mut sadness = all;
    let mut min = all;
    let mut level = a[0];

    // Minimum value calculation logic
    for i in 0..n as usize {
        if i + 1 < n as usize && a[i + 1] == level {
            continue;
        }
        let width = i + 1;
        if n - width * 2 <= 0 {
            break;
        }
        let height = a[i + 1] - a[i];
        if height * (n - width * 2) <= sadness {
            level = a[i + 1];
            sadness -= height * (n - width * 2);
            min = sadness;
        } else {
            let height = sadness / (n - width * 2);
            level += height;
            sadness -= height * (n - width * 2);
            if sadness <= min {
                min = sadness;
            }
            sadness = -(sadness - (n - width * 2));
            if sadness <= min {
                min = sadness;
            }
            break;
        }
    }

    // Output the computed minimum value
    println!("{}", min);
}

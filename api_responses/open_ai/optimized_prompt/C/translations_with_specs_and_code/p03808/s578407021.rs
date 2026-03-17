use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements in the sequence
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n + 1]; // Create a vector to hold the sequence, with an extra space for A[N]

    // Read the sequence of integers
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }
    a[n] = a[0]; // Set A[N] to A[0]

    let mut sum = 0;
    let mut diff = vec![0; n + 1]; // Create a vector for differences

    // Calculate the sum and differences
    for i in 1..=n {
        sum += a[i];
        diff[i] = a[i] - a[i - 1];
    }

    // Check the first condition
    if sum % (n * (n + 1) / 2) != 0 {
        println!("NO");
    } else {
        sum /= n * (n + 1) / 2; // Calculate the average

        diff[0] = 0; // Initialize diff[0]
        let mut valid = true; // Flag to check validity
        for i in 1..=n {
            diff[i] = (sum - diff[i]) / n; // Calculate the adjusted difference
            if diff[i] < 0 || diff[i] > sum {
                valid = false; // Mark as invalid if out of bounds
                break;
            } else {
                diff[0] += diff[i]; // Accumulate the total difference
            }
        }

        // Check the final conditions
        if !valid || diff[0] != sum {
            println!("NO");
        } else {
            for i in 1..=n {
                a[n] -= diff[i] * (n - i + 1); // Adjust A[N]
            }
            if a[n] != 0 {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

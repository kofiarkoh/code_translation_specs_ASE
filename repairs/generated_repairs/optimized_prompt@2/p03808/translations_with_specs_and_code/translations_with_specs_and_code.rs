use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements in the sequence
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse the number of elements.");
        std::process::exit(1);
    });
    let mut a = vec![0; n + 1]; // Create a vector to hold the sequence, with an extra space for A[N]

    // Read the sequence of integers
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap_or_else(|_| {
            eprintln!("Failed to parse element at index {}", i);
            std::process::exit(1);
        });
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
        sum /= n * (n + 1) / 2; // Update sum to the average

        let mut total_diff = 0;
        let mut valid = true;

        for i in 1..=n {
            diff[i] = (sum - diff[i]) / n; // Update the difference
            if diff[i] < 0 || diff[i] > sum {
                valid = false;
                break;
            } else {
                total_diff += diff[i];
            }
        }

        // Check the second condition
        if !valid || total_diff != sum {
            println!("NO");
        } else {
            a[n] = a[0]; // Reset A[N] to A[0] before updating
            for i in 1..=n {
                a[n] -= diff[i] * (n - i + 1); // Update A[N]
            }
            if a[n] != 0 {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

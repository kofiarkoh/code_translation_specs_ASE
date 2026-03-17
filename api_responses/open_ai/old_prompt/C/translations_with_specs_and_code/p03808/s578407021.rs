use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read N
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n + 1);
    
    // Read the array A
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }
    a.push(a[0]); // Mimicking the C code behavior of A[N] = A[0]

    let mut sum = 0;
    let mut diff: Vec<i64> = vec![0; n + 1];

    // Calculate sum and differences
    for i in 1..=n {
        sum += a[i];
        diff[i] = a[i] - a[i - 1];
    }

    // Check if sum is divisible by the triangular number
    let triangular_number = n * (n + 1) / 2;
    if sum % triangular_number != 0 {
        println!("NO");
    } else {
        sum /= triangular_number;
        diff[0] = 0;
        let mut total_diff = 0;
        let mut valid = true;

        for i in 1..=n {
            diff[i] = (sum - diff[i]) / n;
            if diff[i] < 0 || diff[i] > sum {
                valid = false;
                break;
            }
            total_diff += diff[i];
        }

        if !valid || total_diff != sum {
            println!("NO");
        } else {
            let mut last_value = a[n];
            for i in 1..=n {
                last_value -= diff[i] * (n - i + 1);
            }
            if last_value != 0 {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

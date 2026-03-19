use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements in the sequence
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(n >= 1 && n <= 100_000, "N must be between 1 and 100,000");

    let mut a = vec![0; n + 1];
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }
    a[n] = a[0]; // Mimicking the C code behavior

    let mut sum = 0;
    let mut diff = vec![0; n + 1];

    for i in 1..=n {
        sum += a[i];
        diff[i] = a[i] - a[i - 1];
    }

    if sum % (n * (n + 1) / 2) != 0 {
        println!("NO");
    } else {
        sum /= n * (n + 1) / 2;
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
            for i in 1..=n {
                a[n] -= diff[i] * (n - i + 1);
            }
            if a[n] != 0 {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

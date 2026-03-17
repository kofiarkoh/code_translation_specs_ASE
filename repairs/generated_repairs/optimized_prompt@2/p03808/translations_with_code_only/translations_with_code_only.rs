use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut a = vec![0; n + 1];
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..n {
        a[i] = values[i];
    }
    a[n] = a[0];

    let mut sum = 0;
    let mut diff = vec![0; n + 1];

    for i in 1..=n {
        sum += a[i];
        diff[i] = a[i] - a[i - 1];
    }

    let total = n * (n + 1) / 2;
    if sum % total as i64 != 0 { // Cast total to i64 for comparison
        println!("NO");
    } else {
        sum /= total as i64;
        diff[0] = 0;
        let mut i = 1;

        while i <= n {
            diff[i] = (sum - diff[i]) / n as i64;
            if diff[i] < 0 || diff[i] > sum {
                break;
            } else {
                diff[0] += diff[i];
            }
            i += 1;
        }

        if i <= n || diff[0] != sum {
            println!("NO");
        } else {
            for j in 1..=n {
                a[n] -= diff[j] * (n - j + 1) as i64;
            }
            if a[n] != 0 {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
    io::stdout().flush().unwrap();
}

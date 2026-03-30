use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse N");
    let k: usize = iter.next().unwrap().parse().expect("Failed to parse K");

    let mut a = vec![0; n + 1];
    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        a[i] = input.trim().parse().expect("Failed to parse A[i]");
    }

    let mut min = n; // Initialize min to n
    let mut b = vec![0; n + 2];
    
    for _ in 1..=k {
        min = n; // Reset min for each iteration
        for i in 1..=n + 1 {
            b[i] = 0; // Reset B array
        }
        for i in 1..=n {
            if a[i] >= i {
                b[1] += 1;
            } else {
                b[i - a[i]] += 1;
            }
            if a[i] >= n - i {
                b[n + 1] -= 1;
            } else {
                b[i + a[i] + 1] -= 1;
            }
        }
        let mut tmp = 0;
        for i in 1..=n {
            tmp += b[i];
            a[i] = tmp;
            if a[i] < min {
                min = a[i];
            }
        }
    }

    for i in 1..n {
        print!("{} ", a[i]);
    }
    println!("{}", a[n]);
    io::stdout().flush().unwrap();
}

use std::io::{self, BufRead};
use std::cmp::Ordering;

fn cmp(a: &i32, b: &i32) -> Ordering {
    b.cmp(a) // Reverse order for sorting
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut a = vec![0; 1_000_001]; // Allocate space for 1,000,001 integers

    loop {
        let mut n = String::new();
        input.read_line(&mut n).unwrap();
        let n: usize = n.trim().parse().unwrap();
        if n == 0 {
            break; // Exit loop if n is 0
        }

        let mut m = 0;
        for i in 0..n {
            let mut num = String::new();
            input.read_line(&mut num).unwrap();
            a[i] = num.trim().parse().unwrap();
        }

        // Sort the array in descending order
        a[0..n].sort_by(cmp);

        let mut k = 0;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j <= n {
                if j == n || a[i] != a[j] {
                    if m < j - i {
                        m = j - i;
                        k = a[i];
                    }
                    i = j - 1; // Move i to the last unique element
                    break;
                }
                j += 1;
            }
            if j > n {
                break; // Exit if j exceeds n
            }
        }

        if n / 2 < m {
            println!("{}", k);
        } else {
            println!("NO COLOR");
        }
    }
}

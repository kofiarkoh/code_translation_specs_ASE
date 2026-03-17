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
        let n: usize = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid input
        };
        if n == 0 {
            break; // Exit loop if n is 0
        }

        let mut m = 0;
        for i in 0..n {
            let mut num = String::new();
            input.read_line(&mut num).unwrap();
            a[i] = match num.trim().parse() {
                Ok(value) => value,
                Err(_) => continue, // Skip invalid input
            };
        }

        // Sort the array in descending order
        a[0..n].sort_by(cmp);

        let mut k = 0;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n { // Change to j < n to avoid out-of-bounds
                if a[i] != a[j] {
                    if m < j - i {
                        m = j - i;
                        k = a[i];
                    }
                    i = j - 1; // Move i to the last unique element
                    break;
                }
                j += 1;
            }
            if j == n { // Exit if j reaches n
                break;
            }
            i += 1; // Increment i to continue checking
        }

        if n / 2 < m {
            println!("{}", k);
        } else {
            println!("NO COLOR");
        }
    }
}

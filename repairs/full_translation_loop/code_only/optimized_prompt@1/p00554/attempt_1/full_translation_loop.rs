use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    let mut a = vec![0; 1001]; // Using 1001 to match 1-based indexing
    let mut b = vec![0; 1001];
    let mut count1 = 0;
    let mut count2 = 0;

    // Read pairs of a[i] and b[i]
    for i in 1..=m {
        let line = lines.next().unwrap().unwrap();
        let mut pair = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        a[i] = pair.next().unwrap();
        b[i] = pair.next().unwrap();

        if a[i] >= n {
            a[i] = 0;
            b[i] = 0;
            count1 += 1;
        }
    }

    if m - 1 > count1 {
        for i in 1..=m {
            b[i] = n - a[i];
        }

        // Bubble sort to sort b
        for i in 1..m {
            for j in (i + 1)..=m {
                if b[i] > b[j] {
                    b.swap(i, j); // Swap elements
                }
            }
        }

        for i in 1..=m - count1 - 1 {
            count2 += b[i];
        }
    }

    println!("{}", count2);
}

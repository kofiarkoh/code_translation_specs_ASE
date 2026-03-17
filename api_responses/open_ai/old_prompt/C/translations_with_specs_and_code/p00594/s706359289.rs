use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut buffer = String::new();
    
    while input.read_line(&mut buffer).unwrap() > 0 {
        let n: usize = buffer.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        
        let mut a = Vec::with_capacity(n);
        buffer.clear();
        
        // Read n integers
        for _ in 0..n {
            input.read_line(&mut buffer).unwrap();
            let num: i32 = buffer.trim().parse().unwrap();
            a.push(num);
            buffer.clear();
        }
        
        // Sort the array
        a.sort_unstable();
        
        let mut m = 0; // Length of the longest sequence
        let mut k = 0; // The dominant integer
        let mut i = 0;

        while i < n {
            let mut j = i + 1;
            while j < n && a[i] == a[j] {
                j += 1;
            }
            if m < j - i {
                m = j - i;
                k = a[i];
            }
            i = j; // Move to the next different integer
        }

        if m > n / 2 {
            println!("{}", k);
        } else {
            println!("NO COLOR");
        }
    }
}

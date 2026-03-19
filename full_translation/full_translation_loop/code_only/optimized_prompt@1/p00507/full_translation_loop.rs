use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i32> = Vec::with_capacity(n);
    
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Use i64 to accommodate the large values
    let mut r = [9999999999_i64, 9999999999_i64, 9999999999_i64];
    
    for i in 0..n {
        for j in 0..n {
            if i != j {
                let mut k = 10;
                while a[j] % k != a[j] {
                    k *= 10;
                }
                let mut temp_a = a[i] as i64 * k as i64 + a[j] as i64;
                for k in 0..3 {
                    if r[k] > temp_a {
                        // Swap using XOR trick to avoid temporary variable
                        temp_a ^= r[k];
                        r[k] ^= temp_a;
                        temp_a ^= r[k];
                    }
                }
            }
        }
    }
    
    println!("{}", r[2]);
}

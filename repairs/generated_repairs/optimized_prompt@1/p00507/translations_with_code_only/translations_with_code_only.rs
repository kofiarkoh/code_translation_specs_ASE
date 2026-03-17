use std::io::{self, BufRead};

fn cmp(a: &i32, b: &i32) -> bool {
    a > b
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n];

    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    let mut r = [9999999999i64, 9999999999i64, 9999999999i64];

    for i in 0..n {
        for j in 0..n {
            if i != j {
                let mut k = 10;
                while a[j] % k != a[j] {
                    k *= 10;
                }
                let temp_a = (a[i] as i64) * k + (a[j] as i64);
                for k in 0..3 {
                    if r[k] > temp_a {
                        // Swap r[k] and temp_a
                        std::mem::swap(&mut r[k], &mut temp_a);
                    }
                }
            }
        }
    }

    println!("{}", r[2]);
}

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    while input.read_line(&mut buffer).unwrap() > 0 {
        let n: usize = match buffer.trim().parse() {
            Ok(num) if num > 0 => num,
            _ => break,
        };

        let mut a = vec![0; n];
        for i in 0..n {
            buffer.clear();
            input.read_line(&mut buffer).unwrap();
            a[i] = buffer.trim().parse().unwrap();
        }

        let mut swap_count = 0;
        for u in (1..n).rev() {
            for i in 0..u {
                if a[i] > a[i + 1] {
                    a.swap(i, i + 1); // Using Rust's safe swap method
                    swap_count += 1;
                }
            }
        }
        println!("{}", swap_count);
        buffer.clear(); // Clear buffer for the next iteration
    }
}

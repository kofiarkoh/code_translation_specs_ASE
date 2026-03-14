use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut m: i32 = input.trim().parse().unwrap();
        let mut num = [0; 8];

        // Extract digits from m
        for j in (0..8).rev() {
            num[j] = m % 10;
            m /= 10;
        }

        // Sort the digits
        for j in 0..8 {
            for k in j..8 {
                if num[j] > num[k] {
                    num.swap(j, k);
                }
            }
        }

        // Calculate min
        let mut min = 0;
        for j in 0..8 {
            min += num[j];
            if j < 7 {
                min *= 10;
            }
        }

        // Calculate max
        let mut max = 0;
        for j in (0..8).rev() {
            max += num[j];
            if j > 0 {
                max *= 10;
            }
        }

        // Output the difference
        println!("{}", max - min);
    }
}

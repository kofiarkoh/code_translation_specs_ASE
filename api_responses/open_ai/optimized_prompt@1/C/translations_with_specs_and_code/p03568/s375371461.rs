use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for N
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut count_of_even = 0;

    // Read the next N integers
    for _ in 0..n {
        let a: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        if a % 2 == 0 {
            count_of_even += 1;
        }
    }

    // Calculate 3^N
    let exp = 3_i32.pow(n as u32);
    // Calculate 2^(count of even integers)
    let odd = 2_i32.pow(count_of_even as u32);

    // Output the difference
    println!("{}", exp - odd);
}

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let a_start = nums.next().unwrap();
    let a_end = nums.next().unwrap();
    let b_start = nums.next().unwrap();
    let b_end = nums.next().unwrap();

    let mut a = [0; 101];
    let mut b = [0; 101];
    let mut c = [0; 101]; // Changed to 101 to match the C code's indexing

    // Set values in array a
    for i in a_start..=a_end {
        a[i as usize] = 1;
    }

    // Set values in array b
    for i in b_start..=b_end {
        b[i as usize] = 1;
    }

    let mut time = 0;

    // Calculate overlaps
    for i in 0..=100 {
        c[i] = a[i] + b[i];
        if c[i] == 2 {
            time += 1;
        }
    }

    // Output the result
    if time == 0 {
        print!("{}", time);
    } else {
        print!("{}", time - 1);
    }
}

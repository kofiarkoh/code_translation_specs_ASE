use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut a = vec![0; n];
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    a.copy_from_slice(&nums[..n]);

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let q: usize = input.trim().parse().unwrap();

    let mut tmp = vec![0; n];

    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let b = parts[0];
        let m = parts[1];
        let e = parts[2];

        // Copy the relevant segment to the temporary array
        for i in b..e {
            tmp[i] = a[i];
        }

        let e_len = e - b;
        let m_adjusted = m % e_len; // Adjust m to be within the bounds of the segment

        // Perform the circular shift
        for i in 0..e_len {
            a[b + i] = tmp[b + (i + e_len - m_adjusted) % e_len];
        }
    }

    // Output the modified array
    for i in 0..n {
        if i > 0 {
            print!(" ");
        }
        print!("{}", a[i]);
    }
    println!(); // Ensure a newline at the end
}

use std::io::{self, BufRead};

fn get_ints() -> (i32, i32) {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    (nums.next().unwrap(), nums.next().unwrap())
}

fn main() {
    let (n, k) = get_ints();

    // Ensure that k is within the valid range
    if k < 0 || k > n {
        return; // No output if k is out of bounds
    }

    for pat in 0..(1 << n) {
        if pat.count_ones() != k as u32 {
            continue;
        }
        print!("{}:", pat);
        for i in 0..n {
            if pat & (1 << i) != 0 {
                print!(" {}", i);
            }
        }
        println!(); // Print newline after each pattern
    }
}

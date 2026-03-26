use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the integer N
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Allocate space for S1 and S2
    let mut s1 = String::new();
    let mut s2 = String::new();

    // Read the two strings
    handle.read_line(&mut s1).unwrap();
    handle.read_line(&mut s2).unwrap();

    // Remove the newline character from the strings
    let s1 = s1.trim();
    let s2 = s2.trim();

    let mut ans: i64 = 1;
    let mut t: i32;

    if s1.chars().nth(0) == s2.chars().nth(0) {
        ans = 3;
        t = 1;
    } else {
        ans = 6;
        t = 0;
    }

    let m = 1_000_000_007;

    for i in 1..n {
        if s1.chars().nth(i) == s2.chars().nth(i) {
            if t == 1 {
                ans = (ans * 2) % m;
            }
            t = 1;
        }
        if i < n - 1 && s1.chars().nth(i) == s1.chars().nth(i + 1) {
            if t == 1 {
                ans = (ans * 2) % m;
            } else {
                ans = (ans * 3) % m;
            }
            t = 0;
        }
    }

    // Print the result
    println!("{}", ans);
}

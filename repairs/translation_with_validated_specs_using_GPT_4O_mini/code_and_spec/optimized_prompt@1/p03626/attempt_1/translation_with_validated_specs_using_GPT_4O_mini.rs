use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the length of the strings
    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a positive integer");

    // Read the two strings
    let mut s1 = String::new();
    let mut s2 = String::new();
    reader.read_line(&mut s1).expect("Failed to read line");
    reader.read_line(&mut s2).expect("Failed to read line");

    // Ensure the strings are of the correct length
    assert_eq!(s1.len(), n);
    assert_eq!(s2.len(), n);

    let mut ans: i64 = 1;
    let mut t: i32;

    if s1.chars().nth(0) == s2.chars().nth(0) {
        ans = 3;
        t = 1;
    } else {
        ans = 6;
        t = 0;
    }

    let m = 1000000007;

    for i in 1..n {
        if s1.chars().nth(i) == s2.chars().nth(i) {
            if t == 1 {
                ans = (ans * 2) % m;
            }
            t = 1;
        }
        if s1.chars().nth(i) == s1.chars().nth(i + 1) {
            if t == 1 {
                ans = (ans * 2) % m;
            } else {
                ans = (ans * 3) % m;
            }
            t = 0;
        }
    }

    println!("{}", ans);
}

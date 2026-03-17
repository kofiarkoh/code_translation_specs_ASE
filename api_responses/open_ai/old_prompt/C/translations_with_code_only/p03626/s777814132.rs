use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let s1: Vec<char> = lines.next().unwrap().chars().collect();
    let s2: Vec<char> = lines.next().unwrap().chars().collect();

    let mut ans: i64 = 1;
    let mut t: i32;

    if s1[0] == s2[0] {
        ans = 3;
        t = 1;
    } else {
        ans = 6;
        t = 0;
    }

    let m: i64 = 1_000_000_007;

    for i in 1..n {
        if s1[i] == s2[i] {
            if t == 1 {
                ans = (ans * 2) % m;
            }
            t = 1;
        }
        if i < n - 1 && s1[i] == s1[i + 1] {
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

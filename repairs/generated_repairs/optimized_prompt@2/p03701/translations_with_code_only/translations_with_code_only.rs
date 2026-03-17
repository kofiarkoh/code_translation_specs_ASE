use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Invalid input for n");

    let mut s = vec![0; n];
    let mut k = 0;
    let mut min = None;

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        s[i] = input.trim().parse::<usize>().expect("Invalid input for s[i]");
        k += s[i];

        if s[i] % 10 == 0 {
            s[i] = 101; // Emulating the C behavior of setting to 101
        }

        if min.is_none() || min.unwrap() > s[i] {
            min = Some(s[i]);
        }
    }

    let mut ans = k;
    if k % 10 == 0 {
        if min == Some(101) {
            ans = 0;
        } else {
            ans = k - min.unwrap();
        }
    }

    // Print the result with a newline
    println!("{}", ans);
}

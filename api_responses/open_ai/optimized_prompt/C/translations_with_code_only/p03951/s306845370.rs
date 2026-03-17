use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut s = String::new();
    let mut t = String::new();

    io::stdin().read_line(&mut s).unwrap();
    io::stdin().read_line(&mut t).unwrap();

    // Ensure strings are trimmed to remove any trailing newline characters
    let s = s.trim();
    let t = t.trim();

    let mut ans = 0;
    let mut i = n as isize - 1; // Use isize for negative indexing
    let mut j;

    while i >= 0 {
        j = i; // Reset j to i for each outer loop iteration
        while j >= 0 && s.chars().nth(i as usize) != t.chars().nth(j as usize) {
            j -= 1;
        }
        if j >= 0 {
            while j >= 0 && s.chars().nth(i as usize) == t.chars().nth(j as usize) {
                i -= 1;
                j -= 1;
            }
            if j < 0 {
                ans = 2 * n - (n - i - 1) as isize;
                break;
            }
        } else {
            ans = 2 * n - (n - i - 1) as isize;
            break;
        }
        i -= 1; // Decrement i for the next iteration of the outer loop
    }

    // Print the result
    print!("{}", ans);
}

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut s = String::new();
    let mut t = String::new();

    io::stdin().read_line(&mut s).unwrap();
    io::stdin().read_line(&mut t).unwrap();

    let mut ans = 0;
    let mut i = n as isize - 1;

    while i >= 0 {
        let mut j = n as isize - 1; // Reset j to the last index of T for each outer loop iteration
        while j >= 0 && s.as_bytes()[i as usize] != t.as_bytes()[j as usize] {
            j -= 1;
        }
        if j >= 0 {
            let mut k = i; // Use a new variable k to track the matching characters
            while j >= 0 && k >= 0 && s.as_bytes()[k as usize] == t.as_bytes()[j as usize] {
                k -= 1;
                j -= 1;
            }
            if j < 0 {
                ans = 2 * n as isize - (n as isize - k - 1);
                break;
            }
        } else {
            ans = 2 * n as isize - (n as isize - i - 1);
            break;
        }
        i -= 1; // Decrement i for the next iteration of the outer loop
    }

    // Adjust the answer to account for the case when no matches were found
    if ans == 0 {
        ans = 2 * n as isize; // If no matches, the answer should be 2*N
    }

    print!("{}", ans);
}
